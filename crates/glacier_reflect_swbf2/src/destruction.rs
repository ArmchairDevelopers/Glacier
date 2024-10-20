use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Debug)]
pub struct ConnectivityEntityData {
    pub breakable_model_event_ids: Vec<i32>,
    pub static_model_event_ids: Vec<i32>,
    pub static_model_first_indices: Vec<u32>,
    pub static_model_to_breakable_parts_array: Vec<StaticModelToBreakableParts>,
    pub breakable_part_to_static_model_part_array: Vec<BreakablePartToStaticEntityPart>,
    pub breakable_model_extra_radius: f32,
}

pub const CONNECTIVITYENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectivityEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BreakableModelEventIds",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConnectivityEntityData, breakable_model_event_ids),
            },
            FieldInfoData {
                name: "StaticModelEventIds",
                flags: MemberInfoFlags::new(144),
                field_type: INT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConnectivityEntityData, static_model_event_ids),
            },
            FieldInfoData {
                name: "StaticModelFirstIndices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConnectivityEntityData, static_model_first_indices),
            },
            FieldInfoData {
                name: "StaticModelToBreakablePartsArray",
                flags: MemberInfoFlags::new(144),
                field_type: STATICMODELTOBREAKABLEPARTS_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConnectivityEntityData, static_model_to_breakable_parts_array),
            },
            FieldInfoData {
                name: "BreakablePartToStaticModelPartArray",
                flags: MemberInfoFlags::new(144),
                field_type: BREAKABLEPARTTOSTATICENTITYPART_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ConnectivityEntityData, breakable_part_to_static_model_part_array),
            },
            FieldInfoData {
                name: "BreakableModelExtraRadius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ConnectivityEntityData, breakable_model_extra_radius),
            },
        ],
    }),
    array_type: Some(CONNECTIVITYENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConnectivityEntityData {
    fn type_info() -> &'static TypeInfo {
        CONNECTIVITYENTITYDATA_TYPE_INFO
    }
}


pub const CONNECTIVITYENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectivityEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ConnectivityEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BreakablePartToStaticEntityPart {
    pub breakable_part_index: u32,
    pub static_model_part_index: u32,
}

pub const BREAKABLEPARTTOSTATICENTITYPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BreakablePartToStaticEntityPart",
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BreakablePartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BreakablePartToStaticEntityPart, breakable_part_index),
            },
            FieldInfoData {
                name: "StaticModelPartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BreakablePartToStaticEntityPart, static_model_part_index),
            },
        ],
    }),
    array_type: Some(BREAKABLEPARTTOSTATICENTITYPART_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BreakablePartToStaticEntityPart {
    fn type_info() -> &'static TypeInfo {
        BREAKABLEPARTTOSTATICENTITYPART_TYPE_INFO
    }
}


pub const BREAKABLEPARTTOSTATICENTITYPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BreakablePartToStaticEntityPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("BreakablePartToStaticEntityPart-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StaticModelToBreakableParts {
    pub static_model_index: u32,
    pub breakable_part_start_index: u32,
    pub breakable_part_count: u32,
}

pub const STATICMODELTOBREAKABLEPARTS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticModelToBreakableParts",
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "StaticModelIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(StaticModelToBreakableParts, static_model_index),
            },
            FieldInfoData {
                name: "BreakablePartStartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(StaticModelToBreakableParts, breakable_part_start_index),
            },
            FieldInfoData {
                name: "BreakablePartCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(StaticModelToBreakableParts, breakable_part_count),
            },
        ],
    }),
    array_type: Some(STATICMODELTOBREAKABLEPARTS_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for StaticModelToBreakableParts {
    fn type_info() -> &'static TypeInfo {
        STATICMODELTOBREAKABLEPARTS_TYPE_INFO
    }
}


pub const STATICMODELTOBREAKABLEPARTS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StaticModelToBreakableParts-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("StaticModelToBreakableParts-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DestructionVolumeComponentData {
    pub destruction_volume_data: super::world_base::DestructionVolumeBaseData,
}

pub const DESTRUCTIONVOLUMECOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "DestructionVolumeData",
                flags: MemberInfoFlags::new(0),
                field_type: DESTRUCTIONVOLUMEBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(DestructionVolumeComponentData, destruction_volume_data),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONVOLUMECOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionVolumeComponentData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONVOLUMECOMPONENTDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONVOLUMECOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionVolumeComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("DestructionVolumeComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionComponentOnHealthTransitionTriggeredMessage {
}

pub const DESTRUCTIONCOMPONENTONHEALTHTRANSITIONTRIGGEREDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponentOnHealthTransitionTriggeredMessage",
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for DestructionComponentOnHealthTransitionTriggeredMessage {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONCOMPONENTONHEALTHTRANSITIONTRIGGEREDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct HealthTransitionSpawnReferenceObjectData {
    pub spawn_linear_velocity: super::core::Vec3,
    pub spawn_angular_velocity: super::core::Vec3,
}

pub const HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionSpawnReferenceObjectData",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SPATIALREFERENCEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SpawnLinearVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionSpawnReferenceObjectData, spawn_linear_velocity),
            },
            FieldInfoData {
                name: "SpawnAngularVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionSpawnReferenceObjectData, spawn_angular_velocity),
            },
        ],
    }),
    array_type: Some(HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for HealthTransitionSpawnReferenceObjectData {
    fn type_info() -> &'static TypeInfo {
        HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_TYPE_INFO
    }
}


pub const HEALTHTRANSITIONSPAWNREFERENCEOBJECTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionSpawnReferenceObjectData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionSpawnReferenceObjectData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HealthTransitionData {
    pub part_state: super::entity::PartState,
    pub health_transition_value: u32,
    pub objects: Vec<super::entity::GameObjectData>,
    pub in_event_id: u32,
    pub out_event_id: u32,
}

pub const HEALTHTRANSITIONDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionData",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PartState",
                flags: MemberInfoFlags::new(0),
                field_type: PARTSTATE_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionData, part_state),
            },
            FieldInfoData {
                name: "HealthTransitionValue",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionData, health_transition_value),
            },
            FieldInfoData {
                name: "Objects",
                flags: MemberInfoFlags::new(144),
                field_type: GAMEOBJECTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionData, objects),
            },
            FieldInfoData {
                name: "InEventId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionData, in_event_id),
            },
            FieldInfoData {
                name: "OutEventId",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionData, out_event_id),
            },
        ],
    }),
    array_type: Some(HEALTHTRANSITIONDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthTransitionData {
    fn type_info() -> &'static TypeInfo {
        HEALTHTRANSITIONDATA_TYPE_INFO
    }
}


pub const HEALTHTRANSITIONDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct HealthTransitionPartData {
    pub part: super::entity::PartData,
    pub health_transitions: Vec<HealthTransitionData>,
    pub health: u32,
    pub radiosity_material_data: Vec<PartRadiosityMaterialData>,
}

pub const HEALTHTRANSITIONPARTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPartData",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Part",
                flags: MemberInfoFlags::new(0),
                field_type: PARTDATA_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionPartData, part),
            },
            FieldInfoData {
                name: "HealthTransitions",
                flags: MemberInfoFlags::new(144),
                field_type: HEALTHTRANSITIONDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionPartData, health_transitions),
            },
            FieldInfoData {
                name: "Health",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionPartData, health),
            },
            FieldInfoData {
                name: "RadiosityMaterialData",
                flags: MemberInfoFlags::new(144),
                field_type: PARTRADIOSITYMATERIALDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(HealthTransitionPartData, radiosity_material_data),
            },
        ],
    }),
    array_type: Some(HEALTHTRANSITIONPARTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for HealthTransitionPartData {
    fn type_info() -> &'static TypeInfo {
        HEALTHTRANSITIONPARTDATA_TYPE_INFO
    }
}


pub const HEALTHTRANSITIONPARTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPartData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionPartData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PartRadiosityMaterialData {
    pub material_guid: super::core::Guid,
    pub default_opacity: f32,
}

pub const PARTRADIOSITYMATERIALDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartRadiosityMaterialData",
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MaterialGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(PartRadiosityMaterialData, material_guid),
            },
            FieldInfoData {
                name: "DefaultOpacity",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PartRadiosityMaterialData, default_opacity),
            },
        ],
    }),
    array_type: Some(PARTRADIOSITYMATERIALDATA_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for PartRadiosityMaterialData {
    fn type_info() -> &'static TypeInfo {
        PARTRADIOSITYMATERIALDATA_TYPE_INFO
    }
}


pub const PARTRADIOSITYMATERIALDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PartRadiosityMaterialData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("PartRadiosityMaterialData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CalculateConnectedPartsPipelineResult {
    pub success: bool,
    pub touching_part_pairs: Vec<TouchingPartPair>,
}

pub const CALCULATECONNECTEDPARTSPIPELINERESULT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineResult",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Success",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CalculateConnectedPartsPipelineResult, success),
            },
            FieldInfoData {
                name: "TouchingPartPairs",
                flags: MemberInfoFlags::new(144),
                field_type: TOUCHINGPARTPAIR_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CalculateConnectedPartsPipelineResult, touching_part_pairs),
            },
        ],
    }),
    array_type: Some(CALCULATECONNECTEDPARTSPIPELINERESULT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CalculateConnectedPartsPipelineResult {
    fn type_info() -> &'static TypeInfo {
        CALCULATECONNECTEDPARTSPIPELINERESULT_TYPE_INFO
    }
}


pub const CALCULATECONNECTEDPARTSPIPELINERESULT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineResult-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("CalculateConnectedPartsPipelineResult-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CalculateConnectedPartsPipelineParams {
}

pub const CALCULATECONNECTEDPARTSPIPELINEPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineParams",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CALCULATECONNECTEDPARTSPIPELINEPARAMS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CalculateConnectedPartsPipelineParams {
    fn type_info() -> &'static TypeInfo {
        CALCULATECONNECTEDPARTSPIPELINEPARAMS_TYPE_INFO
    }
}


pub const CALCULATECONNECTEDPARTSPIPELINEPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CalculateConnectedPartsPipelineParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("CalculateConnectedPartsPipelineParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TouchingPartPair {
    pub guid0: super::core::Guid,
    pub guid1: super::core::Guid,
}

pub const TOUCHINGPARTPAIR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TouchingPartPair",
    flags: MemberInfoFlags::new(36937),
    module: "Destruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Guid0",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(TouchingPartPair, guid0),
            },
            FieldInfoData {
                name: "Guid1",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(TouchingPartPair, guid1),
            },
        ],
    }),
    array_type: Some(TOUCHINGPARTPAIR_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for TouchingPartPair {
    fn type_info() -> &'static TypeInfo {
        TOUCHINGPARTPAIR_TYPE_INFO
    }
}


pub const TOUCHINGPARTPAIR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TouchingPartPair-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("TouchingPartPair-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ConnectionConstraint {
    pub connected_part1: super::entity::PartData,
    pub connected_part2: super::entity::PartData,
}

pub const CONNECTIONCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectionConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUPPORTCONSTRAINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ConnectedPart1",
                flags: MemberInfoFlags::new(0),
                field_type: PARTDATA_TYPE_INFO,
                rust_offset: offset_of!(ConnectionConstraint, connected_part1),
            },
            FieldInfoData {
                name: "ConnectedPart2",
                flags: MemberInfoFlags::new(0),
                field_type: PARTDATA_TYPE_INFO,
                rust_offset: offset_of!(ConnectionConstraint, connected_part2),
            },
        ],
    }),
    array_type: Some(CONNECTIONCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConnectionConstraint {
    fn type_info() -> &'static TypeInfo {
        CONNECTIONCONSTRAINT_TYPE_INFO
    }
}


pub const CONNECTIONCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConnectionConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ConnectionConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SelfSupportConstraint {
    pub self_supporting_part: super::entity::PartData,
}

pub const SELFSUPPORTCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelfSupportConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SUPPORTCONSTRAINT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SelfSupportingPart",
                flags: MemberInfoFlags::new(0),
                field_type: PARTDATA_TYPE_INFO,
                rust_offset: offset_of!(SelfSupportConstraint, self_supporting_part),
            },
        ],
    }),
    array_type: Some(SELFSUPPORTCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelfSupportConstraint {
    fn type_info() -> &'static TypeInfo {
        SELFSUPPORTCONSTRAINT_TYPE_INFO
    }
}


pub const SELFSUPPORTCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelfSupportConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("SelfSupportConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SupportConstraint {
}

pub const SUPPORTCONSTRAINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SupportConstraint",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SUPPORTCONSTRAINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SupportConstraint {
    fn type_info() -> &'static TypeInfo {
        SUPPORTCONSTRAINT_TYPE_INFO
    }
}


pub const SUPPORTCONSTRAINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SupportConstraint-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("SupportConstraint-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DestructionComponentData {
    pub realm: super::core::Realm,
    pub health_transition_parts: Vec<HealthTransitionPartData>,
    pub support_constraints: Vec<SupportConstraint>,
    pub edge_model_info: EdgeModelInfo,
    pub networkable_enabled: bool,
}

pub const DESTRUCTIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(DestructionComponentData, realm),
            },
            FieldInfoData {
                name: "HealthTransitionParts",
                flags: MemberInfoFlags::new(144),
                field_type: HEALTHTRANSITIONPARTDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DestructionComponentData, health_transition_parts),
            },
            FieldInfoData {
                name: "SupportConstraints",
                flags: MemberInfoFlags::new(144),
                field_type: SUPPORTCONSTRAINT_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DestructionComponentData, support_constraints),
            },
            FieldInfoData {
                name: "EdgeModelInfo",
                flags: MemberInfoFlags::new(0),
                field_type: EDGEMODELINFO_TYPE_INFO,
                rust_offset: offset_of!(DestructionComponentData, edge_model_info),
            },
            FieldInfoData {
                name: "NetworkableEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DestructionComponentData, networkable_enabled),
            },
        ],
    }),
    array_type: Some(DESTRUCTIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DestructionComponentData {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const DESTRUCTIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("DestructionComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EdgeModelInfo {
    pub edge_models_data: super::world_base::EdgeModelsBaseData,
    pub rigid_bodies: Vec<super::physics::RigidBodyData>,
}

pub const EDGEMODELINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInfo",
    flags: MemberInfoFlags::new(73),
    module: "Destruction",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "EdgeModelsData",
                flags: MemberInfoFlags::new(0),
                field_type: EDGEMODELSBASEDATA_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelInfo, edge_models_data),
            },
            FieldInfoData {
                name: "RigidBodies",
                flags: MemberInfoFlags::new(144),
                field_type: RIGIDBODYDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EdgeModelInfo, rigid_bodies),
            },
        ],
    }),
    array_type: Some(EDGEMODELINFO_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EdgeModelInfo {
    fn type_info() -> &'static TypeInfo {
        EDGEMODELINFO_TYPE_INFO
    }
}


pub const EDGEMODELINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("EdgeModelInfo-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerHealthTransitionPart {
}

pub const SERVERHEALTHTRANSITIONPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHealthTransitionPart",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHTRANSITIONPART_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERHEALTHTRANSITIONPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerHealthTransitionPart {
    fn type_info() -> &'static TypeInfo {
        SERVERHEALTHTRANSITIONPART_TYPE_INFO
    }
}


pub const SERVERHEALTHTRANSITIONPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerHealthTransitionPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ServerHealthTransitionPart-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HealthTransitionPart {
}

pub const HEALTHTRANSITIONPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPart",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HEALTHTRANSITIONPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HealthTransitionPart {
    fn type_info() -> &'static TypeInfo {
        HEALTHTRANSITIONPART_TYPE_INFO
    }
}


pub const HEALTHTRANSITIONPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransitionPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransitionPart-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HealthTransition {
}

pub const HEALTHTRANSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransition",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(HEALTHTRANSITION_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for HealthTransition {
    fn type_info() -> &'static TypeInfo {
        HEALTHTRANSITION_TYPE_INFO
    }
}


pub const HEALTHTRANSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "HealthTransition-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("HealthTransition-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EdgeModelOwner {
}

pub const EDGEMODELOWNER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelOwner",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(EDGEMODELOWNER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EdgeModelOwner {
    fn type_info() -> &'static TypeInfo {
        EDGEMODELOWNER_TYPE_INFO
    }
}


pub const EDGEMODELOWNER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EdgeModelOwner-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("EdgeModelOwner-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DestructionComponent {
}

pub const DESTRUCTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponent",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DestructionComponent {
    fn type_info() -> &'static TypeInfo {
        DESTRUCTIONCOMPONENT_TYPE_INFO
    }
}


pub const DESTRUCTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DestructionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("DestructionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientHealthTransitionPart {
}

pub const CLIENTHEALTHTRANSITIONPART_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHealthTransitionPart",
    flags: MemberInfoFlags::new(101),
    module: "Destruction",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(HEALTHTRANSITIONPART_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTHEALTHTRANSITIONPART_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientHealthTransitionPart {
    fn type_info() -> &'static TypeInfo {
        CLIENTHEALTHTRANSITIONPART_TYPE_INFO
    }
}


pub const CLIENTHEALTHTRANSITIONPART_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientHealthTransitionPart-Array",
    flags: MemberInfoFlags::new(145),
    module: "Destruction",
    data: TypeInfoData::Array("ClientHealthTransitionPart-Array"),
    array_type: None,
    alignment: 8,
};


