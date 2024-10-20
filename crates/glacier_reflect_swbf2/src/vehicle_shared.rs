use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_vehicle_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(FLAPCOMPONENTDATA_TYPE_INFO);
    registry.register_type(FLAPCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WINGCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WINGCOMPONENTDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct FlapComponentData {
    pub rotation_axis: super::game_shared::RotationAxisEnum,
    pub rotation_scale: f32,
}

pub const FLAPCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlapComponentData",
    flags: MemberInfoFlags::new(101),
    module: "VehicleShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BONECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: ROTATIONAXISENUM_TYPE_INFO,
                rust_offset: offset_of!(FlapComponentData, rotation_axis),
            },
            FieldInfoData {
                name: "RotationScale",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(FlapComponentData, rotation_scale),
            },
        ],
    }),
    array_type: Some(FLAPCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FlapComponentData {
    fn type_info() -> &'static TypeInfo {
        FLAPCOMPONENTDATA_TYPE_INFO
    }
}


pub const FLAPCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlapComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "VehicleShared",
    data: TypeInfoData::Array("FlapComponentData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct WingComponentData {
    pub config: super::physics::WingPhysicsData,
}

pub const WINGCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingComponentData",
    flags: MemberInfoFlags::new(101),
    module: "VehicleShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BONECOMPONENTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Config",
                flags: MemberInfoFlags::new(0),
                field_type: WINGPHYSICSDATA_TYPE_INFO,
                rust_offset: offset_of!(WingComponentData, config),
            },
        ],
    }),
    array_type: Some(WINGCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WingComponentData {
    fn type_info() -> &'static TypeInfo {
        WINGCOMPONENTDATA_TYPE_INFO
    }
}


pub const WINGCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "VehicleShared",
    data: TypeInfoData::Array("WingComponentData-Array"),
    array_type: None,
    alignment: 8,
};


