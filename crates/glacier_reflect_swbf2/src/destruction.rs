use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_destruction_types(registry: &mut TypeRegistry) {
    registry.register_type(CONNECTIVITYENTITYDATA_TYPE_INFO);
    registry.register_type(CONNECTIVITYENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BREAKABLEPARTTOSTATICENTITYPART_TYPE_INFO);
    registry.register_type(BREAKABLEPARTTOSTATICENTITYPART_ARRAY_TYPE_INFO);
    registry.register_type(STATICMODELTOBREAKABLEPARTS_TYPE_INFO);
    registry.register_type(STATICMODELTOBREAKABLEPARTS_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMECOMPONENTDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONVOLUMECOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONCOMPONENTONHEALTHTRANSITIONTRIGGEREDMESSAGE_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONDATA_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONDATA_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONPARTDATA_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONPARTDATA_ARRAY_TYPE_INFO);
    registry.register_type(PARTRADIOSITYMATERIALDATA_TYPE_INFO);
    registry.register_type(PARTRADIOSITYMATERIALDATA_ARRAY_TYPE_INFO);
    registry.register_type(CALCULATECONNECTEDPARTSPIPELINERESULT_TYPE_INFO);
    registry.register_type(CALCULATECONNECTEDPARTSPIPELINERESULT_ARRAY_TYPE_INFO);
    registry.register_type(CALCULATECONNECTEDPARTSPIPELINEPARAMS_TYPE_INFO);
    registry.register_type(CALCULATECONNECTEDPARTSPIPELINEPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(TOUCHINGPARTPAIR_TYPE_INFO);
    registry.register_type(TOUCHINGPARTPAIR_ARRAY_TYPE_INFO);
    registry.register_type(CONNECTIONCONSTRAINT_TYPE_INFO);
    registry.register_type(CONNECTIONCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(SELFSUPPORTCONSTRAINT_TYPE_INFO);
    registry.register_type(SELFSUPPORTCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(SUPPORTCONSTRAINT_TYPE_INFO);
    registry.register_type(SUPPORTCONSTRAINT_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DESTRUCTIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELINFO_TYPE_INFO);
    registry.register_type(EDGEMODELINFO_ARRAY_TYPE_INFO);
    registry.register_type(SERVERHEALTHTRANSITIONPART_TYPE_INFO);
    registry.register_type(SERVERHEALTHTRANSITIONPART_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONPART_TYPE_INFO);
    registry.register_type(HEALTHTRANSITIONPART_ARRAY_TYPE_INFO);
    registry.register_type(HEALTHTRANSITION_TYPE_INFO);
    registry.register_type(HEALTHTRANSITION_ARRAY_TYPE_INFO);
    registry.register_type(EDGEMODELOWNER_TYPE_INFO);
    registry.register_type(EDGEMODELOWNER_ARRAY_TYPE_INFO);
    registry.register_type(DESTRUCTIONCOMPONENT_TYPE_INFO);
    registry.register_type(DESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTHEALTHTRANSITIONPART_TYPE_INFO);
    registry.register_type(CLIENTHEALTHTRANSITIONPART_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ConnectivityEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub breakable_model_event_ids: Vec<i32>,
    pub static_model_event_ids: Vec<i32>,
    pub static_model_first_indices: Vec<u32>,
    pub static_model_to_breakable_parts_array: Vec<BoxedTypeObject /* StaticModelToBreakableParts */>,
    pub breakable_part_to_static_model_part_array: Vec<BoxedTypeObject /* BreakablePartToStaticEntityPart */>,
    pub breakable_model_extra_radius: f32,
}

pub trait ConnectivityEntityDataTrait: super::entity::EntityDataTrait {
    fn breakable_model_event_ids(&self) -> &Vec<i32>;
    fn breakable_model_event_ids_mut(&mut self) -> &mut Vec<i32>;
    fn static_model_event_ids(&self) -> &Vec<i32>;
    fn static_model_event_ids_mut(&mut self) -> &mut Vec<i32>;
    fn static_model_first_indices(&self) -> &Vec<u32>;
    fn static_model_first_indices_mut(&mut self) -> &mut Vec<u32>;
    fn static_model_to_breakable_parts_array(&self) -> &Vec<BoxedTypeObject /* StaticModelToBreakableParts */>;
    fn static_model_to_breakable_parts_array_mut(&mut self) -> &mut Vec<BoxedTypeObject /* StaticModelToBreakableParts */>;
    fn breakable_part_to_static_model_part_array(&self) -> &Vec<BoxedTypeObject /* BreakablePartToStaticEntityPart */>;
    fn breakable_part_to_static_model_part_array_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BreakablePartToStaticEntityPart */>;
    fn breakable_model_extra_radius(&self) -> &f32;
    fn breakable_model_extra_radius_mut(&mut self) -> &mut f32;
}

impl ConnectivityEntityDataTrait for ConnectivityEntityData {
    fn breakable_model_event_ids(&self) -> &Vec<i32> {
        &self.breakable_model_event_ids
    }
    fn breakable_model_event_ids_mut(&mut self) -> &mut Vec<i32> {
        &mut self.breakable_model_event_ids
    }
    fn static_model_event_ids(&self) -> &Vec<i32> {
        &self.static_model_event_ids
    }
    fn static_model_event_ids_mut(&mut self) -> &mut Vec<i32> {
        &mut self.static_model_event_ids
    }
    fn static_model_first_indices(&self) -> &Vec<u32> {
        &self.static_model_first_indices
    }
    fn static_model_first_indices_mut(&mut self) -> &mut Vec<u32> {
        &mut self.static_model_first_indices
    }
    fn static_model_to_breakable_parts_array(&self) -> &Vec<BoxedTypeObject /* StaticModelToBreakableParts */> {
        &self.static_model_to_breakable_parts_array
    }
    fn static_model_to_breakable_parts_array_mut(&mut self) -> &mut Vec<BoxedTypeObject /* StaticModelToBreakableParts */> {
        &mut self.static_model_to_breakable_parts_array
    }
    fn breakable_part_to_static_model_part_array(&self) -> &Vec<BoxedTypeObject /* BreakablePartToStaticEntityPart */> {
        &self.breakable_part_to_static_model_part_array
    }
    fn breakable_part_to_static_model_part_array_mut(&mut self) -> &mut Vec<BoxedTypeObject /* BreakablePartToStaticEntityPart */> {
        &mut self.breakable_part_to_static_model_part_array
    }
    fn breakable_model_extra_radius(&self) -> &f32 {
        &self.breakable_model_extra_radius
    }
    fn breakable_model_extra_radius_mut(&mut self) -> &mut f32 {
        &mut self.breakable_model_extra_radius
    }
}

impl super::entity::EntityDataTrait for ConnectivityEntityData {
}

impl super::entity::GameObjectDataTrait for ConnectivityEntityData {
}

impl super::core::DataBusPeerTrait for ConnectivityEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConnectivityEntityData {
}

impl super::core::DataContainerTrait for ConnectivityEntityData {
}

pub static CONNECTIVITYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectivityEntityData",
    name_hash: 4164659659,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ConnectivityEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConnectivityEntityData as Default>::default())),
            create_boxed: || Box::new(<ConnectivityEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BreakableModelEventIds",
                name_hash: 1491985709,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(ConnectivityEntityData, breakable_model_event_ids),
            },
            FieldInfoData {
                name: "StaticModelEventIds",
                name_hash: 3960339168,
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(ConnectivityEntityData, static_model_event_ids),
            },
            FieldInfoData {
                name: "StaticModelFirstIndices",
                name_hash: 1366001143,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(ConnectivityEntityData, static_model_first_indices),
            },
            FieldInfoData {
                name: "StaticModelToBreakablePartsArray",
                name_hash: 3769834337,
                flags: MemberInfoFlags::new(144),
                field_type: "StaticModelToBreakableParts-Array",
                rust_offset: offset_of!(ConnectivityEntityData, static_model_to_breakable_parts_array),
            },
            FieldInfoData {
                name: "BreakablePartToStaticModelPartArray",
                name_hash: 2726585189,
                flags: MemberInfoFlags::new(144),
                field_type: "BreakablePartToStaticEntityPart-Array",
                rust_offset: offset_of!(ConnectivityEntityData, breakable_part_to_static_model_part_array),
            },
            FieldInfoData {
                name: "BreakableModelExtraRadius",
                name_hash: 2110173853,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConnectivityEntityData, breakable_model_extra_radius),
            },
        ],
    }),
    array_type: Some(CONNECTIVITYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConnectivityEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONNECTIVITYENTITYDATA_TYPE_INFO
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


pub static CONNECTIVITYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectivityEntityData-Array",
    name_hash: 3658811647,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ConnectivityEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BreakablePartToStaticEntityPart {
    pub breakable_part_index: u32,
    pub static_model_part_index: u32,
}

pub trait BreakablePartToStaticEntityPartTrait: TypeObject {
    fn breakable_part_index(&self) -> &u32;
    fn breakable_part_index_mut(&mut self) -> &mut u32;
    fn static_model_part_index(&self) -> &u32;
    fn static_model_part_index_mut(&mut self) -> &mut u32;
}

impl BreakablePartToStaticEntityPartTrait for BreakablePartToStaticEntityPart {
    fn breakable_part_index(&self) -> &u32 {
        &self.breakable_part_index
    }
    fn breakable_part_index_mut(&mut self) -> &mut u32 {
        &mut self.breakable_part_index
    }
    fn static_model_part_index(&self) -> &u32 {
        &self.static_model_part_index
    }
    fn static_model_part_index_mut(&mut self) -> &mut u32 {
        &mut self.static_model_part_index
    }
}

pub static BREAKABLEPARTTOSTATICENTITYPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BreakablePartToStaticEntityPart",
    name_hash: 3819974536,
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BreakablePartToStaticEntityPart as Default>::default())),
            create_boxed: || Box::new(<BreakablePartToStaticEntityPart as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BreakablePartIndex",
                name_hash: 3855974617,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BreakablePartToStaticEntityPart, breakable_part_index),
            },
            FieldInfoData {
                name: "StaticModelPartIndex",
                name_hash: 1596876187,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BreakablePartToStaticEntityPart, static_model_part_index),
            },
        ],
    }),
    array_type: Some(BREAKABLEPARTTOSTATICENTITYPART_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BreakablePartToStaticEntityPart {
    fn type_info(&self) -> &'static TypeInfo {
        BREAKABLEPARTTOSTATICENTITYPART_TYPE_INFO
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


pub static BREAKABLEPARTTOSTATICENTITYPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BreakablePartToStaticEntityPart-Array",
    name_hash: 4245934524,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("BreakablePartToStaticEntityPart"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StaticModelToBreakableParts {
    pub static_model_index: u32,
    pub breakable_part_start_index: u32,
    pub breakable_part_count: u32,
}

pub trait StaticModelToBreakablePartsTrait: TypeObject {
    fn static_model_index(&self) -> &u32;
    fn static_model_index_mut(&mut self) -> &mut u32;
    fn breakable_part_start_index(&self) -> &u32;
    fn breakable_part_start_index_mut(&mut self) -> &mut u32;
    fn breakable_part_count(&self) -> &u32;
    fn breakable_part_count_mut(&mut self) -> &mut u32;
}

impl StaticModelToBreakablePartsTrait for StaticModelToBreakableParts {
    fn static_model_index(&self) -> &u32 {
        &self.static_model_index
    }
    fn static_model_index_mut(&mut self) -> &mut u32 {
        &mut self.static_model_index
    }
    fn breakable_part_start_index(&self) -> &u32 {
        &self.breakable_part_start_index
    }
    fn breakable_part_start_index_mut(&mut self) -> &mut u32 {
        &mut self.breakable_part_start_index
    }
    fn breakable_part_count(&self) -> &u32 {
        &self.breakable_part_count
    }
    fn breakable_part_count_mut(&mut self) -> &mut u32 {
        &mut self.breakable_part_count
    }
}

pub static STATICMODELTOBREAKABLEPARTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticModelToBreakableParts",
    name_hash: 1417881048,
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StaticModelToBreakableParts as Default>::default())),
            create_boxed: || Box::new(<StaticModelToBreakableParts as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "StaticModelIndex",
                name_hash: 655693516,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(StaticModelToBreakableParts, static_model_index),
            },
            FieldInfoData {
                name: "BreakablePartStartIndex",
                name_hash: 3818440377,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(StaticModelToBreakableParts, breakable_part_start_index),
            },
            FieldInfoData {
                name: "BreakablePartCount",
                name_hash: 3849422468,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(StaticModelToBreakableParts, breakable_part_count),
            },
        ],
    }),
    array_type: Some(STATICMODELTOBREAKABLEPARTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for StaticModelToBreakableParts {
    fn type_info(&self) -> &'static TypeInfo {
        STATICMODELTOBREAKABLEPARTS_TYPE_INFO
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


pub static STATICMODELTOBREAKABLEPARTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticModelToBreakableParts-Array",
    name_hash: 3501015148,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("StaticModelToBreakableParts"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DestructionVolumeComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub destruction_volume_data: Option<LockedTypeObject /* super::world_base::DestructionVolumeBaseData */>,
}

pub trait DestructionVolumeComponentDataTrait: super::entity::GameComponentDataTrait {
    fn destruction_volume_data(&self) -> &Option<LockedTypeObject /* super::world_base::DestructionVolumeBaseData */>;
    fn destruction_volume_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::world_base::DestructionVolumeBaseData */>;
}

impl DestructionVolumeComponentDataTrait for DestructionVolumeComponentData {
    fn destruction_volume_data(&self) -> &Option<LockedTypeObject /* super::world_base::DestructionVolumeBaseData */> {
        &self.destruction_volume_data
    }
    fn destruction_volume_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::world_base::DestructionVolumeBaseData */> {
        &mut self.destruction_volume_data
    }
}

impl super::entity::GameComponentDataTrait for DestructionVolumeComponentData {
}

impl super::entity::ComponentDataTrait for DestructionVolumeComponentData {
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

impl super::entity::GameObjectDataTrait for DestructionVolumeComponentData {
}

impl super::core::DataBusPeerTrait for DestructionVolumeComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DestructionVolumeComponentData {
}

impl super::core::DataContainerTrait for DestructionVolumeComponentData {
}

pub static DESTRUCTIONVOLUMECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponentData",
    name_hash: 315036908,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(DestructionVolumeComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionVolumeComponentData as Default>::default())),
            create_boxed: || Box::new(<DestructionVolumeComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "DestructionVolumeData",
                name_hash: 3546689379,
                flags: MemberInfoFlags::new(0),
                field_type: "DestructionVolumeBaseData",
                rust_offset: offset_of!(DestructionVolumeComponentData, destruction_volume_data),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionVolumeComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONVOLUMECOMPONENTDATA_TYPE_INFO
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


pub static DESTRUCTIONVOLUMECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponentData-Array",
    name_hash: 3870078168,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("DestructionVolumeComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DestructionComponentOnHealthTransitionTriggeredMessage {
}

pub trait DestructionComponentOnHealthTransitionTriggeredMessageTrait: TypeObject {
}

impl DestructionComponentOnHealthTransitionTriggeredMessageTrait for DestructionComponentOnHealthTransitionTriggeredMessage {
}

pub static DESTRUCTIONCOMPONENTONHEALTHTRANSITIONTRIGGEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponentOnHealthTransitionTriggeredMessage",
    name_hash: 1740937588,
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionComponentOnHealthTransitionTriggeredMessage as Default>::default())),
            create_boxed: || Box::new(<DestructionComponentOnHealthTransitionTriggeredMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for DestructionComponentOnHealthTransitionTriggeredMessage {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONCOMPONENTONHEALTHTRANSITIONTRIGGEREDMESSAGE_TYPE_INFO
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthTransitionSpawnReferenceObjectData {
    pub _glacier_base: super::entity::SpatialReferenceObjectData,
    pub spawn_linear_velocity: super::core::Vec3,
    pub spawn_angular_velocity: super::core::Vec3,
}

pub trait HealthTransitionSpawnReferenceObjectDataTrait: super::entity::SpatialReferenceObjectDataTrait {
    fn spawn_linear_velocity(&self) -> &super::core::Vec3;
    fn spawn_linear_velocity_mut(&mut self) -> &mut super::core::Vec3;
    fn spawn_angular_velocity(&self) -> &super::core::Vec3;
    fn spawn_angular_velocity_mut(&mut self) -> &mut super::core::Vec3;
}

impl HealthTransitionSpawnReferenceObjectDataTrait for HealthTransitionSpawnReferenceObjectData {
    fn spawn_linear_velocity(&self) -> &super::core::Vec3 {
        &self.spawn_linear_velocity
    }
    fn spawn_linear_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.spawn_linear_velocity
    }
    fn spawn_angular_velocity(&self) -> &super::core::Vec3 {
        &self.spawn_angular_velocity
    }
    fn spawn_angular_velocity_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.spawn_angular_velocity
    }
}

impl super::entity::SpatialReferenceObjectDataTrait for HealthTransitionSpawnReferenceObjectData {
    fn local_player_id(&self) -> &super::core::LocalPlayerId {
        self._glacier_base.local_player_id()
    }
    fn local_player_id_mut(&mut self) -> &mut super::core::LocalPlayerId {
        self._glacier_base.local_player_id_mut()
    }
}

impl super::entity::ReferenceObjectDataTrait for HealthTransitionSpawnReferenceObjectData {
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

impl super::entity::GameObjectDataTrait for HealthTransitionSpawnReferenceObjectData {
}

impl super::core::DataBusPeerTrait for HealthTransitionSpawnReferenceObjectData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for HealthTransitionSpawnReferenceObjectData {
}

impl super::core::DataContainerTrait for HealthTransitionSpawnReferenceObjectData {
}

pub static HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionSpawnReferenceObjectData",
    name_hash: 3922819267,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        super_class_offset: offset_of!(HealthTransitionSpawnReferenceObjectData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthTransitionSpawnReferenceObjectData as Default>::default())),
            create_boxed: || Box::new(<HealthTransitionSpawnReferenceObjectData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SpawnLinearVelocity",
                name_hash: 1090557460,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HealthTransitionSpawnReferenceObjectData, spawn_linear_velocity),
            },
            FieldInfoData {
                name: "SpawnAngularVelocity",
                name_hash: 194970123,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(HealthTransitionSpawnReferenceObjectData, spawn_angular_velocity),
            },
        ],
    }),
    array_type: Some(HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HealthTransitionSpawnReferenceObjectData {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_TYPE_INFO
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


pub static HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionSpawnReferenceObjectData-Array",
    name_hash: 4228561911,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionSpawnReferenceObjectData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthTransitionData {
    pub _glacier_base: super::entity::EntityData,
    pub part_state: Option<LockedTypeObject /* super::entity::PartState */>,
    pub health_transition_value: u32,
    pub objects: Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>,
    pub in_event_id: u32,
    pub out_event_id: u32,
}

pub trait HealthTransitionDataTrait: super::entity::EntityDataTrait {
    fn part_state(&self) -> &Option<LockedTypeObject /* super::entity::PartState */>;
    fn part_state_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartState */>;
    fn health_transition_value(&self) -> &u32;
    fn health_transition_value_mut(&mut self) -> &mut u32;
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>>;
    fn in_event_id(&self) -> &u32;
    fn in_event_id_mut(&mut self) -> &mut u32;
    fn out_event_id(&self) -> &u32;
    fn out_event_id_mut(&mut self) -> &mut u32;
}

impl HealthTransitionDataTrait for HealthTransitionData {
    fn part_state(&self) -> &Option<LockedTypeObject /* super::entity::PartState */> {
        &self.part_state
    }
    fn part_state_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartState */> {
        &mut self.part_state
    }
    fn health_transition_value(&self) -> &u32 {
        &self.health_transition_value
    }
    fn health_transition_value_mut(&mut self) -> &mut u32 {
        &mut self.health_transition_value
    }
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &self.objects
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        &mut self.objects
    }
    fn in_event_id(&self) -> &u32 {
        &self.in_event_id
    }
    fn in_event_id_mut(&mut self) -> &mut u32 {
        &mut self.in_event_id
    }
    fn out_event_id(&self) -> &u32 {
        &self.out_event_id
    }
    fn out_event_id_mut(&mut self) -> &mut u32 {
        &mut self.out_event_id
    }
}

impl super::entity::EntityDataTrait for HealthTransitionData {
}

impl super::entity::GameObjectDataTrait for HealthTransitionData {
}

impl super::core::DataBusPeerTrait for HealthTransitionData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for HealthTransitionData {
}

impl super::core::DataContainerTrait for HealthTransitionData {
}

pub static HEALTHTRANSITIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionData",
    name_hash: 1692543974,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(HealthTransitionData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthTransitionData as Default>::default())),
            create_boxed: || Box::new(<HealthTransitionData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "PartState",
                name_hash: 3179262629,
                flags: MemberInfoFlags::new(0),
                field_type: "PartState",
                rust_offset: offset_of!(HealthTransitionData, part_state),
            },
            FieldInfoData {
                name: "HealthTransitionValue",
                name_hash: 4288419421,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HealthTransitionData, health_transition_value),
            },
            FieldInfoData {
                name: "Objects",
                name_hash: 105488131,
                flags: MemberInfoFlags::new(144),
                field_type: "GameObjectData-Array",
                rust_offset: offset_of!(HealthTransitionData, objects),
            },
            FieldInfoData {
                name: "InEventId",
                name_hash: 3917437187,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HealthTransitionData, in_event_id),
            },
            FieldInfoData {
                name: "OutEventId",
                name_hash: 47754794,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HealthTransitionData, out_event_id),
            },
        ],
    }),
    array_type: Some(HEALTHTRANSITIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthTransitionData {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHTRANSITIONDATA_TYPE_INFO
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


pub static HEALTHTRANSITIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionData-Array",
    name_hash: 4087998674,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthTransitionPartData {
    pub _glacier_base: super::entity::EntityData,
    pub part: Option<LockedTypeObject /* super::entity::PartData */>,
    pub health_transitions: Vec<Option<LockedTypeObject /* HealthTransitionData */>>,
    pub health: u32,
    pub radiosity_material_data: Vec<BoxedTypeObject /* PartRadiosityMaterialData */>,
}

pub trait HealthTransitionPartDataTrait: super::entity::EntityDataTrait {
    fn part(&self) -> &Option<LockedTypeObject /* super::entity::PartData */>;
    fn part_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */>;
    fn health_transitions(&self) -> &Vec<Option<LockedTypeObject /* HealthTransitionData */>>;
    fn health_transitions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HealthTransitionData */>>;
    fn health(&self) -> &u32;
    fn health_mut(&mut self) -> &mut u32;
    fn radiosity_material_data(&self) -> &Vec<BoxedTypeObject /* PartRadiosityMaterialData */>;
    fn radiosity_material_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* PartRadiosityMaterialData */>;
}

impl HealthTransitionPartDataTrait for HealthTransitionPartData {
    fn part(&self) -> &Option<LockedTypeObject /* super::entity::PartData */> {
        &self.part
    }
    fn part_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */> {
        &mut self.part
    }
    fn health_transitions(&self) -> &Vec<Option<LockedTypeObject /* HealthTransitionData */>> {
        &self.health_transitions
    }
    fn health_transitions_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HealthTransitionData */>> {
        &mut self.health_transitions
    }
    fn health(&self) -> &u32 {
        &self.health
    }
    fn health_mut(&mut self) -> &mut u32 {
        &mut self.health
    }
    fn radiosity_material_data(&self) -> &Vec<BoxedTypeObject /* PartRadiosityMaterialData */> {
        &self.radiosity_material_data
    }
    fn radiosity_material_data_mut(&mut self) -> &mut Vec<BoxedTypeObject /* PartRadiosityMaterialData */> {
        &mut self.radiosity_material_data
    }
}

impl super::entity::EntityDataTrait for HealthTransitionPartData {
}

impl super::entity::GameObjectDataTrait for HealthTransitionPartData {
}

impl super::core::DataBusPeerTrait for HealthTransitionPartData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for HealthTransitionPartData {
}

impl super::core::DataContainerTrait for HealthTransitionPartData {
}

pub static HEALTHTRANSITIONPARTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPartData",
    name_hash: 1062034769,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(HealthTransitionPartData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthTransitionPartData as Default>::default())),
            create_boxed: || Box::new(<HealthTransitionPartData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Part",
                name_hash: 2089448370,
                flags: MemberInfoFlags::new(0),
                field_type: "PartData",
                rust_offset: offset_of!(HealthTransitionPartData, part),
            },
            FieldInfoData {
                name: "HealthTransitions",
                name_hash: 211586533,
                flags: MemberInfoFlags::new(144),
                field_type: "HealthTransitionData-Array",
                rust_offset: offset_of!(HealthTransitionPartData, health_transitions),
            },
            FieldInfoData {
                name: "Health",
                name_hash: 3054337113,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(HealthTransitionPartData, health),
            },
            FieldInfoData {
                name: "RadiosityMaterialData",
                name_hash: 1546463064,
                flags: MemberInfoFlags::new(144),
                field_type: "PartRadiosityMaterialData-Array",
                rust_offset: offset_of!(HealthTransitionPartData, radiosity_material_data),
            },
        ],
    }),
    array_type: Some(HEALTHTRANSITIONPARTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthTransitionPartData {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHTRANSITIONPARTDATA_TYPE_INFO
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


pub static HEALTHTRANSITIONPARTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPartData-Array",
    name_hash: 3538325349,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionPartData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PartRadiosityMaterialData {
    pub material_guid: glacier_util::guid::Guid,
    pub default_opacity: f32,
}

pub trait PartRadiosityMaterialDataTrait: TypeObject {
    fn material_guid(&self) -> &glacier_util::guid::Guid;
    fn material_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn default_opacity(&self) -> &f32;
    fn default_opacity_mut(&mut self) -> &mut f32;
}

impl PartRadiosityMaterialDataTrait for PartRadiosityMaterialData {
    fn material_guid(&self) -> &glacier_util::guid::Guid {
        &self.material_guid
    }
    fn material_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.material_guid
    }
    fn default_opacity(&self) -> &f32 {
        &self.default_opacity
    }
    fn default_opacity_mut(&mut self) -> &mut f32 {
        &mut self.default_opacity
    }
}

pub static PARTRADIOSITYMATERIALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartRadiosityMaterialData",
    name_hash: 2222301007,
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PartRadiosityMaterialData as Default>::default())),
            create_boxed: || Box::new(<PartRadiosityMaterialData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                name_hash: 160722513,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(PartRadiosityMaterialData, material_guid),
            },
            FieldInfoData {
                name: "DefaultOpacity",
                name_hash: 3611811607,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PartRadiosityMaterialData, default_opacity),
            },
        ],
    }),
    array_type: Some(PARTRADIOSITYMATERIALDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PartRadiosityMaterialData {
    fn type_info(&self) -> &'static TypeInfo {
        PARTRADIOSITYMATERIALDATA_TYPE_INFO
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


pub static PARTRADIOSITYMATERIALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartRadiosityMaterialData-Array",
    name_hash: 126930043,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("PartRadiosityMaterialData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CalculateConnectedPartsPipelineResult {
    pub _glacier_base: super::core::DataContainer,
    pub success: bool,
    pub touching_part_pairs: Vec<BoxedTypeObject /* TouchingPartPair */>,
}

pub trait CalculateConnectedPartsPipelineResultTrait: super::core::DataContainerTrait {
    fn success(&self) -> &bool;
    fn success_mut(&mut self) -> &mut bool;
    fn touching_part_pairs(&self) -> &Vec<BoxedTypeObject /* TouchingPartPair */>;
    fn touching_part_pairs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TouchingPartPair */>;
}

impl CalculateConnectedPartsPipelineResultTrait for CalculateConnectedPartsPipelineResult {
    fn success(&self) -> &bool {
        &self.success
    }
    fn success_mut(&mut self) -> &mut bool {
        &mut self.success
    }
    fn touching_part_pairs(&self) -> &Vec<BoxedTypeObject /* TouchingPartPair */> {
        &self.touching_part_pairs
    }
    fn touching_part_pairs_mut(&mut self) -> &mut Vec<BoxedTypeObject /* TouchingPartPair */> {
        &mut self.touching_part_pairs
    }
}

impl super::core::DataContainerTrait for CalculateConnectedPartsPipelineResult {
}

pub static CALCULATECONNECTEDPARTSPIPELINERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineResult",
    name_hash: 137629809,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CalculateConnectedPartsPipelineResult, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CalculateConnectedPartsPipelineResult as Default>::default())),
            create_boxed: || Box::new(<CalculateConnectedPartsPipelineResult as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Success",
                name_hash: 2134798598,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CalculateConnectedPartsPipelineResult, success),
            },
            FieldInfoData {
                name: "TouchingPartPairs",
                name_hash: 75644654,
                flags: MemberInfoFlags::new(144),
                field_type: "TouchingPartPair-Array",
                rust_offset: offset_of!(CalculateConnectedPartsPipelineResult, touching_part_pairs),
            },
        ],
    }),
    array_type: Some(CALCULATECONNECTEDPARTSPIPELINERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CalculateConnectedPartsPipelineResult {
    fn type_info(&self) -> &'static TypeInfo {
        CALCULATECONNECTEDPARTSPIPELINERESULT_TYPE_INFO
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


pub static CALCULATECONNECTEDPARTSPIPELINERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineResult-Array",
    name_hash: 1720161093,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("CalculateConnectedPartsPipelineResult"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CalculateConnectedPartsPipelineParams {
    pub _glacier_base: super::core::DataContainer,
}

pub trait CalculateConnectedPartsPipelineParamsTrait: super::core::DataContainerTrait {
}

impl CalculateConnectedPartsPipelineParamsTrait for CalculateConnectedPartsPipelineParams {
}

impl super::core::DataContainerTrait for CalculateConnectedPartsPipelineParams {
}

pub static CALCULATECONNECTEDPARTSPIPELINEPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineParams",
    name_hash: 206399204,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CalculateConnectedPartsPipelineParams, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CalculateConnectedPartsPipelineParams as Default>::default())),
            create_boxed: || Box::new(<CalculateConnectedPartsPipelineParams as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CALCULATECONNECTEDPARTSPIPELINEPARAMS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CalculateConnectedPartsPipelineParams {
    fn type_info(&self) -> &'static TypeInfo {
        CALCULATECONNECTEDPARTSPIPELINEPARAMS_TYPE_INFO
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


pub static CALCULATECONNECTEDPARTSPIPELINEPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineParams-Array",
    name_hash: 1370041040,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("CalculateConnectedPartsPipelineParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TouchingPartPair {
    pub guid0: glacier_util::guid::Guid,
    pub guid1: glacier_util::guid::Guid,
}

pub trait TouchingPartPairTrait: TypeObject {
    fn guid0(&self) -> &glacier_util::guid::Guid;
    fn guid0_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn guid1(&self) -> &glacier_util::guid::Guid;
    fn guid1_mut(&mut self) -> &mut glacier_util::guid::Guid;
}

impl TouchingPartPairTrait for TouchingPartPair {
    fn guid0(&self) -> &glacier_util::guid::Guid {
        &self.guid0
    }
    fn guid0_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.guid0
    }
    fn guid1(&self) -> &glacier_util::guid::Guid {
        &self.guid1
    }
    fn guid1_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.guid1
    }
}

pub static TOUCHINGPARTPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TouchingPartPair",
    name_hash: 1303797501,
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TouchingPartPair as Default>::default())),
            create_boxed: || Box::new(<TouchingPartPair as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Guid0",
                name_hash: 208443530,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(TouchingPartPair, guid0),
            },
            FieldInfoData {
                name: "Guid1",
                name_hash: 208443531,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(TouchingPartPair, guid1),
            },
        ],
    }),
    array_type: Some(TOUCHINGPARTPAIR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TouchingPartPair {
    fn type_info(&self) -> &'static TypeInfo {
        TOUCHINGPARTPAIR_TYPE_INFO
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


pub static TOUCHINGPARTPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TouchingPartPair-Array",
    name_hash: 1693043657,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("TouchingPartPair"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ConnectionConstraint {
    pub _glacier_base: SupportConstraint,
    pub connected_part1: Option<LockedTypeObject /* super::entity::PartData */>,
    pub connected_part2: Option<LockedTypeObject /* super::entity::PartData */>,
}

pub trait ConnectionConstraintTrait: SupportConstraintTrait {
    fn connected_part1(&self) -> &Option<LockedTypeObject /* super::entity::PartData */>;
    fn connected_part1_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */>;
    fn connected_part2(&self) -> &Option<LockedTypeObject /* super::entity::PartData */>;
    fn connected_part2_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */>;
}

impl ConnectionConstraintTrait for ConnectionConstraint {
    fn connected_part1(&self) -> &Option<LockedTypeObject /* super::entity::PartData */> {
        &self.connected_part1
    }
    fn connected_part1_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */> {
        &mut self.connected_part1
    }
    fn connected_part2(&self) -> &Option<LockedTypeObject /* super::entity::PartData */> {
        &self.connected_part2
    }
    fn connected_part2_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */> {
        &mut self.connected_part2
    }
}

impl SupportConstraintTrait for ConnectionConstraint {
}

impl super::core::DataContainerTrait for ConnectionConstraint {
}

pub static CONNECTIONCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectionConstraint",
    name_hash: 106169974,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUPPORTCONSTRAINT_TYPE_INFO),
        super_class_offset: offset_of!(ConnectionConstraint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConnectionConstraint as Default>::default())),
            create_boxed: || Box::new(<ConnectionConstraint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ConnectedPart1",
                name_hash: 2622089916,
                flags: MemberInfoFlags::new(0),
                field_type: "PartData",
                rust_offset: offset_of!(ConnectionConstraint, connected_part1),
            },
            FieldInfoData {
                name: "ConnectedPart2",
                name_hash: 2622089919,
                flags: MemberInfoFlags::new(0),
                field_type: "PartData",
                rust_offset: offset_of!(ConnectionConstraint, connected_part2),
            },
        ],
    }),
    array_type: Some(CONNECTIONCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConnectionConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        CONNECTIONCONSTRAINT_TYPE_INFO
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


pub static CONNECTIONCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectionConstraint-Array",
    name_hash: 2320001858,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ConnectionConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SelfSupportConstraint {
    pub _glacier_base: SupportConstraint,
    pub self_supporting_part: Option<LockedTypeObject /* super::entity::PartData */>,
}

pub trait SelfSupportConstraintTrait: SupportConstraintTrait {
    fn self_supporting_part(&self) -> &Option<LockedTypeObject /* super::entity::PartData */>;
    fn self_supporting_part_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */>;
}

impl SelfSupportConstraintTrait for SelfSupportConstraint {
    fn self_supporting_part(&self) -> &Option<LockedTypeObject /* super::entity::PartData */> {
        &self.self_supporting_part
    }
    fn self_supporting_part_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PartData */> {
        &mut self.self_supporting_part
    }
}

impl SupportConstraintTrait for SelfSupportConstraint {
}

impl super::core::DataContainerTrait for SelfSupportConstraint {
}

pub static SELFSUPPORTCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelfSupportConstraint",
    name_hash: 3722139443,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUPPORTCONSTRAINT_TYPE_INFO),
        super_class_offset: offset_of!(SelfSupportConstraint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SelfSupportConstraint as Default>::default())),
            create_boxed: || Box::new(<SelfSupportConstraint as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "SelfSupportingPart",
                name_hash: 361062721,
                flags: MemberInfoFlags::new(0),
                field_type: "PartData",
                rust_offset: offset_of!(SelfSupportConstraint, self_supporting_part),
            },
        ],
    }),
    array_type: Some(SELFSUPPORTCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelfSupportConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        SELFSUPPORTCONSTRAINT_TYPE_INFO
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


pub static SELFSUPPORTCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelfSupportConstraint-Array",
    name_hash: 1453400711,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("SelfSupportConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SupportConstraint {
    pub _glacier_base: super::core::DataContainer,
}

pub trait SupportConstraintTrait: super::core::DataContainerTrait {
}

impl SupportConstraintTrait for SupportConstraint {
}

impl super::core::DataContainerTrait for SupportConstraint {
}

pub static SUPPORTCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SupportConstraint",
    name_hash: 3974023919,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(SupportConstraint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SupportConstraint as Default>::default())),
            create_boxed: || Box::new(<SupportConstraint as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SUPPORTCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SupportConstraint {
    fn type_info(&self) -> &'static TypeInfo {
        SUPPORTCONSTRAINT_TYPE_INFO
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


pub static SUPPORTCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SupportConstraint-Array",
    name_hash: 499951835,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("SupportConstraint"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DestructionComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub realm: super::core::Realm,
    pub health_transition_parts: Vec<Option<LockedTypeObject /* HealthTransitionPartData */>>,
    pub support_constraints: Vec<Option<LockedTypeObject /* SupportConstraint */>>,
    pub edge_model_info: EdgeModelInfo,
    pub networkable_enabled: bool,
}

pub trait DestructionComponentDataTrait: super::entity::ComponentDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn health_transition_parts(&self) -> &Vec<Option<LockedTypeObject /* HealthTransitionPartData */>>;
    fn health_transition_parts_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HealthTransitionPartData */>>;
    fn support_constraints(&self) -> &Vec<Option<LockedTypeObject /* SupportConstraint */>>;
    fn support_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SupportConstraint */>>;
    fn edge_model_info(&self) -> &EdgeModelInfo;
    fn edge_model_info_mut(&mut self) -> &mut EdgeModelInfo;
    fn networkable_enabled(&self) -> &bool;
    fn networkable_enabled_mut(&mut self) -> &mut bool;
}

impl DestructionComponentDataTrait for DestructionComponentData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn health_transition_parts(&self) -> &Vec<Option<LockedTypeObject /* HealthTransitionPartData */>> {
        &self.health_transition_parts
    }
    fn health_transition_parts_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* HealthTransitionPartData */>> {
        &mut self.health_transition_parts
    }
    fn support_constraints(&self) -> &Vec<Option<LockedTypeObject /* SupportConstraint */>> {
        &self.support_constraints
    }
    fn support_constraints_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* SupportConstraint */>> {
        &mut self.support_constraints
    }
    fn edge_model_info(&self) -> &EdgeModelInfo {
        &self.edge_model_info
    }
    fn edge_model_info_mut(&mut self) -> &mut EdgeModelInfo {
        &mut self.edge_model_info
    }
    fn networkable_enabled(&self) -> &bool {
        &self.networkable_enabled
    }
    fn networkable_enabled_mut(&mut self) -> &mut bool {
        &mut self.networkable_enabled
    }
}

impl super::entity::ComponentDataTrait for DestructionComponentData {
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

impl super::entity::GameObjectDataTrait for DestructionComponentData {
}

impl super::core::DataBusPeerTrait for DestructionComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DestructionComponentData {
}

impl super::core::DataContainerTrait for DestructionComponentData {
}

pub static DESTRUCTIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponentData",
    name_hash: 2062943396,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(DestructionComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionComponentData as Default>::default())),
            create_boxed: || Box::new(<DestructionComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(DestructionComponentData, realm),
            },
            FieldInfoData {
                name: "HealthTransitionParts",
                name_hash: 5168690,
                flags: MemberInfoFlags::new(144),
                field_type: "HealthTransitionPartData-Array",
                rust_offset: offset_of!(DestructionComponentData, health_transition_parts),
            },
            FieldInfoData {
                name: "SupportConstraints",
                name_hash: 2293770428,
                flags: MemberInfoFlags::new(144),
                field_type: "SupportConstraint-Array",
                rust_offset: offset_of!(DestructionComponentData, support_constraints),
            },
            FieldInfoData {
                name: "EdgeModelInfo",
                name_hash: 1407151719,
                flags: MemberInfoFlags::new(0),
                field_type: "EdgeModelInfo",
                rust_offset: offset_of!(DestructionComponentData, edge_model_info),
            },
            FieldInfoData {
                name: "NetworkableEnabled",
                name_hash: 1247463060,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(DestructionComponentData, networkable_enabled),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONCOMPONENTDATA_TYPE_INFO
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


pub static DESTRUCTIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponentData-Array",
    name_hash: 1510283280,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("DestructionComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EdgeModelInfo {
    pub edge_models_data: Option<LockedTypeObject /* super::world_base::EdgeModelsBaseData */>,
    pub rigid_bodies: Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>,
}

pub trait EdgeModelInfoTrait: TypeObject {
    fn edge_models_data(&self) -> &Option<LockedTypeObject /* super::world_base::EdgeModelsBaseData */>;
    fn edge_models_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::world_base::EdgeModelsBaseData */>;
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>;
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>>;
}

impl EdgeModelInfoTrait for EdgeModelInfo {
    fn edge_models_data(&self) -> &Option<LockedTypeObject /* super::world_base::EdgeModelsBaseData */> {
        &self.edge_models_data
    }
    fn edge_models_data_mut(&mut self) -> &mut Option<LockedTypeObject /* super::world_base::EdgeModelsBaseData */> {
        &mut self.edge_models_data
    }
    fn rigid_bodies(&self) -> &Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        &self.rigid_bodies
    }
    fn rigid_bodies_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::physics::RigidBodyData */>> {
        &mut self.rigid_bodies
    }
}

pub static EDGEMODELINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInfo",
    name_hash: 1407151719,
    flags: MemberInfoFlags::new(73),
    module: "Destruction",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelInfo as Default>::default())),
            create_boxed: || Box::new(<EdgeModelInfo as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "EdgeModelsData",
                name_hash: 3558352074,
                flags: MemberInfoFlags::new(0),
                field_type: "EdgeModelsBaseData",
                rust_offset: offset_of!(EdgeModelInfo, edge_models_data),
            },
            FieldInfoData {
                name: "RigidBodies",
                name_hash: 3015855522,
                flags: MemberInfoFlags::new(144),
                field_type: "RigidBodyData-Array",
                rust_offset: offset_of!(EdgeModelInfo, rigid_bodies),
            },
        ],
    }),
    array_type: Some(EDGEMODELINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EdgeModelInfo {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELINFO_TYPE_INFO
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


pub static EDGEMODELINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInfo-Array",
    name_hash: 1411027027,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("EdgeModelInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ServerHealthTransitionPart {
    pub _glacier_base: HealthTransitionPart,
}

pub trait ServerHealthTransitionPartTrait: HealthTransitionPartTrait {
}

impl ServerHealthTransitionPartTrait for ServerHealthTransitionPart {
}

impl HealthTransitionPartTrait for ServerHealthTransitionPart {
}

impl super::entity::EntityTrait for ServerHealthTransitionPart {
}

impl super::entity::EntityBusPeerTrait for ServerHealthTransitionPart {
}

pub static SERVERHEALTHTRANSITIONPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHealthTransitionPart",
    name_hash: 3527456196,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHTRANSITIONPART_TYPE_INFO),
        super_class_offset: offset_of!(ServerHealthTransitionPart, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerHealthTransitionPart as Default>::default())),
            create_boxed: || Box::new(<ServerHealthTransitionPart as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERHEALTHTRANSITIONPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHealthTransitionPart {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERHEALTHTRANSITIONPART_TYPE_INFO
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


pub static SERVERHEALTHTRANSITIONPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHealthTransitionPart-Array",
    name_hash: 3112458864,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ServerHealthTransitionPart"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthTransitionPart {
    pub _glacier_base: super::entity::Entity,
}

pub trait HealthTransitionPartTrait: super::entity::EntityTrait {
}

impl HealthTransitionPartTrait for HealthTransitionPart {
}

impl super::entity::EntityTrait for HealthTransitionPart {
}

impl super::entity::EntityBusPeerTrait for HealthTransitionPart {
}

pub static HEALTHTRANSITIONPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPart",
    name_hash: 1692113441,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(HealthTransitionPart, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthTransitionPart as Default>::default())),
            create_boxed: || Box::new(<HealthTransitionPart as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEALTHTRANSITIONPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HealthTransitionPart {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHTRANSITIONPART_TYPE_INFO
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


pub static HEALTHTRANSITIONPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPart-Array",
    name_hash: 834430101,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionPart"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct HealthTransition {
    pub _glacier_base: super::entity::Entity,
}

pub trait HealthTransitionTrait: super::entity::EntityTrait {
}

impl HealthTransitionTrait for HealthTransition {
}

impl super::entity::EntityTrait for HealthTransition {
}

impl super::entity::EntityBusPeerTrait for HealthTransition {
}

pub static HEALTHTRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransition",
    name_hash: 3520475862,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(HealthTransition, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<HealthTransition as Default>::default())),
            create_boxed: || Box::new(<HealthTransition as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(HEALTHTRANSITION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HealthTransition {
    fn type_info(&self) -> &'static TypeInfo {
        HEALTHTRANSITION_TYPE_INFO
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


pub static HEALTHTRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransition-Array",
    name_hash: 3998144610,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransition"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EdgeModelOwner {
}

pub trait EdgeModelOwnerTrait: TypeObject {
}

impl EdgeModelOwnerTrait for EdgeModelOwner {
}

pub static EDGEMODELOWNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelOwner",
    name_hash: 3498167816,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EdgeModelOwner as Default>::default())),
            create_boxed: || Box::new(<EdgeModelOwner as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(EDGEMODELOWNER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EdgeModelOwner {
    fn type_info(&self) -> &'static TypeInfo {
        EDGEMODELOWNER_TYPE_INFO
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


pub static EDGEMODELOWNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelOwner-Array",
    name_hash: 1900171324,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("EdgeModelOwner"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct DestructionComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait DestructionComponentTrait: super::entity::ComponentTrait {
}

impl DestructionComponentTrait for DestructionComponent {
}

impl super::entity::ComponentTrait for DestructionComponent {
}

impl super::entity::EntityBusPeerTrait for DestructionComponent {
}

pub static DESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponent",
    name_hash: 3123248532,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(DestructionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DestructionComponent as Default>::default())),
            create_boxed: || Box::new(<DestructionComponent as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        DESTRUCTIONCOMPONENT_TYPE_INFO
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


pub static DESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponent-Array",
    name_hash: 3636607392,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("DestructionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientHealthTransitionPart {
    pub _glacier_base: HealthTransitionPart,
}

pub trait ClientHealthTransitionPartTrait: HealthTransitionPartTrait {
}

impl ClientHealthTransitionPartTrait for ClientHealthTransitionPart {
}

impl HealthTransitionPartTrait for ClientHealthTransitionPart {
}

impl super::entity::EntityTrait for ClientHealthTransitionPart {
}

impl super::entity::EntityBusPeerTrait for ClientHealthTransitionPart {
}

pub static CLIENTHEALTHTRANSITIONPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHealthTransitionPart",
    name_hash: 2892462232,
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHTRANSITIONPART_TYPE_INFO),
        super_class_offset: offset_of!(ClientHealthTransitionPart, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientHealthTransitionPart as Default>::default())),
            create_boxed: || Box::new(<ClientHealthTransitionPart as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTHEALTHTRANSITIONPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientHealthTransitionPart {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTHEALTHTRANSITIONPART_TYPE_INFO
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


pub static CLIENTHEALTHTRANSITIONPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHealthTransitionPart-Array",
    name_hash: 1576051372,
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ClientHealthTransitionPart"),
    array_type: None,
    alignment: 8,
};


