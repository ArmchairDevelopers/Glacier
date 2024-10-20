use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_p_a2_client_types(registry: &mut TypeRegistry) {
    registry.register_type(PA2ATTITUDEENTITYDATA_TYPE_INFO);
    registry.register_type(PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2ATTITUDEASSET_TYPE_INFO);
    registry.register_type(PA2ATTITUDEASSET_ARRAY_TYPE_INFO);
    registry.register_type(PA2BEHAVIORENTITYDATA_TYPE_INFO);
    registry.register_type(PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETENTITYDATA_TYPE_INFO);
    registry.register_type(PA2TARGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETINGMODE_TYPE_INFO);
    registry.register_type(PA2TARGETINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETTYPE_TYPE_INFO);
    registry.register_type(PA2TARGETTYPE_ARRAY_TYPE_INFO);
    registry.register_type(TARGETFLAGS_TYPE_INFO);
    registry.register_type(TARGETFLAGS_ARRAY_TYPE_INFO);
    registry.register_type(PA2TARGETSTATE_TYPE_INFO);
    registry.register_type(PA2TARGETSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2TARGETENTITY_TYPE_INFO);
    registry.register_type(CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2BEHAVIORENTITY_TYPE_INFO);
    registry.register_type(CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPA2ATTITUDEENTITY_TYPE_INFO);
    registry.register_type(CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct PA2AttitudeEntityData {
    pub attitude: PA2AttitudeAsset,
}

pub const PA2ATTITUDEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Attitude",
                flags: MemberInfoFlags::new(0),
                field_type: PA2ATTITUDEASSET_TYPE_INFO,
                rust_offset: offset_of!(PA2AttitudeEntityData, attitude),
            },
        ],
    }),
    array_type: Some(PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2AttitudeEntityData {
    fn type_info() -> &'static TypeInfo {
        PA2ATTITUDEENTITYDATA_TYPE_INFO
    }
}


pub const PA2ATTITUDEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2AttitudeEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PA2AttitudeAsset {
    pub active_game_state: super::ant::AntRef,
    pub primary_target_min_duration: f32,
}

pub const PA2ATTITUDEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeAsset",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINERPOLICYASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ActiveGameState",
                flags: MemberInfoFlags::new(0),
                field_type: ANTREF_TYPE_INFO,
                rust_offset: offset_of!(PA2AttitudeAsset, active_game_state),
            },
            FieldInfoData {
                name: "PrimaryTargetMinDuration",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PA2AttitudeAsset, primary_target_min_duration),
            },
        ],
    }),
    array_type: Some(PA2ATTITUDEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PA2AttitudeAsset {
    fn type_info() -> &'static TypeInfo {
        PA2ATTITUDEASSET_TYPE_INFO
    }
}


pub const PA2ATTITUDEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2AttitudeAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2AttitudeAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PA2BehaviorEntityData {
    pub targeting_bone: super::entity::GameplayBones,
    pub primary_targeting_mode: PA2TargetingMode,
    pub glance_targeting_mode: PA2TargetingMode,
    pub override_primary_target_enabled: bool,
    pub override_primary_target_position: super::core::Vec3,
}

pub const PA2BEHAVIORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BehaviorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "TargetingBone",
                flags: MemberInfoFlags::new(0),
                field_type: GAMEPLAYBONES_TYPE_INFO,
                rust_offset: offset_of!(PA2BehaviorEntityData, targeting_bone),
            },
            FieldInfoData {
                name: "PrimaryTargetingMode",
                flags: MemberInfoFlags::new(0),
                field_type: PA2TARGETINGMODE_TYPE_INFO,
                rust_offset: offset_of!(PA2BehaviorEntityData, primary_targeting_mode),
            },
            FieldInfoData {
                name: "GlanceTargetingMode",
                flags: MemberInfoFlags::new(0),
                field_type: PA2TARGETINGMODE_TYPE_INFO,
                rust_offset: offset_of!(PA2BehaviorEntityData, glance_targeting_mode),
            },
            FieldInfoData {
                name: "OverridePrimaryTargetEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PA2BehaviorEntityData, override_primary_target_enabled),
            },
            FieldInfoData {
                name: "OverridePrimaryTargetPosition",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(PA2BehaviorEntityData, override_primary_target_position),
            },
        ],
    }),
    array_type: Some(PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PA2BehaviorEntityData {
    fn type_info() -> &'static TypeInfo {
        PA2BEHAVIORENTITYDATA_TYPE_INFO
    }
}


pub const PA2BEHAVIORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2BehaviorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2BehaviorEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct PA2TargetEntityData {
    pub transform: super::core::LinearTransform,
    pub radius: f32,
    pub enabled: bool,
    pub targeting_mode: PA2TargetingMode,
    pub target_type: PA2TargetType,
}

pub const PA2TARGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(PA2TargetEntityData, transform),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(PA2TargetEntityData, radius),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PA2TargetEntityData, enabled),
            },
            FieldInfoData {
                name: "TargetingMode",
                flags: MemberInfoFlags::new(0),
                field_type: PA2TARGETINGMODE_TYPE_INFO,
                rust_offset: offset_of!(PA2TargetEntityData, targeting_mode),
            },
            FieldInfoData {
                name: "TargetType",
                flags: MemberInfoFlags::new(0),
                field_type: PA2TARGETTYPE_TYPE_INFO,
                rust_offset: offset_of!(PA2TargetEntityData, target_type),
            },
        ],
    }),
    array_type: Some(PA2TARGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PA2TargetEntityData {
    fn type_info() -> &'static TypeInfo {
        PA2TARGETENTITYDATA_TYPE_INFO
    }
}


pub const PA2TARGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PA2TargetingMode {
    #[default]
    PA2TargetingMode_General = 0,
    PA2TargetingMode_ConnectedOnly = 1,
    PA2TargetingMode_UnConnectedOnly = 2,
    PA2TargetingMode_Disabled = 3,
}

pub const PA2TARGETINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetingMode",
    flags: MemberInfoFlags::new(49429),
    module: "PA2Client",
    data: TypeInfoData::Enum,
    array_type: Some(PA2TARGETINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PA2TargetingMode {
    fn type_info() -> &'static TypeInfo {
        PA2TARGETINGMODE_TYPE_INFO
    }
}


pub const PA2TARGETINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetingMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum PA2TargetType {
    #[default]
    PA2TargetType_General = 0,
    PA2TargetType_Primary = 1,
    PA2TargetType_Glance = 2,
}

pub const PA2TARGETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetType",
    flags: MemberInfoFlags::new(49429),
    module: "PA2Client",
    data: TypeInfoData::Enum,
    array_type: Some(PA2TARGETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PA2TargetType {
    fn type_info() -> &'static TypeInfo {
        PA2TARGETTYPE_TYPE_INFO
    }
}


pub const PA2TARGETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TargetFlags {
    #[default]
    TargetFlags_PrimaryCandidate = 0,
    TargetFlags_GlanceCandidate = 1,
}

pub const TARGETFLAGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetFlags",
    flags: MemberInfoFlags::new(49429),
    module: "PA2Client",
    data: TypeInfoData::Enum,
    array_type: Some(TARGETFLAGS_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TargetFlags {
    fn type_info() -> &'static TypeInfo {
        TARGETFLAGS_TYPE_INFO
    }
}


pub const TARGETFLAGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TargetFlags-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("TargetFlags-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PA2TargetState {
}

pub const PA2TARGETSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetState",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(PA2TARGETSTATE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PA2TargetState {
    fn type_info() -> &'static TypeInfo {
        PA2TARGETSTATE_TYPE_INFO
    }
}


pub const PA2TARGETSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PA2TargetState-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("PA2TargetState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPA2TargetEntity {
}

pub const CLIENTPA2TARGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetEntity",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2TargetEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPA2TARGETENTITY_TYPE_INFO
    }
}


pub const CLIENTPA2TARGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2TargetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2TargetEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPA2BehaviorEntity {
}

pub const CLIENTPA2BEHAVIORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2BehaviorEntity",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2BehaviorEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPA2BEHAVIORENTITY_TYPE_INFO
    }
}


pub const CLIENTPA2BEHAVIORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2BehaviorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2BehaviorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPA2AttitudeEntity {
}

pub const CLIENTPA2ATTITUDEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2AttitudeEntity",
    flags: MemberInfoFlags::new(101),
    module: "PA2Client",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPA2AttitudeEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPA2ATTITUDEENTITY_TYPE_INFO
    }
}


pub const CLIENTPA2ATTITUDEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPA2AttitudeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "PA2Client",
    data: TypeInfoData::Array("ClientPA2AttitudeEntity-Array"),
    array_type: None,
    alignment: 8,
};


