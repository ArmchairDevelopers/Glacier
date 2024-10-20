use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_deformation_types(registry: &mut TypeRegistry) {
    registry.register_type(DEFORMATIONCOMPONENTDATA_TYPE_INFO);
    registry.register_type(DEFORMATIONCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(BONETOPARTMAPPING_TYPE_INFO);
    registry.register_type(BONETOPARTMAPPING_ARRAY_TYPE_INFO);
    registry.register_type(DEFORMATIONASSET_TYPE_INFO);
    registry.register_type(DEFORMATIONASSET_ARRAY_TYPE_INFO);
    registry.register_type(SERVERDEFORMATIONCOMPONENT_TYPE_INFO);
    registry.register_type(SERVERDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(DEFORMATIONRESOURCE_TYPE_INFO);
    registry.register_type(DEFORMATIONRESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(DEFORMATIONMANAGER_TYPE_INFO);
    registry.register_type(DEFORMATIONMANAGER_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTDEFORMATIONCOMPONENT_TYPE_INFO);
    registry.register_type(CLIENTDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct DeformationComponentData {
    pub skeleton_asset: super::entity::SkeletonAsset,
    pub deformation_asset: DeformationAsset,
    pub bone_to_part_mapping: Vec<BoneToPartMapping>,
    pub impulse_scale: f32,
    pub mass: f32,
}

pub const DEFORMATIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SkeletonAsset",
                flags: MemberInfoFlags::new(0),
                field_type: SKELETONASSET_TYPE_INFO,
                rust_offset: offset_of!(DeformationComponentData, skeleton_asset),
            },
            FieldInfoData {
                name: "DeformationAsset",
                flags: MemberInfoFlags::new(0),
                field_type: DEFORMATIONASSET_TYPE_INFO,
                rust_offset: offset_of!(DeformationComponentData, deformation_asset),
            },
            FieldInfoData {
                name: "BoneToPartMapping",
                flags: MemberInfoFlags::new(144),
                field_type: BONETOPARTMAPPING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(DeformationComponentData, bone_to_part_mapping),
            },
            FieldInfoData {
                name: "ImpulseScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DeformationComponentData, impulse_scale),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(DeformationComponentData, mass),
            },
        ],
    }),
    array_type: Some(DEFORMATIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DeformationComponentData {
    fn type_info() -> &'static TypeInfo {
        DEFORMATIONCOMPONENTDATA_TYPE_INFO
    }
}


pub const DEFORMATIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoneToPartMapping {
    pub bone_index: u32,
    pub part_index: u32,
}

pub const BONETOPARTMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneToPartMapping",
    flags: MemberInfoFlags::new(36937),
    module: "Deformation",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "BoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BoneToPartMapping, bone_index),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(BoneToPartMapping, part_index),
            },
        ],
    }),
    array_type: Some(BONETOPARTMAPPING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoneToPartMapping {
    fn type_info() -> &'static TypeInfo {
        BONETOPARTMAPPING_TYPE_INFO
    }
}


pub const BONETOPARTMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneToPartMapping-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("BoneToPartMapping-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeformationAsset {
    pub soft_bodies_simulation_resource: super::core::ResourceRef,
}

pub const DEFORMATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationAsset",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SoftBodiesSimulationResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(DeformationAsset, soft_bodies_simulation_resource),
            },
        ],
    }),
    array_type: Some(DEFORMATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DeformationAsset {
    fn type_info() -> &'static TypeInfo {
        DEFORMATIONASSET_TYPE_INFO
    }
}


pub const DEFORMATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ServerDeformationComponent {
}

pub const SERVERDEFORMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeformationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDeformationComponent {
    fn type_info() -> &'static TypeInfo {
        SERVERDEFORMATIONCOMPONENT_TYPE_INFO
    }
}


pub const SERVERDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeformationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("ServerDeformationComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeformationResource {
}

pub const DEFORMATIONRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationResource",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(DEFORMATIONRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DeformationResource {
    fn type_info() -> &'static TypeInfo {
        DEFORMATIONRESOURCE_TYPE_INFO
    }
}


pub const DEFORMATIONRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DeformationManager {
}

pub const DEFORMATIONMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationManager",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IGLOOSUBSYSTEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DEFORMATIONMANAGER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DeformationManager {
    fn type_info() -> &'static TypeInfo {
        DEFORMATIONMANAGER_TYPE_INFO
    }
}


pub const DEFORMATIONMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationManager-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientDeformationComponent {
}

pub const CLIENTDEFORMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDeformationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(COMPONENT_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDeformationComponent {
    fn type_info() -> &'static TypeInfo {
        CLIENTDEFORMATIONCOMPONENT_TYPE_INFO
    }
}


pub const CLIENTDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDeformationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("ClientDeformationComponent-Array"),
    array_type: None,
    alignment: 8,
};


