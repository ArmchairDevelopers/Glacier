use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_decal_types(registry: &mut TypeRegistry) {
    registry.register_type(DECALDYNAMICSTATE_TYPE_INFO);
    registry.register_type(DECALDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DECALSTATICSTATE_TYPE_INFO);
    registry.register_type(DECALSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DECALLIFETIME_TYPE_INFO);
    registry.register_type(DECALLIFETIME_ARRAY_TYPE_INFO);
    registry.register_type(DECALVOLUMEENTITY_TYPE_INFO);
    registry.register_type(DECALVOLUMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DECALENTITY_TYPE_INFO);
    registry.register_type(DECALENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecalDynamicState {
    pub is_detached: bool,
    pub field_flag_changed0: u8,
}

pub const DECALDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "Decal",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "IsDetached",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalDynamicState, is_detached),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DecalDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DECALDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DecalDynamicState {
    fn type_info() -> &'static TypeInfo {
        DECALDYNAMICSTATE_TYPE_INFO
    }
}


pub const DECALDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct DecalStaticState {
    pub transform: super::core::LinearTransform,
    pub asset: super::render_base::DecalTemplateBaseAsset,
    pub life_time: DecalLifeTime,
    pub apply_on_terrain: bool,
    pub exclusive_model: super::world_base::ModelHandle,
    pub exclusive_model_part: u32,
    pub ignored_models: Vec<super::world_base::ModelHandle>,
    pub field_flag_changed0: u8,
}

pub const DECALSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalStaticState",
    flags: MemberInfoFlags::new(73),
    module: "Decal",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, transform),
            },
            FieldInfoData {
                name: "Asset",
                flags: MemberInfoFlags::new(0),
                field_type: DECALTEMPLATEBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, asset),
            },
            FieldInfoData {
                name: "LifeTime",
                flags: MemberInfoFlags::new(0),
                field_type: DECALLIFETIME_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, life_time),
            },
            FieldInfoData {
                name: "ApplyOnTerrain",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, apply_on_terrain),
            },
            FieldInfoData {
                name: "ExclusiveModel",
                flags: MemberInfoFlags::new(0),
                field_type: MODELHANDLE_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, exclusive_model),
            },
            FieldInfoData {
                name: "ExclusiveModelPart",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, exclusive_model_part),
            },
            FieldInfoData {
                name: "IgnoredModels",
                flags: MemberInfoFlags::new(144),
                field_type: MODELHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, ignored_models),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(DecalStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(DECALSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DecalStaticState {
    fn type_info() -> &'static TypeInfo {
        DECALSTATICSTATE_TYPE_INFO
    }
}


pub const DECALSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum DecalLifeTime {
    #[default]
    DecalLifeTime_RingBuffer = 0,
    DecalLifeTime_RingBufferRecreate = 1,
}

pub const DECALLIFETIME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalLifeTime",
    flags: MemberInfoFlags::new(49429),
    module: "Decal",
    data: TypeInfoData::Enum,
    array_type: Some(DECALLIFETIME_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for DecalLifeTime {
    fn type_info() -> &'static TypeInfo {
        DECALLIFETIME_TYPE_INFO
    }
}


pub const DECALLIFETIME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalLifeTime-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalLifeTime-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecalVolumeEntity {
}

pub const DECALVOLUMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntity",
    flags: MemberInfoFlags::new(101),
    module: "Decal",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RENDERVOLUMEENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DECALVOLUMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DecalVolumeEntity {
    fn type_info() -> &'static TypeInfo {
        DECALVOLUMEENTITY_TYPE_INFO
    }
}


pub const DECALVOLUMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalVolumeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalVolumeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecalEntity {
}

pub const DECALENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntity",
    flags: MemberInfoFlags::new(101),
    module: "Decal",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DECALENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DecalEntity {
    fn type_info() -> &'static TypeInfo {
        DECALENTITY_TYPE_INFO
    }
}


pub const DECALENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DecalEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Decal",
    data: TypeInfoData::Array("DecalEntity-Array"),
    array_type: None,
    alignment: 8,
};


