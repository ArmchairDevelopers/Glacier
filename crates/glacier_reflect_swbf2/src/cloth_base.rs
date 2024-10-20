use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_cloth_base_types(registry: &mut TypeRegistry) {
    registry.register_type(CLOTHCONTROLHANDLE_TYPE_INFO);
    registry.register_type(CLOTHCONTROLHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(SIMULATIONDYNAMICSTATE_TYPE_INFO);
    registry.register_type(SIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(SIMULATIONSTATICSTATE_TYPE_INFO);
    registry.register_type(SIMULATIONSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHWRAPPINGASSET_TYPE_INFO);
    registry.register_type(CLOTHWRAPPINGASSET_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHPROCESSINGMODE_TYPE_INFO);
    registry.register_type(CLOTHPROCESSINGMODE_ARRAY_TYPE_INFO);
    registry.register_type(BOXCLOTHCOLLISION_TYPE_INFO);
    registry.register_type(BOXCLOTHCOLLISION_ARRAY_TYPE_INFO);
    registry.register_type(TAPEREDCAPSULECLOTHCOLLISION_TYPE_INFO);
    registry.register_type(TAPEREDCAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO);
    registry.register_type(CAPSULECLOTHCOLLISION_TYPE_INFO);
    registry.register_type(CAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO);
    registry.register_type(SPHERECLOTHCOLLISION_TYPE_INFO);
    registry.register_type(SPHERECLOTHCOLLISION_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHCOLLISIONGEOMETRY_TYPE_INFO);
    registry.register_type(CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHBASEASSET_TYPE_INFO);
    registry.register_type(CLOTHBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(CLOTHCONTROLDYNAMICSTATE_TYPE_INFO);
    registry.register_type(CLOTHCONTROLDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(EACLOTHENTITYDATA_TYPE_INFO);
    registry.register_type(EACLOTHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EACLOTHASSETDATA_TYPE_INFO);
    registry.register_type(EACLOTHASSETDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothControlHandle {
}

pub const CLOTHCONTROLHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlHandle",
    flags: MemberInfoFlags::new(73),
    module: "ClothBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(CLOTHCONTROLHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ClothControlHandle {
    fn type_info() -> &'static TypeInfo {
        CLOTHCONTROLHANDLE_TYPE_INFO
    }
}


pub const CLOTHCONTROLHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothControlHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimulationDynamicState {
}

pub const SIMULATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "ClothBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SimulationDynamicState {
    fn type_info() -> &'static TypeInfo {
        SIMULATIONDYNAMICSTATE_TYPE_INFO
    }
}


pub const SIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SimulationDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimulationStaticState {
}

pub const SIMULATIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "ClothBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(SIMULATIONSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SimulationStaticState {
    fn type_info() -> &'static TypeInfo {
        SIMULATIONSTATICSTATE_TYPE_INFO
    }
}


pub const SIMULATIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SimulationStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothWrappingAsset {
    pub cloth_wrapping_asset_resource: super::core::ResourceRef,
}

pub const CLOTHWRAPPINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothWrappingAsset",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClothWrappingAssetResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(ClothWrappingAsset, cloth_wrapping_asset_resource),
            },
        ],
    }),
    array_type: Some(CLOTHWRAPPINGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothWrappingAsset {
    fn type_info() -> &'static TypeInfo {
        CLOTHWRAPPINGASSET_TYPE_INFO
    }
}


pub const CLOTHWRAPPINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothWrappingAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothWrappingAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ClothProcessingMode {
    #[default]
    ClothProcessingMode_Simulated = 0,
    ClothProcessingMode_Skinned = 1,
    ClothProcessingMode_Frozen = 2,
    ClothProcessingMode_Count = 3,
}

pub const CLOTHPROCESSINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothProcessingMode",
    flags: MemberInfoFlags::new(49429),
    module: "ClothBase",
    data: TypeInfoData::Enum,
    array_type: Some(CLOTHPROCESSINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClothProcessingMode {
    fn type_info() -> &'static TypeInfo {
        CLOTHPROCESSINGMODE_TYPE_INFO
    }
}


pub const CLOTHPROCESSINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothProcessingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothProcessingMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoxClothCollision {
    pub half_lengths: super::core::Vec3,
}

pub const BOXCLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "HalfLengths",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(BoxClothCollision, half_lengths),
            },
        ],
    }),
    array_type: Some(BOXCLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoxClothCollision {
    fn type_info() -> &'static TypeInfo {
        BOXCLOTHCOLLISION_TYPE_INFO
    }
}


pub const BOXCLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("BoxClothCollision-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TaperedCapsuleClothCollision {
    pub radius_a: f32,
    pub radius_b: f32,
    pub half_length: f32,
}

pub const TAPEREDCAPSULECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TaperedCapsuleClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RadiusA",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TaperedCapsuleClothCollision, radius_a),
            },
            FieldInfoData {
                name: "RadiusB",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TaperedCapsuleClothCollision, radius_b),
            },
            FieldInfoData {
                name: "HalfLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TaperedCapsuleClothCollision, half_length),
            },
        ],
    }),
    array_type: Some(TAPEREDCAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TaperedCapsuleClothCollision {
    fn type_info() -> &'static TypeInfo {
        TAPEREDCAPSULECLOTHCOLLISION_TYPE_INFO
    }
}


pub const TAPEREDCAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TaperedCapsuleClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("TaperedCapsuleClothCollision-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CapsuleClothCollision {
    pub radius: f32,
    pub half_length: f32,
}

pub const CAPSULECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CapsuleClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CapsuleClothCollision, radius),
            },
            FieldInfoData {
                name: "HalfLength",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(CapsuleClothCollision, half_length),
            },
        ],
    }),
    array_type: Some(CAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CapsuleClothCollision {
    fn type_info() -> &'static TypeInfo {
        CAPSULECLOTHCOLLISION_TYPE_INFO
    }
}


pub const CAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CapsuleClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("CapsuleClothCollision-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SphereClothCollision {
    pub radius: f32,
}

pub const SPHERECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SphereClothCollision, radius),
            },
        ],
    }),
    array_type: Some(SPHERECLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SphereClothCollision {
    fn type_info() -> &'static TypeInfo {
        SPHERECLOTHCOLLISION_TYPE_INFO
    }
}


pub const SPHERECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SphereClothCollision-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothCollisionGeometry {
    pub transform: super::core::LinearTransform,
}

pub const CLOTHCOLLISIONGEOMETRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionGeometry",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ClothCollisionGeometry, transform),
            },
        ],
    }),
    array_type: Some(CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothCollisionGeometry {
    fn type_info() -> &'static TypeInfo {
        CLOTHCOLLISIONGEOMETRY_TYPE_INFO
    }
}


pub const CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionGeometry-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothCollisionGeometry-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClothBaseAsset {
    pub cloth_asset_resource: super::core::ResourceRef,
}

pub const CLOTHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ClothAssetResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(ClothBaseAsset, cloth_asset_resource),
            },
        ],
    }),
    array_type: Some(CLOTHBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothBaseAsset {
    fn type_info() -> &'static TypeInfo {
        CLOTHBASEASSET_TYPE_INFO
    }
}


pub const CLOTHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ClothControlDynamicState {
    pub user_colliders: Vec<ClothCollisionGeometry>,
    pub user_collider_joint_ids: Vec<u32>,
    pub authored_colliders_enabled: bool,
    pub root_transform: super::core::LinearTransform,
    pub ground_plane_transform: super::core::LinearTransform,
    pub next_processing_mode: ClothProcessingMode,
    pub trigger_re_initialize: bool,
    pub processing_mode_overridden: bool,
    pub enable_ground_plane: bool,
    pub use_authored_ground_plane: bool,
    pub force_skip_interpolation: bool,
    pub trigger_retarget: bool,
    pub trigger_teleport: bool,
    pub root_motion_contribution_enabled: bool,
    pub field_flag_changed0: u16,
}

pub const CLOTHCONTROLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "ClothBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "UserColliders",
                flags: MemberInfoFlags::new(144),
                field_type: CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, user_colliders),
            },
            FieldInfoData {
                name: "UserColliderJointIds",
                flags: MemberInfoFlags::new(144),
                field_type: UINT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, user_collider_joint_ids),
            },
            FieldInfoData {
                name: "AuthoredCollidersEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, authored_colliders_enabled),
            },
            FieldInfoData {
                name: "RootTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, root_transform),
            },
            FieldInfoData {
                name: "GroundPlaneTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, ground_plane_transform),
            },
            FieldInfoData {
                name: "NextProcessingMode",
                flags: MemberInfoFlags::new(0),
                field_type: CLOTHPROCESSINGMODE_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, next_processing_mode),
            },
            FieldInfoData {
                name: "TriggerReInitialize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, trigger_re_initialize),
            },
            FieldInfoData {
                name: "ProcessingModeOverridden",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, processing_mode_overridden),
            },
            FieldInfoData {
                name: "EnableGroundPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, enable_ground_plane),
            },
            FieldInfoData {
                name: "UseAuthoredGroundPlane",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, use_authored_ground_plane),
            },
            FieldInfoData {
                name: "ForceSkipInterpolation",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, force_skip_interpolation),
            },
            FieldInfoData {
                name: "TriggerRetarget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, trigger_retarget),
            },
            FieldInfoData {
                name: "TriggerTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, trigger_teleport),
            },
            FieldInfoData {
                name: "RootMotionContributionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, root_motion_contribution_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(ClothControlDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CLOTHCONTROLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothControlDynamicState {
    fn type_info() -> &'static TypeInfo {
        CLOTHCONTROLDYNAMICSTATE_TYPE_INFO
    }
}


pub const CLOTHCONTROLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothControlDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EAClothEntityData {
}

pub const EACLOTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(EACLOTHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EAClothEntityData {
    fn type_info() -> &'static TypeInfo {
        EACLOTHENTITYDATA_TYPE_INFO
    }
}


pub const EACLOTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("EAClothEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EAClothAssetData {
}

pub const EACLOTHASSETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothAssetData",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(EACLOTHASSETDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EAClothAssetData {
    fn type_info() -> &'static TypeInfo {
        EACLOTHASSETDATA_TYPE_INFO
    }
}


pub const EACLOTHASSETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothAssetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("EAClothAssetData-Array"),
    array_type: None,
    alignment: 8,
};


