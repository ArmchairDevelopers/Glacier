use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ClothControlHandle {
}

pub trait ClothControlHandleTrait: TypeObject {
}

impl ClothControlHandleTrait for ClothControlHandle {
}

pub static CLOTHCONTROLHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlHandle",
    flags: MemberInfoFlags::new(73),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothControlHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLOTHCONTROLHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for ClothControlHandle {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCONTROLHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOTHCONTROLHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothControlHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SimulationDynamicState {
}

pub trait SimulationDynamicStateTrait: TypeObject {
}

impl SimulationDynamicStateTrait for SimulationDynamicState {
}

pub static SIMULATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimulationDynamicState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SimulationDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        SIMULATIONDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SimulationDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SimulationStaticState {
}

pub trait SimulationStaticStateTrait: TypeObject {
}

impl SimulationStaticStateTrait for SimulationStaticState {
}

pub static SIMULATIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimulationStaticState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SIMULATIONSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for SimulationStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        SIMULATIONSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SIMULATIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SimulationStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothWrappingAsset {
    pub _glacier_base: super::core::Asset,
    pub cloth_wrapping_asset_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait ClothWrappingAssetTrait: super::core::AssetTrait {
    fn cloth_wrapping_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl ClothWrappingAssetTrait for ClothWrappingAsset {
    fn cloth_wrapping_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.cloth_wrapping_asset_resource
    }
}

impl super::core::AssetTrait for ClothWrappingAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for ClothWrappingAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLOTHWRAPPINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothWrappingAsset",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothWrappingAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClothWrappingAssetResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(ClothWrappingAsset, cloth_wrapping_asset_resource),
            },
        ],
    }),
    array_type: Some(CLOTHWRAPPINGASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothWrappingAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHWRAPPINGASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOTHWRAPPINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothWrappingAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothWrappingAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ClothProcessingMode {
    #[default]
    ClothProcessingMode_Simulated = 0,
    ClothProcessingMode_Skinned = 1,
    ClothProcessingMode_Frozen = 2,
    ClothProcessingMode_Count = 3,
}

pub static CLOTHPROCESSINGMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothProcessingMode",
    flags: MemberInfoFlags::new(49429),
    module: "ClothBase",
    data: TypeInfoData::Enum,
    array_type: Some(CLOTHPROCESSINGMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ClothProcessingMode {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHPROCESSINGMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOTHPROCESSINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothProcessingMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothProcessingMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoxClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub half_lengths: super::core::Vec3,
}

pub trait BoxClothCollisionTrait: ClothCollisionGeometryTrait {
    fn half_lengths(&self) -> &super::core::Vec3;
}

impl BoxClothCollisionTrait for BoxClothCollision {
    fn half_lengths(&self) -> &super::core::Vec3 {
        &self.half_lengths
    }
}

impl ClothCollisionGeometryTrait for BoxClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::core::DataContainerTrait for BoxClothCollision {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static BOXCLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoxClothCollision as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "HalfLengths",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(BoxClothCollision, half_lengths),
            },
        ],
    }),
    array_type: Some(BOXCLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for BoxClothCollision {
    fn type_info(&self) -> &'static TypeInfo {
        BOXCLOTHCOLLISION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static BOXCLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("BoxClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TaperedCapsuleClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub radius_a: f32,
    pub radius_b: f32,
    pub half_length: f32,
}

pub trait TaperedCapsuleClothCollisionTrait: ClothCollisionGeometryTrait {
    fn radius_a(&self) -> &f32;
    fn radius_b(&self) -> &f32;
    fn half_length(&self) -> &f32;
}

impl TaperedCapsuleClothCollisionTrait for TaperedCapsuleClothCollision {
    fn radius_a(&self) -> &f32 {
        &self.radius_a
    }
    fn radius_b(&self) -> &f32 {
        &self.radius_b
    }
    fn half_length(&self) -> &f32 {
        &self.half_length
    }
}

impl ClothCollisionGeometryTrait for TaperedCapsuleClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::core::DataContainerTrait for TaperedCapsuleClothCollision {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static TAPEREDCAPSULECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TaperedCapsuleClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TaperedCapsuleClothCollision as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RadiusA",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TaperedCapsuleClothCollision, radius_a),
            },
            FieldInfoData {
                name: "RadiusB",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TaperedCapsuleClothCollision, radius_b),
            },
            FieldInfoData {
                name: "HalfLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TaperedCapsuleClothCollision, half_length),
            },
        ],
    }),
    array_type: Some(TAPEREDCAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TaperedCapsuleClothCollision {
    fn type_info(&self) -> &'static TypeInfo {
        TAPEREDCAPSULECLOTHCOLLISION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TAPEREDCAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TaperedCapsuleClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("TaperedCapsuleClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CapsuleClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub radius: f32,
    pub half_length: f32,
}

pub trait CapsuleClothCollisionTrait: ClothCollisionGeometryTrait {
    fn radius(&self) -> &f32;
    fn half_length(&self) -> &f32;
}

impl CapsuleClothCollisionTrait for CapsuleClothCollision {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn half_length(&self) -> &f32 {
        &self.half_length
    }
}

impl ClothCollisionGeometryTrait for CapsuleClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::core::DataContainerTrait for CapsuleClothCollision {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CAPSULECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CapsuleClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CapsuleClothCollision as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CapsuleClothCollision, radius),
            },
            FieldInfoData {
                name: "HalfLength",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CapsuleClothCollision, half_length),
            },
        ],
    }),
    array_type: Some(CAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for CapsuleClothCollision {
    fn type_info(&self) -> &'static TypeInfo {
        CAPSULECLOTHCOLLISION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CapsuleClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("CapsuleClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SphereClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub radius: f32,
}

pub trait SphereClothCollisionTrait: ClothCollisionGeometryTrait {
    fn radius(&self) -> &f32;
}

impl SphereClothCollisionTrait for SphereClothCollision {
    fn radius(&self) -> &f32 {
        &self.radius
    }
}

impl ClothCollisionGeometryTrait for SphereClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
}

impl super::core::DataContainerTrait for SphereClothCollision {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static SPHERECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereClothCollision",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereClothCollision as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SphereClothCollision, radius),
            },
        ],
    }),
    array_type: Some(SPHERECLOTHCOLLISION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for SphereClothCollision {
    fn type_info(&self) -> &'static TypeInfo {
        SPHERECLOTHCOLLISION_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SPHERECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereClothCollision-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SphereClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothCollisionGeometry {
    pub _glacier_base: super::core::DataContainer,
    pub transform: super::core::LinearTransform,
}

pub trait ClothCollisionGeometryTrait: super::core::DataContainerTrait {
    fn transform(&self) -> &super::core::LinearTransform;
}

impl ClothCollisionGeometryTrait for ClothCollisionGeometry {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
}

impl super::core::DataContainerTrait for ClothCollisionGeometry {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLOTHCOLLISIONGEOMETRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionGeometry",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothCollisionGeometry as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothCollisionGeometry, transform),
            },
        ],
    }),
    array_type: Some(CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothCollisionGeometry {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCOLLISIONGEOMETRY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionGeometry-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothCollisionGeometry"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothBaseAsset {
    pub _glacier_base: super::core::Asset,
    pub cloth_asset_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait ClothBaseAssetTrait: super::core::AssetTrait {
    fn cloth_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
}

impl ClothBaseAssetTrait for ClothBaseAsset {
    fn cloth_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.cloth_asset_resource
    }
}

impl super::core::AssetTrait for ClothBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for ClothBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static CLOTHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothBaseAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ClothAssetResource",
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(ClothBaseAsset, cloth_asset_resource),
            },
        ],
    }),
    array_type: Some(CLOTHBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ClothBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOTHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClothControlDynamicState {
    pub user_colliders: Vec<Option<Arc<Mutex<dyn ClothCollisionGeometryTrait>>>>,
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

pub trait ClothControlDynamicStateTrait: TypeObject {
    fn user_colliders(&self) -> &Vec<Option<Arc<Mutex<dyn ClothCollisionGeometryTrait>>>>;
    fn user_collider_joint_ids(&self) -> &Vec<u32>;
    fn authored_colliders_enabled(&self) -> &bool;
    fn root_transform(&self) -> &super::core::LinearTransform;
    fn ground_plane_transform(&self) -> &super::core::LinearTransform;
    fn next_processing_mode(&self) -> &ClothProcessingMode;
    fn trigger_re_initialize(&self) -> &bool;
    fn processing_mode_overridden(&self) -> &bool;
    fn enable_ground_plane(&self) -> &bool;
    fn use_authored_ground_plane(&self) -> &bool;
    fn force_skip_interpolation(&self) -> &bool;
    fn trigger_retarget(&self) -> &bool;
    fn trigger_teleport(&self) -> &bool;
    fn root_motion_contribution_enabled(&self) -> &bool;
    fn field_flag_changed0(&self) -> &u16;
}

impl ClothControlDynamicStateTrait for ClothControlDynamicState {
    fn user_colliders(&self) -> &Vec<Option<Arc<Mutex<dyn ClothCollisionGeometryTrait>>>> {
        &self.user_colliders
    }
    fn user_collider_joint_ids(&self) -> &Vec<u32> {
        &self.user_collider_joint_ids
    }
    fn authored_colliders_enabled(&self) -> &bool {
        &self.authored_colliders_enabled
    }
    fn root_transform(&self) -> &super::core::LinearTransform {
        &self.root_transform
    }
    fn ground_plane_transform(&self) -> &super::core::LinearTransform {
        &self.ground_plane_transform
    }
    fn next_processing_mode(&self) -> &ClothProcessingMode {
        &self.next_processing_mode
    }
    fn trigger_re_initialize(&self) -> &bool {
        &self.trigger_re_initialize
    }
    fn processing_mode_overridden(&self) -> &bool {
        &self.processing_mode_overridden
    }
    fn enable_ground_plane(&self) -> &bool {
        &self.enable_ground_plane
    }
    fn use_authored_ground_plane(&self) -> &bool {
        &self.use_authored_ground_plane
    }
    fn force_skip_interpolation(&self) -> &bool {
        &self.force_skip_interpolation
    }
    fn trigger_retarget(&self) -> &bool {
        &self.trigger_retarget
    }
    fn trigger_teleport(&self) -> &bool {
        &self.trigger_teleport
    }
    fn root_motion_contribution_enabled(&self) -> &bool {
        &self.root_motion_contribution_enabled
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static CLOTHCONTROLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothControlDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UserColliders",
                flags: MemberInfoFlags::new(144),
                field_type: "ClothCollisionGeometry-Array",
                rust_offset: offset_of!(ClothControlDynamicState, user_colliders),
            },
            FieldInfoData {
                name: "UserColliderJointIds",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(ClothControlDynamicState, user_collider_joint_ids),
            },
            FieldInfoData {
                name: "AuthoredCollidersEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, authored_colliders_enabled),
            },
            FieldInfoData {
                name: "RootTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothControlDynamicState, root_transform),
            },
            FieldInfoData {
                name: "GroundPlaneTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothControlDynamicState, ground_plane_transform),
            },
            FieldInfoData {
                name: "NextProcessingMode",
                flags: MemberInfoFlags::new(0),
                field_type: "ClothProcessingMode",
                rust_offset: offset_of!(ClothControlDynamicState, next_processing_mode),
            },
            FieldInfoData {
                name: "TriggerReInitialize",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, trigger_re_initialize),
            },
            FieldInfoData {
                name: "ProcessingModeOverridden",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, processing_mode_overridden),
            },
            FieldInfoData {
                name: "EnableGroundPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, enable_ground_plane),
            },
            FieldInfoData {
                name: "UseAuthoredGroundPlane",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, use_authored_ground_plane),
            },
            FieldInfoData {
                name: "ForceSkipInterpolation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, force_skip_interpolation),
            },
            FieldInfoData {
                name: "TriggerRetarget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, trigger_retarget),
            },
            FieldInfoData {
                name: "TriggerTeleport",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, trigger_teleport),
            },
            FieldInfoData {
                name: "RootMotionContributionEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, root_motion_contribution_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(ClothControlDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(CLOTHCONTROLDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ClothControlDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        CLOTHCONTROLDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLOTHCONTROLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothControlDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EAClothEntityData {
}

pub trait EAClothEntityDataTrait: TypeObject {
}

impl EAClothEntityDataTrait for EAClothEntityData {
}

pub static EACLOTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothEntityData",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EAClothEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EACLOTHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EAClothEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        EACLOTHENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EACLOTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("EAClothEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EAClothAssetData {
}

pub trait EAClothAssetDataTrait: TypeObject {
}

impl EAClothAssetDataTrait for EAClothAssetData {
}

pub static EACLOTHASSETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothAssetData",
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EAClothAssetData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EACLOTHASSETDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EAClothAssetData {
    fn type_info(&self) -> &'static TypeInfo {
        EACLOTHASSETDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static EACLOTHASSETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothAssetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("EAClothAssetData"),
    array_type: None,
    alignment: 8,
};


