use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_terrain_sim_types(registry: &mut TypeRegistry) {
    registry.register_type(WATERASSET_TYPE_INFO);
    registry.register_type(WATERASSET_ARRAY_TYPE_INFO);
    registry.register_type(PHYSICSTERRAINUPDATERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(PHYSICSTERRAINUPDATERCOMPONENTDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct WaterAsset {
    pub _glacier_base: super::physics::PhysicsResourceContainerAsset,
}

pub trait WaterAssetTrait: super::physics::PhysicsResourceContainerAssetTrait {
}

impl WaterAssetTrait for WaterAsset {
}

impl super::physics::PhysicsResourceContainerAssetTrait for WaterAsset {
    fn physics_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        self._glacier_base.physics_resource()
    }
    fn physics_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        self._glacier_base.physics_resource_mut()
    }
}

impl super::core::AssetTrait for WaterAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for WaterAsset {
}

pub static WATERASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAsset",
    flags: MemberInfoFlags::new(101),
    module: "TerrainSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::physics::PHYSICSRESOURCECONTAINERASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WaterAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WATERASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WaterAsset {
    fn type_info(&self) -> &'static TypeInfo {
        WATERASSET_TYPE_INFO
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


pub static WATERASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WaterAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainSim",
    data: TypeInfoData::Array("WaterAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PhysicsTerrainUpdaterComponentData {
    pub _glacier_base: super::entity::ComponentData,
    pub in_categories: super::physics::PhysicsCategorySet,
    pub collides_with_categories: super::physics::PhysicsCategorySet,
}

pub trait PhysicsTerrainUpdaterComponentDataTrait: super::entity::ComponentDataTrait {
    fn in_categories(&self) -> &super::physics::PhysicsCategorySet;
    fn in_categories_mut(&mut self) -> &mut super::physics::PhysicsCategorySet;
    fn collides_with_categories(&self) -> &super::physics::PhysicsCategorySet;
    fn collides_with_categories_mut(&mut self) -> &mut super::physics::PhysicsCategorySet;
}

impl PhysicsTerrainUpdaterComponentDataTrait for PhysicsTerrainUpdaterComponentData {
    fn in_categories(&self) -> &super::physics::PhysicsCategorySet {
        &self.in_categories
    }
    fn in_categories_mut(&mut self) -> &mut super::physics::PhysicsCategorySet {
        &mut self.in_categories
    }
    fn collides_with_categories(&self) -> &super::physics::PhysicsCategorySet {
        &self.collides_with_categories
    }
    fn collides_with_categories_mut(&mut self) -> &mut super::physics::PhysicsCategorySet {
        &mut self.collides_with_categories
    }
}

impl super::entity::ComponentDataTrait for PhysicsTerrainUpdaterComponentData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn client_index(&self) -> &u8 {
        self._glacier_base.client_index()
    }
    fn client_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.client_index_mut()
    }
    fn server_index(&self) -> &u8 {
        self._glacier_base.server_index()
    }
    fn server_index_mut(&mut self) -> &mut u8 {
        self._glacier_base.server_index_mut()
    }
    fn excluded(&self) -> &bool {
        self._glacier_base.excluded()
    }
    fn excluded_mut(&mut self) -> &mut bool {
        self._glacier_base.excluded_mut()
    }
}

impl super::entity::GameObjectDataTrait for PhysicsTerrainUpdaterComponentData {
}

impl super::core::DataBusPeerTrait for PhysicsTerrainUpdaterComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PhysicsTerrainUpdaterComponentData {
}

impl super::core::DataContainerTrait for PhysicsTerrainUpdaterComponentData {
}

pub static PHYSICSTERRAINUPDATERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTerrainUpdaterComponentData",
    flags: MemberInfoFlags::new(101),
    module: "TerrainSim",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::COMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PhysicsTerrainUpdaterComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InCategories",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsCategorySet",
                rust_offset: offset_of!(PhysicsTerrainUpdaterComponentData, in_categories),
            },
            FieldInfoData {
                name: "CollidesWithCategories",
                flags: MemberInfoFlags::new(0),
                field_type: "PhysicsCategorySet",
                rust_offset: offset_of!(PhysicsTerrainUpdaterComponentData, collides_with_categories),
            },
        ],
    }),
    array_type: Some(PHYSICSTERRAINUPDATERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PhysicsTerrainUpdaterComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        PHYSICSTERRAINUPDATERCOMPONENTDATA_TYPE_INFO
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


pub static PHYSICSTERRAINUPDATERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PhysicsTerrainUpdaterComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "TerrainSim",
    data: TypeInfoData::Array("PhysicsTerrainUpdaterComponentData"),
    array_type: None,
    alignment: 8,
};


