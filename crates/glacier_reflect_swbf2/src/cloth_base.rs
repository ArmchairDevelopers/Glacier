use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothControlHandle {
}

pub trait ClothControlHandleTrait: TypeObject {
}

impl ClothControlHandleTrait for ClothControlHandle {
}

pub static CLOTHCONTROLHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlHandle",
    name_hash: 109887540,
    flags: MemberInfoFlags::new(73),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothControlHandle as Default>::default())),
            create_boxed: || Box::new(<ClothControlHandle as Default>::default()),
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


pub static CLOTHCONTROLHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlHandle-Array",
    name_hash: 3576511360,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothControlHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SimulationDynamicState {
}

pub trait SimulationDynamicStateTrait: TypeObject {
}

impl SimulationDynamicStateTrait for SimulationDynamicState {
}

pub static SIMULATIONDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationDynamicState",
    name_hash: 4102825876,
    flags: MemberInfoFlags::new(36937),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimulationDynamicState as Default>::default())),
            create_boxed: || Box::new(<SimulationDynamicState as Default>::default()),
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


pub static SIMULATIONDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationDynamicState-Array",
    name_hash: 526181280,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SimulationDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SimulationStaticState {
}

pub trait SimulationStaticStateTrait: TypeObject {
}

impl SimulationStaticStateTrait for SimulationStaticState {
}

pub static SIMULATIONSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationStaticState",
    name_hash: 1535211801,
    flags: MemberInfoFlags::new(36937),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimulationStaticState as Default>::default())),
            create_boxed: || Box::new(<SimulationStaticState as Default>::default()),
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


pub static SIMULATIONSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimulationStaticState-Array",
    name_hash: 2977158829,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SimulationStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothWrappingAsset {
    pub _glacier_base: super::core::Asset,
    pub cloth_wrapping_asset_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait ClothWrappingAssetTrait: super::core::AssetTrait {
    fn cloth_wrapping_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn cloth_wrapping_asset_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl ClothWrappingAssetTrait for ClothWrappingAsset {
    fn cloth_wrapping_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.cloth_wrapping_asset_resource
    }
    fn cloth_wrapping_asset_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.cloth_wrapping_asset_resource
    }
}

impl super::core::AssetTrait for ClothWrappingAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ClothWrappingAsset {
}

pub static CLOTHWRAPPINGASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothWrappingAsset",
    name_hash: 728541805,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ClothWrappingAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothWrappingAsset as Default>::default())),
            create_boxed: || Box::new(<ClothWrappingAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClothWrappingAssetResource",
                name_hash: 2648747175,
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


pub static CLOTHWRAPPINGASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothWrappingAsset-Array",
    name_hash: 813166425,
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
    name_hash: 2191243761,
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


pub static CLOTHPROCESSINGMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothProcessingMode-Array",
    name_hash: 2171100357,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothProcessingMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct BoxClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub half_lengths: super::core::Vec3,
}

pub trait BoxClothCollisionTrait: ClothCollisionGeometryTrait {
    fn half_lengths(&self) -> &super::core::Vec3;
    fn half_lengths_mut(&mut self) -> &mut super::core::Vec3;
}

impl BoxClothCollisionTrait for BoxClothCollision {
    fn half_lengths(&self) -> &super::core::Vec3 {
        &self.half_lengths
    }
    fn half_lengths_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.half_lengths
    }
}

impl ClothCollisionGeometryTrait for BoxClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::core::DataContainerTrait for BoxClothCollision {
}

pub static BOXCLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxClothCollision",
    name_hash: 2924046450,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        super_class_offset: offset_of!(BoxClothCollision, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoxClothCollision as Default>::default())),
            create_boxed: || Box::new(<BoxClothCollision as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "HalfLengths",
                name_hash: 3036559145,
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


pub static BOXCLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoxClothCollision-Array",
    name_hash: 1073717062,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("BoxClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct TaperedCapsuleClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub radius_a: f32,
    pub radius_b: f32,
    pub half_length: f32,
}

pub trait TaperedCapsuleClothCollisionTrait: ClothCollisionGeometryTrait {
    fn radius_a(&self) -> &f32;
    fn radius_a_mut(&mut self) -> &mut f32;
    fn radius_b(&self) -> &f32;
    fn radius_b_mut(&mut self) -> &mut f32;
    fn half_length(&self) -> &f32;
    fn half_length_mut(&mut self) -> &mut f32;
}

impl TaperedCapsuleClothCollisionTrait for TaperedCapsuleClothCollision {
    fn radius_a(&self) -> &f32 {
        &self.radius_a
    }
    fn radius_a_mut(&mut self) -> &mut f32 {
        &mut self.radius_a
    }
    fn radius_b(&self) -> &f32 {
        &self.radius_b
    }
    fn radius_b_mut(&mut self) -> &mut f32 {
        &mut self.radius_b
    }
    fn half_length(&self) -> &f32 {
        &self.half_length
    }
    fn half_length_mut(&mut self) -> &mut f32 {
        &mut self.half_length
    }
}

impl ClothCollisionGeometryTrait for TaperedCapsuleClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::core::DataContainerTrait for TaperedCapsuleClothCollision {
}

pub static TAPEREDCAPSULECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TaperedCapsuleClothCollision",
    name_hash: 3017347561,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        super_class_offset: offset_of!(TaperedCapsuleClothCollision, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TaperedCapsuleClothCollision as Default>::default())),
            create_boxed: || Box::new(<TaperedCapsuleClothCollision as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "RadiusA",
                name_hash: 1473252924,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TaperedCapsuleClothCollision, radius_a),
            },
            FieldInfoData {
                name: "RadiusB",
                name_hash: 1473252927,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TaperedCapsuleClothCollision, radius_b),
            },
            FieldInfoData {
                name: "HalfLength",
                name_hash: 872920090,
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


pub static TAPEREDCAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TaperedCapsuleClothCollision-Array",
    name_hash: 1681148125,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("TaperedCapsuleClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CapsuleClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub radius: f32,
    pub half_length: f32,
}

pub trait CapsuleClothCollisionTrait: ClothCollisionGeometryTrait {
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn half_length(&self) -> &f32;
    fn half_length_mut(&mut self) -> &mut f32;
}

impl CapsuleClothCollisionTrait for CapsuleClothCollision {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn half_length(&self) -> &f32 {
        &self.half_length
    }
    fn half_length_mut(&mut self) -> &mut f32 {
        &mut self.half_length
    }
}

impl ClothCollisionGeometryTrait for CapsuleClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::core::DataContainerTrait for CapsuleClothCollision {
}

pub static CAPSULECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CapsuleClothCollision",
    name_hash: 2853418842,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        super_class_offset: offset_of!(CapsuleClothCollision, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CapsuleClothCollision as Default>::default())),
            create_boxed: || Box::new(<CapsuleClothCollision as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CapsuleClothCollision, radius),
            },
            FieldInfoData {
                name: "HalfLength",
                name_hash: 872920090,
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


pub static CAPSULECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CapsuleClothCollision-Array",
    name_hash: 3858590446,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("CapsuleClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct SphereClothCollision {
    pub _glacier_base: ClothCollisionGeometry,
    pub radius: f32,
}

pub trait SphereClothCollisionTrait: ClothCollisionGeometryTrait {
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
}

impl SphereClothCollisionTrait for SphereClothCollision {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
}

impl ClothCollisionGeometryTrait for SphereClothCollision {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::core::DataContainerTrait for SphereClothCollision {
}

pub static SPHERECLOTHCOLLISION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereClothCollision",
    name_hash: 3609985982,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLOTHCOLLISIONGEOMETRY_TYPE_INFO),
        super_class_offset: offset_of!(SphereClothCollision, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SphereClothCollision as Default>::default())),
            create_boxed: || Box::new(<SphereClothCollision as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Radius",
                name_hash: 3298407133,
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


pub static SPHERECLOTHCOLLISION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SphereClothCollision-Array",
    name_hash: 952693770,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("SphereClothCollision"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothCollisionGeometry {
    pub _glacier_base: super::core::DataContainer,
    pub transform: super::core::LinearTransform,
}

pub trait ClothCollisionGeometryTrait: super::core::DataContainerTrait {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl ClothCollisionGeometryTrait for ClothCollisionGeometry {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
}

impl super::core::DataContainerTrait for ClothCollisionGeometry {
}

pub static CLOTHCOLLISIONGEOMETRY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionGeometry",
    name_hash: 433382653,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ClothCollisionGeometry, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothCollisionGeometry as Default>::default())),
            create_boxed: || Box::new(<ClothCollisionGeometry as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                name_hash: 2270319721,
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


pub static CLOTHCOLLISIONGEOMETRY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothCollisionGeometry-Array",
    name_hash: 1263849929,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothCollisionGeometry"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothBaseAsset {
    pub _glacier_base: super::core::Asset,
    pub cloth_asset_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait ClothBaseAssetTrait: super::core::AssetTrait {
    fn cloth_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn cloth_asset_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl ClothBaseAssetTrait for ClothBaseAsset {
    fn cloth_asset_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.cloth_asset_resource
    }
    fn cloth_asset_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.cloth_asset_resource
    }
}

impl super::core::AssetTrait for ClothBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ClothBaseAsset {
}

pub static CLOTHBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothBaseAsset",
    name_hash: 3602043068,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ClothBaseAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothBaseAsset as Default>::default())),
            create_boxed: || Box::new(<ClothBaseAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ClothAssetResource",
                name_hash: 3968496963,
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


pub static CLOTHBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothBaseAsset-Array",
    name_hash: 222103560,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClothControlDynamicState {
    pub user_colliders: Vec<Option<LockedTypeObject /* ClothCollisionGeometry */>>,
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
    fn user_colliders(&self) -> &Vec<Option<LockedTypeObject /* ClothCollisionGeometry */>>;
    fn user_colliders_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ClothCollisionGeometry */>>;
    fn user_collider_joint_ids(&self) -> &Vec<u32>;
    fn user_collider_joint_ids_mut(&mut self) -> &mut Vec<u32>;
    fn authored_colliders_enabled(&self) -> &bool;
    fn authored_colliders_enabled_mut(&mut self) -> &mut bool;
    fn root_transform(&self) -> &super::core::LinearTransform;
    fn root_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn ground_plane_transform(&self) -> &super::core::LinearTransform;
    fn ground_plane_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn next_processing_mode(&self) -> &ClothProcessingMode;
    fn next_processing_mode_mut(&mut self) -> &mut ClothProcessingMode;
    fn trigger_re_initialize(&self) -> &bool;
    fn trigger_re_initialize_mut(&mut self) -> &mut bool;
    fn processing_mode_overridden(&self) -> &bool;
    fn processing_mode_overridden_mut(&mut self) -> &mut bool;
    fn enable_ground_plane(&self) -> &bool;
    fn enable_ground_plane_mut(&mut self) -> &mut bool;
    fn use_authored_ground_plane(&self) -> &bool;
    fn use_authored_ground_plane_mut(&mut self) -> &mut bool;
    fn force_skip_interpolation(&self) -> &bool;
    fn force_skip_interpolation_mut(&mut self) -> &mut bool;
    fn trigger_retarget(&self) -> &bool;
    fn trigger_retarget_mut(&mut self) -> &mut bool;
    fn trigger_teleport(&self) -> &bool;
    fn trigger_teleport_mut(&mut self) -> &mut bool;
    fn root_motion_contribution_enabled(&self) -> &bool;
    fn root_motion_contribution_enabled_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl ClothControlDynamicStateTrait for ClothControlDynamicState {
    fn user_colliders(&self) -> &Vec<Option<LockedTypeObject /* ClothCollisionGeometry */>> {
        &self.user_colliders
    }
    fn user_colliders_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* ClothCollisionGeometry */>> {
        &mut self.user_colliders
    }
    fn user_collider_joint_ids(&self) -> &Vec<u32> {
        &self.user_collider_joint_ids
    }
    fn user_collider_joint_ids_mut(&mut self) -> &mut Vec<u32> {
        &mut self.user_collider_joint_ids
    }
    fn authored_colliders_enabled(&self) -> &bool {
        &self.authored_colliders_enabled
    }
    fn authored_colliders_enabled_mut(&mut self) -> &mut bool {
        &mut self.authored_colliders_enabled
    }
    fn root_transform(&self) -> &super::core::LinearTransform {
        &self.root_transform
    }
    fn root_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.root_transform
    }
    fn ground_plane_transform(&self) -> &super::core::LinearTransform {
        &self.ground_plane_transform
    }
    fn ground_plane_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.ground_plane_transform
    }
    fn next_processing_mode(&self) -> &ClothProcessingMode {
        &self.next_processing_mode
    }
    fn next_processing_mode_mut(&mut self) -> &mut ClothProcessingMode {
        &mut self.next_processing_mode
    }
    fn trigger_re_initialize(&self) -> &bool {
        &self.trigger_re_initialize
    }
    fn trigger_re_initialize_mut(&mut self) -> &mut bool {
        &mut self.trigger_re_initialize
    }
    fn processing_mode_overridden(&self) -> &bool {
        &self.processing_mode_overridden
    }
    fn processing_mode_overridden_mut(&mut self) -> &mut bool {
        &mut self.processing_mode_overridden
    }
    fn enable_ground_plane(&self) -> &bool {
        &self.enable_ground_plane
    }
    fn enable_ground_plane_mut(&mut self) -> &mut bool {
        &mut self.enable_ground_plane
    }
    fn use_authored_ground_plane(&self) -> &bool {
        &self.use_authored_ground_plane
    }
    fn use_authored_ground_plane_mut(&mut self) -> &mut bool {
        &mut self.use_authored_ground_plane
    }
    fn force_skip_interpolation(&self) -> &bool {
        &self.force_skip_interpolation
    }
    fn force_skip_interpolation_mut(&mut self) -> &mut bool {
        &mut self.force_skip_interpolation
    }
    fn trigger_retarget(&self) -> &bool {
        &self.trigger_retarget
    }
    fn trigger_retarget_mut(&mut self) -> &mut bool {
        &mut self.trigger_retarget
    }
    fn trigger_teleport(&self) -> &bool {
        &self.trigger_teleport
    }
    fn trigger_teleport_mut(&mut self) -> &mut bool {
        &mut self.trigger_teleport
    }
    fn root_motion_contribution_enabled(&self) -> &bool {
        &self.root_motion_contribution_enabled
    }
    fn root_motion_contribution_enabled_mut(&mut self) -> &mut bool {
        &mut self.root_motion_contribution_enabled
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static CLOTHCONTROLDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlDynamicState",
    name_hash: 42272796,
    flags: MemberInfoFlags::new(73),
    module: "ClothBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClothControlDynamicState as Default>::default())),
            create_boxed: || Box::new(<ClothControlDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "UserColliders",
                name_hash: 654421137,
                flags: MemberInfoFlags::new(144),
                field_type: "ClothCollisionGeometry-Array",
                rust_offset: offset_of!(ClothControlDynamicState, user_colliders),
            },
            FieldInfoData {
                name: "UserColliderJointIds",
                name_hash: 352698826,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(ClothControlDynamicState, user_collider_joint_ids),
            },
            FieldInfoData {
                name: "AuthoredCollidersEnabled",
                name_hash: 610225969,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, authored_colliders_enabled),
            },
            FieldInfoData {
                name: "RootTransform",
                name_hash: 3407367919,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothControlDynamicState, root_transform),
            },
            FieldInfoData {
                name: "GroundPlaneTransform",
                name_hash: 790189114,
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ClothControlDynamicState, ground_plane_transform),
            },
            FieldInfoData {
                name: "NextProcessingMode",
                name_hash: 1790214442,
                flags: MemberInfoFlags::new(0),
                field_type: "ClothProcessingMode",
                rust_offset: offset_of!(ClothControlDynamicState, next_processing_mode),
            },
            FieldInfoData {
                name: "TriggerReInitialize",
                name_hash: 1116291522,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, trigger_re_initialize),
            },
            FieldInfoData {
                name: "ProcessingModeOverridden",
                name_hash: 2263740627,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, processing_mode_overridden),
            },
            FieldInfoData {
                name: "EnableGroundPlane",
                name_hash: 3255199031,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, enable_ground_plane),
            },
            FieldInfoData {
                name: "UseAuthoredGroundPlane",
                name_hash: 3693128321,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, use_authored_ground_plane),
            },
            FieldInfoData {
                name: "ForceSkipInterpolation",
                name_hash: 2805112179,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, force_skip_interpolation),
            },
            FieldInfoData {
                name: "TriggerRetarget",
                name_hash: 2305518779,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, trigger_retarget),
            },
            FieldInfoData {
                name: "TriggerTeleport",
                name_hash: 2744322076,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, trigger_teleport),
            },
            FieldInfoData {
                name: "RootMotionContributionEnabled",
                name_hash: 855088926,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ClothControlDynamicState, root_motion_contribution_enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static CLOTHCONTROLDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClothControlDynamicState-Array",
    name_hash: 2082177064,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("ClothControlDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EAClothEntityData {
}

pub trait EAClothEntityDataTrait: TypeObject {
}

impl EAClothEntityDataTrait for EAClothEntityData {
}

pub static EACLOTHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothEntityData",
    name_hash: 1128671030,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EAClothEntityData as Default>::default())),
            create_boxed: || Box::new(<EAClothEntityData as Default>::default()),
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


pub static EACLOTHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothEntityData-Array",
    name_hash: 1095785346,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("EAClothEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct EAClothAssetData {
}

pub trait EAClothAssetDataTrait: TypeObject {
}

impl EAClothAssetDataTrait for EAClothAssetData {
}

pub static EACLOTHASSETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothAssetData",
    name_hash: 1866863773,
    flags: MemberInfoFlags::new(101),
    module: "ClothBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EAClothAssetData as Default>::default())),
            create_boxed: || Box::new(<EAClothAssetData as Default>::default()),
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


pub static EACLOTHASSETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EAClothAssetData-Array",
    name_hash: 2841559081,
    flags: MemberInfoFlags::new(145),
    module: "ClothBase",
    data: TypeInfoData::Array("EAClothAssetData"),
    array_type: None,
    alignment: 8,
};


