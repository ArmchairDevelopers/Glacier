use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct DeformationComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub skeleton_asset: Option<Arc<Mutex<dyn super::entity::SkeletonAssetTrait>>>,
    pub deformation_asset: Option<Arc<Mutex<dyn DeformationAssetTrait>>>,
    pub bone_to_part_mapping: Vec<BoneToPartMapping>,
    pub impulse_scale: f32,
    pub mass: f32,
}

pub trait DeformationComponentDataTrait: super::entity::GameComponentDataTrait {
    fn skeleton_asset(&self) -> &Option<Arc<Mutex<dyn super::entity::SkeletonAssetTrait>>>;
    fn deformation_asset(&self) -> &Option<Arc<Mutex<dyn DeformationAssetTrait>>>;
    fn bone_to_part_mapping(&self) -> &Vec<BoneToPartMapping>;
    fn impulse_scale(&self) -> &f32;
    fn mass(&self) -> &f32;
}

impl DeformationComponentDataTrait for DeformationComponentData {
    fn skeleton_asset(&self) -> &Option<Arc<Mutex<dyn super::entity::SkeletonAssetTrait>>> {
        &self.skeleton_asset
    }
    fn deformation_asset(&self) -> &Option<Arc<Mutex<dyn DeformationAssetTrait>>> {
        &self.deformation_asset
    }
    fn bone_to_part_mapping(&self) -> &Vec<BoneToPartMapping> {
        &self.bone_to_part_mapping
    }
    fn impulse_scale(&self) -> &f32 {
        &self.impulse_scale
    }
    fn mass(&self) -> &f32 {
        &self.mass
    }
}

impl super::entity::GameComponentDataTrait for DeformationComponentData {
}

impl super::entity::ComponentDataTrait for DeformationComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
}

impl super::entity::GameObjectDataTrait for DeformationComponentData {
}

impl super::core::DataBusPeerTrait for DeformationComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for DeformationComponentData {
}

impl super::core::DataContainerTrait for DeformationComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEFORMATIONCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeformationComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SkeletonAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "SkeletonAsset",
                rust_offset: offset_of!(DeformationComponentData, skeleton_asset),
            },
            FieldInfoData {
                name: "DeformationAsset",
                flags: MemberInfoFlags::new(0),
                field_type: "DeformationAsset",
                rust_offset: offset_of!(DeformationComponentData, deformation_asset),
            },
            FieldInfoData {
                name: "BoneToPartMapping",
                flags: MemberInfoFlags::new(144),
                field_type: "BoneToPartMapping-Array",
                rust_offset: offset_of!(DeformationComponentData, bone_to_part_mapping),
            },
            FieldInfoData {
                name: "ImpulseScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DeformationComponentData, impulse_scale),
            },
            FieldInfoData {
                name: "Mass",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(DeformationComponentData, mass),
            },
        ],
    }),
    array_type: Some(DEFORMATIONCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for DeformationComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        DEFORMATIONCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFORMATIONCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoneToPartMapping {
    pub bone_index: u32,
    pub part_index: u32,
}

pub trait BoneToPartMappingTrait: TypeObject {
    fn bone_index(&self) -> &u32;
    fn part_index(&self) -> &u32;
}

impl BoneToPartMappingTrait for BoneToPartMapping {
    fn bone_index(&self) -> &u32 {
        &self.bone_index
    }
    fn part_index(&self) -> &u32 {
        &self.part_index
    }
}

pub static BONETOPARTMAPPING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneToPartMapping",
    flags: MemberInfoFlags::new(36937),
    module: "Deformation",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoneToPartMapping as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BoneIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BoneToPartMapping, bone_index),
            },
            FieldInfoData {
                name: "PartIndex",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(BoneToPartMapping, part_index),
            },
        ],
    }),
    array_type: Some(BONETOPARTMAPPING_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoneToPartMapping {
    fn type_info(&self) -> &'static TypeInfo {
        BONETOPARTMAPPING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BONETOPARTMAPPING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoneToPartMapping-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("BoneToPartMapping"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DeformationAsset {
    pub _glacier_base: super::core::Asset,
    pub soft_bodies_simulation_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait DeformationAssetTrait: super::core::AssetTrait {
    fn soft_bodies_simulation_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl DeformationAssetTrait for DeformationAsset {
    fn soft_bodies_simulation_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.soft_bodies_simulation_resource
    }
}

impl super::core::AssetTrait for DeformationAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for DeformationAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static DEFORMATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationAsset",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeformationAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SoftBodiesSimulationResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(DeformationAsset, soft_bodies_simulation_resource),
            },
        ],
    }),
    array_type: Some(DEFORMATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DeformationAsset {
    fn type_info(&self) -> &'static TypeInfo {
        DEFORMATIONASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFORMATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerDeformationComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait ServerDeformationComponentTrait: super::entity::ComponentTrait {
}

impl ServerDeformationComponentTrait for ServerDeformationComponent {
}

impl super::entity::ComponentTrait for ServerDeformationComponent {
}

impl super::entity::EntityBusPeerTrait for ServerDeformationComponent {
}

pub static SERVERDEFORMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeformationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerDeformationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerDeformationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERDEFORMATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SERVERDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerDeformationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("ServerDeformationComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DeformationResource {
}

pub trait DeformationResourceTrait: TypeObject {
}

impl DeformationResourceTrait for DeformationResource {
}

pub static DEFORMATIONRESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationResource",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeformationResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFORMATIONRESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DeformationResource {
    fn type_info(&self) -> &'static TypeInfo {
        DEFORMATIONRESOURCE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFORMATIONRESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DeformationManager {
    pub _glacier_base: super::physics::IglooSubsystem,
}

pub trait DeformationManagerTrait: super::physics::IglooSubsystemTrait {
}

impl DeformationManagerTrait for DeformationManager {
}

impl super::physics::IglooSubsystemTrait for DeformationManager {
}

pub static DEFORMATIONMANAGER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationManager",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::IGLOOSUBSYSTEM_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DeformationManager as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DEFORMATIONMANAGER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DeformationManager {
    fn type_info(&self) -> &'static TypeInfo {
        DEFORMATIONMANAGER_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DEFORMATIONMANAGER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DeformationManager-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("DeformationManager"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientDeformationComponent {
    pub _glacier_base: super::entity::Component,
}

pub trait ClientDeformationComponentTrait: super::entity::ComponentTrait {
}

impl ClientDeformationComponentTrait for ClientDeformationComponent {
}

impl super::entity::ComponentTrait for ClientDeformationComponent {
}

impl super::entity::EntityBusPeerTrait for ClientDeformationComponent {
}

pub static CLIENTDEFORMATIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDeformationComponent",
    flags: MemberInfoFlags::new(101),
    module: "Deformation",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientDeformationComponent as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientDeformationComponent {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTDEFORMATIONCOMPONENT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTDEFORMATIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientDeformationComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "Deformation",
    data: TypeInfoData::Array("ClientDeformationComponent"),
    array_type: None,
    alignment: 8,
};


