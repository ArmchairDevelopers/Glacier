use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_vehicle_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(FLAPCOMPONENTDATA_TYPE_INFO);
    registry.register_type(FLAPCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(WINGCOMPONENTDATA_TYPE_INFO);
    registry.register_type(WINGCOMPONENTDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct FlapComponentData {
    pub _glacier_base: super::entity::BoneComponentData,
    pub rotation_axis: super::game_shared::RotationAxisEnum,
    pub rotation_scale: f32,
}

pub trait FlapComponentDataTrait: super::entity::BoneComponentDataTrait {
    fn rotation_axis(&self) -> &super::game_shared::RotationAxisEnum;
    fn rotation_scale(&self) -> &f32;
}

impl FlapComponentDataTrait for FlapComponentData {
    fn rotation_axis(&self) -> &super::game_shared::RotationAxisEnum {
        &self.rotation_axis
    }
    fn rotation_scale(&self) -> &f32 {
        &self.rotation_scale
    }
}

impl super::entity::BoneComponentDataTrait for FlapComponentData {
}

impl super::entity::GameComponentDataTrait for FlapComponentData {
}

impl super::entity::ComponentDataTrait for FlapComponentData {
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

impl super::entity::GameObjectDataTrait for FlapComponentData {
}

impl super::core::DataBusPeerTrait for FlapComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for FlapComponentData {
}

impl super::core::DataContainerTrait for FlapComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FLAPCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlapComponentData",
    flags: MemberInfoFlags::new(101),
    module: "VehicleShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::BONECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FlapComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RotationAxis",
                flags: MemberInfoFlags::new(0),
                field_type: "RotationAxisEnum",
                rust_offset: offset_of!(FlapComponentData, rotation_axis),
            },
            FieldInfoData {
                name: "RotationScale",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FlapComponentData, rotation_scale),
            },
        ],
    }),
    array_type: Some(FLAPCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FlapComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        FLAPCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLAPCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FlapComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "VehicleShared",
    data: TypeInfoData::Array("FlapComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WingComponentData {
    pub _glacier_base: super::entity::BoneComponentData,
    pub config: Option<Arc<Mutex<dyn super::physics::WingPhysicsDataTrait>>>,
}

pub trait WingComponentDataTrait: super::entity::BoneComponentDataTrait {
    fn config(&self) -> &Option<Arc<Mutex<dyn super::physics::WingPhysicsDataTrait>>>;
}

impl WingComponentDataTrait for WingComponentData {
    fn config(&self) -> &Option<Arc<Mutex<dyn super::physics::WingPhysicsDataTrait>>> {
        &self.config
    }
}

impl super::entity::BoneComponentDataTrait for WingComponentData {
}

impl super::entity::GameComponentDataTrait for WingComponentData {
}

impl super::entity::ComponentDataTrait for WingComponentData {
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

impl super::entity::GameObjectDataTrait for WingComponentData {
}

impl super::core::DataBusPeerTrait for WingComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for WingComponentData {
}

impl super::core::DataContainerTrait for WingComponentData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WINGCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingComponentData",
    flags: MemberInfoFlags::new(101),
    module: "VehicleShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::BONECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WingComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Config",
                flags: MemberInfoFlags::new(0),
                field_type: "WingPhysicsData",
                rust_offset: offset_of!(WingComponentData, config),
            },
        ],
    }),
    array_type: Some(WINGCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for WingComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        WINGCOMPONENTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static WINGCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WingComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "VehicleShared",
    data: TypeInfoData::Array("WingComponentData"),
    array_type: None,
    alignment: 8,
};


