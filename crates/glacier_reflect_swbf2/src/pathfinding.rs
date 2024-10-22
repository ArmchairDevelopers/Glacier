use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_pathfinding_types(registry: &mut TypeRegistry) {
    registry.register_type(OBSTACLECONTROLLERENTITYDATA_TYPE_INFO);
    registry.register_type(OBSTACLECONTROLLERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGNAVMESHVOLUMEDATA_TYPE_INFO);
    registry.register_type(PATHFINDINGNAVMESHVOLUMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(OBSTACLEDAT_TYPE_INFO);
    registry.register_type(OBSTACLEDAT_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMOBSTACLEDATA_TYPE_INFO);
    registry.register_type(CUSTOMOBSTACLEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHLINKENTITYDATA_TYPE_INFO);
    registry.register_type(PATHLINKENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PATHLINKDIRECTION_TYPE_INFO);
    registry.register_type(PATHLINKDIRECTION_ARRAY_TYPE_INFO);
    registry.register_type(MOVERFOLLOWLEADERENTITYDATA_TYPE_INFO);
    registry.register_type(MOVERFOLLOWLEADERENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVERFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO);
    registry.register_type(MOVERFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVERCOMPONENTDATA_TYPE_INFO);
    registry.register_type(MOVERCOMPONENTDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYMOVERTYPE_TYPE_INFO);
    registry.register_type(ENTITYMOVERTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MOVERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(MOVERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(FOLLOWERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNEOVERRIDE_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(IDLETUNEOVERRIDE_TYPE_INFO);
    registry.register_type(IDLETUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(GOALTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(GOALTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(PROBERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(PROBERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(JUMPERTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(JUMPERTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(CAUTIONTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(CAUTIONTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNEOVERRIDE_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(TURNINPLACETUNEOVERRIDE_TYPE_INFO);
    registry.register_type(TURNINPLACETUNEOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(RADIUSDATAOVERRIDE_TYPE_INFO);
    registry.register_type(RADIUSDATAOVERRIDE_ARRAY_TYPE_INFO);
    registry.register_type(MOVERTUNE_TYPE_INFO);
    registry.register_type(MOVERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(PATHCREATIONOPTIONS_TYPE_INFO);
    registry.register_type(PATHCREATIONOPTIONS_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWERTUNE_TYPE_INFO);
    registry.register_type(FOLLOWERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNE_TYPE_INFO);
    registry.register_type(SURFACEORIENTTUNE_ARRAY_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNE_TYPE_INFO);
    registry.register_type(AUTOOBSTACLETUNE_ARRAY_TYPE_INFO);
    registry.register_type(IDLETUNE_TYPE_INFO);
    registry.register_type(IDLETUNE_ARRAY_TYPE_INFO);
    registry.register_type(GOALTUNE_TYPE_INFO);
    registry.register_type(GOALTUNE_ARRAY_TYPE_INFO);
    registry.register_type(PROBERTUNE_TYPE_INFO);
    registry.register_type(PROBERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(JUMPERTUNE_TYPE_INFO);
    registry.register_type(JUMPERTUNE_ARRAY_TYPE_INFO);
    registry.register_type(CAUTIONTUNE_TYPE_INFO);
    registry.register_type(CAUTIONTUNE_ARRAY_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNE_TYPE_INFO);
    registry.register_type(REPULSIONACCELERATIONTUNE_ARRAY_TYPE_INFO);
    registry.register_type(TURNINPLACETUNE_TYPE_INFO);
    registry.register_type(TURNINPLACETUNE_ARRAY_TYPE_INFO);
    registry.register_type(RADIUSDATA_TYPE_INFO);
    registry.register_type(RADIUSDATA_ARRAY_TYPE_INFO);
    registry.register_type(BLOCKAGEMODE_TYPE_INFO);
    registry.register_type(BLOCKAGEMODE_ARRAY_TYPE_INFO);
    registry.register_type(ORIENTMODE_TYPE_INFO);
    registry.register_type(ORIENTMODE_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWMOVERSPEC_TYPE_INFO);
    registry.register_type(FOLLOWMOVERSPEC_ARRAY_TYPE_INFO);
    registry.register_type(FOLLOWFORMATION_TYPE_INFO);
    registry.register_type(FOLLOWFORMATION_ARRAY_TYPE_INFO);
    registry.register_type(STOPSPEC_TYPE_INFO);
    registry.register_type(STOPSPEC_ARRAY_TYPE_INFO);
    registry.register_type(GOTOPOSSPEC_TYPE_INFO);
    registry.register_type(GOTOPOSSPEC_ARRAY_TYPE_INFO);
    registry.register_type(PATHSPEC_TYPE_INFO);
    registry.register_type(PATHSPEC_ARRAY_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCE_TYPE_INFO);
    registry.register_type(PATHFINDINGRUNTIMERESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(SERVEROBSTACLECONTROLLERENTITY_TYPE_INFO);
    registry.register_type(SERVEROBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SERVERNAVPOWERSYSTEMENTITY_TYPE_INFO);
    registry.register_type(SERVERNAVPOWERSYSTEMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBSTACLECONTROLLERENTITY_TYPE_INFO);
    registry.register_type(OBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(NAVPOWERPATHSPEC_TYPE_INFO);
    registry.register_type(NAVPOWERPATHSPEC_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct ObstacleControllerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub active_at_start: bool,
    pub obstacle_data: Option<Arc<Mutex<dyn ObstacleDatTrait>>>,
}

pub trait ObstacleControllerEntityDataTrait: super::entity::EntityDataTrait {
    fn active_at_start(&self) -> &bool;
    fn active_at_start_mut(&mut self) -> &mut bool;
    fn obstacle_data(&self) -> &Option<Arc<Mutex<dyn ObstacleDatTrait>>>;
    fn obstacle_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ObstacleDatTrait>>>;
}

impl ObstacleControllerEntityDataTrait for ObstacleControllerEntityData {
    fn active_at_start(&self) -> &bool {
        &self.active_at_start
    }
    fn active_at_start_mut(&mut self) -> &mut bool {
        &mut self.active_at_start
    }
    fn obstacle_data(&self) -> &Option<Arc<Mutex<dyn ObstacleDatTrait>>> {
        &self.obstacle_data
    }
    fn obstacle_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ObstacleDatTrait>>> {
        &mut self.obstacle_data
    }
}

impl super::entity::EntityDataTrait for ObstacleControllerEntityData {
}

impl super::entity::GameObjectDataTrait for ObstacleControllerEntityData {
}

impl super::core::DataBusPeerTrait for ObstacleControllerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ObstacleControllerEntityData {
}

impl super::core::DataContainerTrait for ObstacleControllerEntityData {
}

pub static OBSTACLECONTROLLERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObstacleControllerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ActiveAtStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ObstacleControllerEntityData, active_at_start),
            },
            FieldInfoData {
                name: "ObstacleData",
                flags: MemberInfoFlags::new(0),
                field_type: "ObstacleDat",
                rust_offset: offset_of!(ObstacleControllerEntityData, obstacle_data),
            },
        ],
    }),
    array_type: Some(OBSTACLECONTROLLERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObstacleControllerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        OBSTACLECONTROLLERENTITYDATA_TYPE_INFO
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


pub static OBSTACLECONTROLLERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleControllerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathfindingNavMeshVolumeData {
    pub _glacier_base: super::entity::OBBData,
    pub category: Option<Arc<Mutex<dyn super::entity::PathfindingObjectCategoryAssetTrait>>>,
}

pub trait PathfindingNavMeshVolumeDataTrait: super::entity::OBBDataTrait {
    fn category(&self) -> &Option<Arc<Mutex<dyn super::entity::PathfindingObjectCategoryAssetTrait>>>;
    fn category_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::PathfindingObjectCategoryAssetTrait>>>;
}

impl PathfindingNavMeshVolumeDataTrait for PathfindingNavMeshVolumeData {
    fn category(&self) -> &Option<Arc<Mutex<dyn super::entity::PathfindingObjectCategoryAssetTrait>>> {
        &self.category
    }
    fn category_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::PathfindingObjectCategoryAssetTrait>>> {
        &mut self.category
    }
}

impl super::entity::OBBDataTrait for PathfindingNavMeshVolumeData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
    fn half_extents(&self) -> &super::core::Vec3 {
        self._glacier_base.half_extents()
    }
    fn half_extents_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.half_extents_mut()
    }
}

impl super::entity::BaseShapeDataTrait for PathfindingNavMeshVolumeData {
}

impl super::entity::BaseShapeDataBaseTrait for PathfindingNavMeshVolumeData {
}

impl super::entity::GameObjectDataTrait for PathfindingNavMeshVolumeData {
}

impl super::core::DataBusPeerTrait for PathfindingNavMeshVolumeData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PathfindingNavMeshVolumeData {
}

impl super::core::DataContainerTrait for PathfindingNavMeshVolumeData {
}

pub static PATHFINDINGNAVMESHVOLUMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingNavMeshVolumeData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBBDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingNavMeshVolumeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Category",
                flags: MemberInfoFlags::new(0),
                field_type: "PathfindingObjectCategoryAsset",
                rust_offset: offset_of!(PathfindingNavMeshVolumeData, category),
            },
        ],
    }),
    array_type: Some(PATHFINDINGNAVMESHVOLUMEDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PathfindingNavMeshVolumeData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGNAVMESHVOLUMEDATA_TYPE_INFO
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


pub static PATHFINDINGNAVMESHVOLUMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingNavMeshVolumeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathfindingNavMeshVolumeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObstacleDat {
    pub _glacier_base: super::core::DataContainer,
    pub layer_mask: u32,
    pub penalty_mult: f32,
    pub obstacle_blockage_flags: u32,
    pub user_data: Option<Arc<Mutex<dyn CustomObstacleDataTrait>>>,
    pub obstacle_name: String,
}

pub trait ObstacleDatTrait: super::core::DataContainerTrait {
    fn layer_mask(&self) -> &u32;
    fn layer_mask_mut(&mut self) -> &mut u32;
    fn penalty_mult(&self) -> &f32;
    fn penalty_mult_mut(&mut self) -> &mut f32;
    fn obstacle_blockage_flags(&self) -> &u32;
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32;
    fn user_data(&self) -> &Option<Arc<Mutex<dyn CustomObstacleDataTrait>>>;
    fn user_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CustomObstacleDataTrait>>>;
    fn obstacle_name(&self) -> &String;
    fn obstacle_name_mut(&mut self) -> &mut String;
}

impl ObstacleDatTrait for ObstacleDat {
    fn layer_mask(&self) -> &u32 {
        &self.layer_mask
    }
    fn layer_mask_mut(&mut self) -> &mut u32 {
        &mut self.layer_mask
    }
    fn penalty_mult(&self) -> &f32 {
        &self.penalty_mult
    }
    fn penalty_mult_mut(&mut self) -> &mut f32 {
        &mut self.penalty_mult
    }
    fn obstacle_blockage_flags(&self) -> &u32 {
        &self.obstacle_blockage_flags
    }
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32 {
        &mut self.obstacle_blockage_flags
    }
    fn user_data(&self) -> &Option<Arc<Mutex<dyn CustomObstacleDataTrait>>> {
        &self.user_data
    }
    fn user_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CustomObstacleDataTrait>>> {
        &mut self.user_data
    }
    fn obstacle_name(&self) -> &String {
        &self.obstacle_name
    }
    fn obstacle_name_mut(&mut self) -> &mut String {
        &mut self.obstacle_name
    }
}

impl super::core::DataContainerTrait for ObstacleDat {
}

pub static OBSTACLEDAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleDat",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObstacleDat as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LayerMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ObstacleDat, layer_mask),
            },
            FieldInfoData {
                name: "PenaltyMult",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObstacleDat, penalty_mult),
            },
            FieldInfoData {
                name: "ObstacleBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ObstacleDat, obstacle_blockage_flags),
            },
            FieldInfoData {
                name: "UserData",
                flags: MemberInfoFlags::new(0),
                field_type: "CustomObstacleData",
                rust_offset: offset_of!(ObstacleDat, user_data),
            },
            FieldInfoData {
                name: "ObstacleName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ObstacleDat, obstacle_name),
            },
        ],
    }),
    array_type: Some(OBSTACLEDAT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObstacleDat {
    fn type_info(&self) -> &'static TypeInfo {
        OBSTACLEDAT_TYPE_INFO
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


pub static OBSTACLEDAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleDat-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleDat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CustomObstacleData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait CustomObstacleDataTrait: super::core::DataContainerTrait {
}

impl CustomObstacleDataTrait for CustomObstacleData {
}

impl super::core::DataContainerTrait for CustomObstacleData {
}

pub static CUSTOMOBSTACLEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomObstacleData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomObstacleData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CUSTOMOBSTACLEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomObstacleData {
    fn type_info(&self) -> &'static TypeInfo {
        CUSTOMOBSTACLEDATA_TYPE_INFO
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


pub static CUSTOMOBSTACLEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomObstacleData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CustomObstacleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathLinkEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub points: Vec<super::core::Vec3>,
    pub direction: PathLinkDirection,
    pub link_dat: Option<Arc<Mutex<dyn super::pathfinding_shared::LinkDatTrait>>>,
    pub active_at_start: bool,
    pub deferred_creation: bool,
}

pub trait PathLinkEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn points(&self) -> &Vec<super::core::Vec3>;
    fn points_mut(&mut self) -> &mut Vec<super::core::Vec3>;
    fn direction(&self) -> &PathLinkDirection;
    fn direction_mut(&mut self) -> &mut PathLinkDirection;
    fn link_dat(&self) -> &Option<Arc<Mutex<dyn super::pathfinding_shared::LinkDatTrait>>>;
    fn link_dat_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::pathfinding_shared::LinkDatTrait>>>;
    fn active_at_start(&self) -> &bool;
    fn active_at_start_mut(&mut self) -> &mut bool;
    fn deferred_creation(&self) -> &bool;
    fn deferred_creation_mut(&mut self) -> &mut bool;
}

impl PathLinkEntityDataTrait for PathLinkEntityData {
    fn points(&self) -> &Vec<super::core::Vec3> {
        &self.points
    }
    fn points_mut(&mut self) -> &mut Vec<super::core::Vec3> {
        &mut self.points
    }
    fn direction(&self) -> &PathLinkDirection {
        &self.direction
    }
    fn direction_mut(&mut self) -> &mut PathLinkDirection {
        &mut self.direction
    }
    fn link_dat(&self) -> &Option<Arc<Mutex<dyn super::pathfinding_shared::LinkDatTrait>>> {
        &self.link_dat
    }
    fn link_dat_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::pathfinding_shared::LinkDatTrait>>> {
        &mut self.link_dat
    }
    fn active_at_start(&self) -> &bool {
        &self.active_at_start
    }
    fn active_at_start_mut(&mut self) -> &mut bool {
        &mut self.active_at_start
    }
    fn deferred_creation(&self) -> &bool {
        &self.deferred_creation
    }
    fn deferred_creation_mut(&mut self) -> &mut bool {
        &mut self.deferred_creation
    }
}

impl super::entity::SpatialEntityDataTrait for PathLinkEntityData {
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for PathLinkEntityData {
}

impl super::entity::GameObjectDataTrait for PathLinkEntityData {
}

impl super::core::DataBusPeerTrait for PathLinkEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PathLinkEntityData {
}

impl super::core::DataContainerTrait for PathLinkEntityData {
}

pub static PATHLINKENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathLinkEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(PathLinkEntityData, points),
            },
            FieldInfoData {
                name: "Direction",
                flags: MemberInfoFlags::new(0),
                field_type: "PathLinkDirection",
                rust_offset: offset_of!(PathLinkEntityData, direction),
            },
            FieldInfoData {
                name: "LinkDat",
                flags: MemberInfoFlags::new(0),
                field_type: "LinkDat",
                rust_offset: offset_of!(PathLinkEntityData, link_dat),
            },
            FieldInfoData {
                name: "ActiveAtStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathLinkEntityData, active_at_start),
            },
            FieldInfoData {
                name: "DeferredCreation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathLinkEntityData, deferred_creation),
            },
        ],
    }),
    array_type: Some(PATHLINKENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for PathLinkEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PATHLINKENTITYDATA_TYPE_INFO
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


pub static PATHLINKENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathLinkEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PathLinkDirection {
    #[default]
    PathLinkDirection_Forward = 0,
    PathLinkDirection_Backward = 1,
    PathLinkDirection_Both = 2,
}

pub static PATHLINKDIRECTION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkDirection",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(PATHLINKDIRECTION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PathLinkDirection {
    fn type_info(&self) -> &'static TypeInfo {
        PATHLINKDIRECTION_TYPE_INFO
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


pub static PATHLINKDIRECTION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathLinkDirection-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathLinkDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoverFollowLeaderEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub following_parameters: FollowMoverSpec,
    pub flock_id: u32,
}

pub trait MoverFollowLeaderEntityDataTrait: super::entity::EntityDataTrait {
    fn following_parameters(&self) -> &FollowMoverSpec;
    fn following_parameters_mut(&mut self) -> &mut FollowMoverSpec;
    fn flock_id(&self) -> &u32;
    fn flock_id_mut(&mut self) -> &mut u32;
}

impl MoverFollowLeaderEntityDataTrait for MoverFollowLeaderEntityData {
    fn following_parameters(&self) -> &FollowMoverSpec {
        &self.following_parameters
    }
    fn following_parameters_mut(&mut self) -> &mut FollowMoverSpec {
        &mut self.following_parameters
    }
    fn flock_id(&self) -> &u32 {
        &self.flock_id
    }
    fn flock_id_mut(&mut self) -> &mut u32 {
        &mut self.flock_id
    }
}

impl super::entity::EntityDataTrait for MoverFollowLeaderEntityData {
}

impl super::entity::GameObjectDataTrait for MoverFollowLeaderEntityData {
}

impl super::core::DataBusPeerTrait for MoverFollowLeaderEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MoverFollowLeaderEntityData {
}

impl super::core::DataContainerTrait for MoverFollowLeaderEntityData {
}

pub static MOVERFOLLOWLEADERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowLeaderEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverFollowLeaderEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FollowingParameters",
                flags: MemberInfoFlags::new(0),
                field_type: "FollowMoverSpec",
                rust_offset: offset_of!(MoverFollowLeaderEntityData, following_parameters),
            },
            FieldInfoData {
                name: "FlockId",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverFollowLeaderEntityData, flock_id),
            },
        ],
    }),
    array_type: Some(MOVERFOLLOWLEADERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverFollowLeaderEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVERFOLLOWLEADERENTITYDATA_TYPE_INFO
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


pub static MOVERFOLLOWLEADERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowLeaderEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverFollowLeaderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoverFollowWaypointsEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub type_of_route: super::pathfinding_shared::RouteType,
    pub stop_at_waypoints: bool,
    pub start_at_geometrically_closest_waypoint: bool,
    pub intermediate_allowed_stop_dist_override: f32,
    pub destination_allowed_stop_dist_override: f32,
    pub destination_set_orientation: bool,
}

pub trait MoverFollowWaypointsEntityDataTrait: super::entity::EntityDataTrait {
    fn type_of_route(&self) -> &super::pathfinding_shared::RouteType;
    fn type_of_route_mut(&mut self) -> &mut super::pathfinding_shared::RouteType;
    fn stop_at_waypoints(&self) -> &bool;
    fn stop_at_waypoints_mut(&mut self) -> &mut bool;
    fn start_at_geometrically_closest_waypoint(&self) -> &bool;
    fn start_at_geometrically_closest_waypoint_mut(&mut self) -> &mut bool;
    fn intermediate_allowed_stop_dist_override(&self) -> &f32;
    fn intermediate_allowed_stop_dist_override_mut(&mut self) -> &mut f32;
    fn destination_allowed_stop_dist_override(&self) -> &f32;
    fn destination_allowed_stop_dist_override_mut(&mut self) -> &mut f32;
    fn destination_set_orientation(&self) -> &bool;
    fn destination_set_orientation_mut(&mut self) -> &mut bool;
}

impl MoverFollowWaypointsEntityDataTrait for MoverFollowWaypointsEntityData {
    fn type_of_route(&self) -> &super::pathfinding_shared::RouteType {
        &self.type_of_route
    }
    fn type_of_route_mut(&mut self) -> &mut super::pathfinding_shared::RouteType {
        &mut self.type_of_route
    }
    fn stop_at_waypoints(&self) -> &bool {
        &self.stop_at_waypoints
    }
    fn stop_at_waypoints_mut(&mut self) -> &mut bool {
        &mut self.stop_at_waypoints
    }
    fn start_at_geometrically_closest_waypoint(&self) -> &bool {
        &self.start_at_geometrically_closest_waypoint
    }
    fn start_at_geometrically_closest_waypoint_mut(&mut self) -> &mut bool {
        &mut self.start_at_geometrically_closest_waypoint
    }
    fn intermediate_allowed_stop_dist_override(&self) -> &f32 {
        &self.intermediate_allowed_stop_dist_override
    }
    fn intermediate_allowed_stop_dist_override_mut(&mut self) -> &mut f32 {
        &mut self.intermediate_allowed_stop_dist_override
    }
    fn destination_allowed_stop_dist_override(&self) -> &f32 {
        &self.destination_allowed_stop_dist_override
    }
    fn destination_allowed_stop_dist_override_mut(&mut self) -> &mut f32 {
        &mut self.destination_allowed_stop_dist_override
    }
    fn destination_set_orientation(&self) -> &bool {
        &self.destination_set_orientation
    }
    fn destination_set_orientation_mut(&mut self) -> &mut bool {
        &mut self.destination_set_orientation
    }
}

impl super::entity::EntityDataTrait for MoverFollowWaypointsEntityData {
}

impl super::entity::GameObjectDataTrait for MoverFollowWaypointsEntityData {
}

impl super::core::DataBusPeerTrait for MoverFollowWaypointsEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MoverFollowWaypointsEntityData {
}

impl super::core::DataContainerTrait for MoverFollowWaypointsEntityData {
}

pub static MOVERFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowWaypointsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverFollowWaypointsEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TypeOfRoute",
                flags: MemberInfoFlags::new(0),
                field_type: "RouteType",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, type_of_route),
            },
            FieldInfoData {
                name: "StopAtWaypoints",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, stop_at_waypoints),
            },
            FieldInfoData {
                name: "StartAtGeometricallyClosestWaypoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, start_at_geometrically_closest_waypoint),
            },
            FieldInfoData {
                name: "IntermediateAllowedStopDistOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, intermediate_allowed_stop_dist_override),
            },
            FieldInfoData {
                name: "DestinationAllowedStopDistOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, destination_allowed_stop_dist_override),
            },
            FieldInfoData {
                name: "DestinationSetOrientation",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, destination_set_orientation),
            },
        ],
    }),
    array_type: Some(MOVERFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverFollowWaypointsEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVERFOLLOWWAYPOINTSENTITYDATA_TYPE_INFO
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


pub static MOVERFOLLOWWAYPOINTSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverFollowWaypointsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverFollowWaypointsEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoverComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub r#type: EntityMoverType,
    pub mover_tune: Option<Arc<Mutex<dyn MoverTuneTrait>>>,
    pub goal_plan_failure_treshold: f32,
    pub goal_height_failure_treshold: f32,
    pub radius_data: Option<Arc<Mutex<dyn RadiusDataTrait>>>,
    pub enable_puppet_mode: bool,
    pub move_speed_modifier: f32,
    pub desired_movement_angle_game_state: super::ant::AntRef,
    pub desired_relative_movement_angle_game_state: super::ant::AntRef,
    pub desired_movement_speed_game_state: super::ant::AntRef,
    pub desired_facing_angle_game_state: super::ant::AntRef,
    pub desired_relative_facing_angle_game_state: super::ant::AntRef,
    pub distance_to_goal_game_state: super::ant::AntRef,
}

pub trait MoverComponentDataTrait: super::entity::GameComponentDataTrait {
    fn r#type(&self) -> &EntityMoverType;
    fn r#type_mut(&mut self) -> &mut EntityMoverType;
    fn mover_tune(&self) -> &Option<Arc<Mutex<dyn MoverTuneTrait>>>;
    fn mover_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn MoverTuneTrait>>>;
    fn goal_plan_failure_treshold(&self) -> &f32;
    fn goal_plan_failure_treshold_mut(&mut self) -> &mut f32;
    fn goal_height_failure_treshold(&self) -> &f32;
    fn goal_height_failure_treshold_mut(&mut self) -> &mut f32;
    fn radius_data(&self) -> &Option<Arc<Mutex<dyn RadiusDataTrait>>>;
    fn radius_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RadiusDataTrait>>>;
    fn enable_puppet_mode(&self) -> &bool;
    fn enable_puppet_mode_mut(&mut self) -> &mut bool;
    fn move_speed_modifier(&self) -> &f32;
    fn move_speed_modifier_mut(&mut self) -> &mut f32;
    fn desired_movement_angle_game_state(&self) -> &super::ant::AntRef;
    fn desired_movement_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef;
    fn desired_relative_movement_angle_game_state(&self) -> &super::ant::AntRef;
    fn desired_relative_movement_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef;
    fn desired_movement_speed_game_state(&self) -> &super::ant::AntRef;
    fn desired_movement_speed_game_state_mut(&mut self) -> &mut super::ant::AntRef;
    fn desired_facing_angle_game_state(&self) -> &super::ant::AntRef;
    fn desired_facing_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef;
    fn desired_relative_facing_angle_game_state(&self) -> &super::ant::AntRef;
    fn desired_relative_facing_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef;
    fn distance_to_goal_game_state(&self) -> &super::ant::AntRef;
    fn distance_to_goal_game_state_mut(&mut self) -> &mut super::ant::AntRef;
}

impl MoverComponentDataTrait for MoverComponentData {
    fn r#type(&self) -> &EntityMoverType {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut EntityMoverType {
        &mut self.r#type
    }
    fn mover_tune(&self) -> &Option<Arc<Mutex<dyn MoverTuneTrait>>> {
        &self.mover_tune
    }
    fn mover_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn MoverTuneTrait>>> {
        &mut self.mover_tune
    }
    fn goal_plan_failure_treshold(&self) -> &f32 {
        &self.goal_plan_failure_treshold
    }
    fn goal_plan_failure_treshold_mut(&mut self) -> &mut f32 {
        &mut self.goal_plan_failure_treshold
    }
    fn goal_height_failure_treshold(&self) -> &f32 {
        &self.goal_height_failure_treshold
    }
    fn goal_height_failure_treshold_mut(&mut self) -> &mut f32 {
        &mut self.goal_height_failure_treshold
    }
    fn radius_data(&self) -> &Option<Arc<Mutex<dyn RadiusDataTrait>>> {
        &self.radius_data
    }
    fn radius_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RadiusDataTrait>>> {
        &mut self.radius_data
    }
    fn enable_puppet_mode(&self) -> &bool {
        &self.enable_puppet_mode
    }
    fn enable_puppet_mode_mut(&mut self) -> &mut bool {
        &mut self.enable_puppet_mode
    }
    fn move_speed_modifier(&self) -> &f32 {
        &self.move_speed_modifier
    }
    fn move_speed_modifier_mut(&mut self) -> &mut f32 {
        &mut self.move_speed_modifier
    }
    fn desired_movement_angle_game_state(&self) -> &super::ant::AntRef {
        &self.desired_movement_angle_game_state
    }
    fn desired_movement_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.desired_movement_angle_game_state
    }
    fn desired_relative_movement_angle_game_state(&self) -> &super::ant::AntRef {
        &self.desired_relative_movement_angle_game_state
    }
    fn desired_relative_movement_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.desired_relative_movement_angle_game_state
    }
    fn desired_movement_speed_game_state(&self) -> &super::ant::AntRef {
        &self.desired_movement_speed_game_state
    }
    fn desired_movement_speed_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.desired_movement_speed_game_state
    }
    fn desired_facing_angle_game_state(&self) -> &super::ant::AntRef {
        &self.desired_facing_angle_game_state
    }
    fn desired_facing_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.desired_facing_angle_game_state
    }
    fn desired_relative_facing_angle_game_state(&self) -> &super::ant::AntRef {
        &self.desired_relative_facing_angle_game_state
    }
    fn desired_relative_facing_angle_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.desired_relative_facing_angle_game_state
    }
    fn distance_to_goal_game_state(&self) -> &super::ant::AntRef {
        &self.distance_to_goal_game_state
    }
    fn distance_to_goal_game_state_mut(&mut self) -> &mut super::ant::AntRef {
        &mut self.distance_to_goal_game_state
    }
}

impl super::entity::GameComponentDataTrait for MoverComponentData {
}

impl super::entity::ComponentDataTrait for MoverComponentData {
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

impl super::entity::GameObjectDataTrait for MoverComponentData {
}

impl super::core::DataBusPeerTrait for MoverComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MoverComponentData {
}

impl super::core::DataContainerTrait for MoverComponentData {
}

pub static MOVERCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverComponentData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "EntityMoverType",
                rust_offset: offset_of!(MoverComponentData, r#type),
            },
            FieldInfoData {
                name: "moverTune",
                flags: MemberInfoFlags::new(0),
                field_type: "MoverTune",
                rust_offset: offset_of!(MoverComponentData, mover_tune),
            },
            FieldInfoData {
                name: "goalPlanFailureTreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverComponentData, goal_plan_failure_treshold),
            },
            FieldInfoData {
                name: "goalHeightFailureTreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverComponentData, goal_height_failure_treshold),
            },
            FieldInfoData {
                name: "radiusData",
                flags: MemberInfoFlags::new(0),
                field_type: "RadiusData",
                rust_offset: offset_of!(MoverComponentData, radius_data),
            },
            FieldInfoData {
                name: "EnablePuppetMode",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverComponentData, enable_puppet_mode),
            },
            FieldInfoData {
                name: "MoveSpeedModifier",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverComponentData, move_speed_modifier),
            },
            FieldInfoData {
                name: "DesiredMovementAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_movement_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredRelativeMovementAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_relative_movement_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredMovementSpeedGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_movement_speed_game_state),
            },
            FieldInfoData {
                name: "DesiredFacingAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_facing_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredRelativeFacingAngleGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_relative_facing_angle_game_state),
            },
            FieldInfoData {
                name: "DistanceToGoalGameState",
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, distance_to_goal_game_state),
            },
        ],
    }),
    array_type: Some(MOVERCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for MoverComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVERCOMPONENTDATA_TYPE_INFO
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


pub static MOVERCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverComponentData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntityMoverType {
    #[default]
    EntityMoverType_Only_Repulsor = 0,
    EntityMoverType_Both_MoverRepulsor = 1,
}

pub static ENTITYMOVERTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityMoverType",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYMOVERTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityMoverType {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYMOVERTYPE_TYPE_INFO
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


pub static ENTITYMOVERTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityMoverType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("EntityMoverType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoverTuneOverride {
    pub _glacier_base: MoverTune,
}

pub trait MoverTuneOverrideTrait: MoverTuneTrait {
}

impl MoverTuneOverrideTrait for MoverTuneOverride {
}

impl MoverTuneTrait for MoverTuneOverride {
    fn speed(&self) -> &f32 {
        self._glacier_base.speed()
    }
    fn speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.speed_mut()
    }
    fn max_speed_fraction(&self) -> &f32 {
        self._glacier_base.max_speed_fraction()
    }
    fn max_speed_fraction_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_speed_fraction_mut()
    }
    fn radius_data(&self) -> &Option<Arc<Mutex<dyn RadiusDataTrait>>> {
        self._glacier_base.radius_data()
    }
    fn radius_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RadiusDataTrait>>> {
        self._glacier_base.radius_data_mut()
    }
    fn bulk(&self) -> &f32 {
        self._glacier_base.bulk()
    }
    fn bulk_mut(&mut self) -> &mut f32 {
        self._glacier_base.bulk_mut()
    }
    fn cruise_acc(&self) -> &f32 {
        self._glacier_base.cruise_acc()
    }
    fn cruise_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.cruise_acc_mut()
    }
    fn start_stop_acc(&self) -> &f32 {
        self._glacier_base.start_stop_acc()
    }
    fn start_stop_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.start_stop_acc_mut()
    }
    fn repulsor_type(&self) -> &i32 {
        self._glacier_base.repulsor_type()
    }
    fn repulsor_type_mut(&mut self) -> &mut i32 {
        self._glacier_base.repulsor_type_mut()
    }
    fn flock_acc(&self) -> &f32 {
        self._glacier_base.flock_acc()
    }
    fn flock_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.flock_acc_mut()
    }
    fn max_flock_acc_dist(&self) -> &f32 {
        self._glacier_base.max_flock_acc_dist()
    }
    fn max_flock_acc_dist_mut(&mut self) -> &mut f32 {
        self._glacier_base.max_flock_acc_dist_mut()
    }
    fn path_acc(&self) -> &f32 {
        self._glacier_base.path_acc()
    }
    fn path_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.path_acc_mut()
    }
    fn caution_tune(&self) -> &Option<Arc<Mutex<dyn CautionTuneTrait>>> {
        self._glacier_base.caution_tune()
    }
    fn caution_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CautionTuneTrait>>> {
        self._glacier_base.caution_tune_mut()
    }
    fn backpedal_fraction(&self) -> &f32 {
        self._glacier_base.backpedal_fraction()
    }
    fn backpedal_fraction_mut(&mut self) -> &mut f32 {
        self._glacier_base.backpedal_fraction_mut()
    }
    fn plan_layer(&self) -> &u32 {
        self._glacier_base.plan_layer()
    }
    fn plan_layer_mut(&mut self) -> &mut u32 {
        self._glacier_base.plan_layer_mut()
    }
    fn path_sharing_penalty(&self) -> &f32 {
        self._glacier_base.path_sharing_penalty()
    }
    fn path_sharing_penalty_mut(&mut self) -> &mut f32 {
        self._glacier_base.path_sharing_penalty_mut()
    }
    fn obstacle_mode(&self) -> &BlockageMode {
        self._glacier_base.obstacle_mode()
    }
    fn obstacle_mode_mut(&mut self) -> &mut BlockageMode {
        self._glacier_base.obstacle_mode_mut()
    }
    fn obstacle_blockage_flags(&self) -> &u32 {
        self._glacier_base.obstacle_blockage_flags()
    }
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.obstacle_blockage_flags_mut()
    }
    fn auto_ob_tune(&self) -> &Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>> {
        self._glacier_base.auto_ob_tune()
    }
    fn auto_ob_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>> {
        self._glacier_base.auto_ob_tune_mut()
    }
    fn repulsor_blockage_flags(&self) -> &u32 {
        self._glacier_base.repulsor_blockage_flags()
    }
    fn repulsor_blockage_flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.repulsor_blockage_flags_mut()
    }
    fn repulsor_identity_flags(&self) -> &u32 {
        self._glacier_base.repulsor_identity_flags()
    }
    fn repulsor_identity_flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.repulsor_identity_flags_mut()
    }
    fn link_usage_flags(&self) -> &u32 {
        self._glacier_base.link_usage_flags()
    }
    fn link_usage_flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.link_usage_flags_mut()
    }
    fn path_options(&self) -> &Option<Arc<Mutex<dyn PathCreationOptionsTrait>>> {
        self._glacier_base.path_options()
    }
    fn path_options_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PathCreationOptionsTrait>>> {
        self._glacier_base.path_options_mut()
    }
    fn jumper_tune(&self) -> &Option<Arc<Mutex<dyn JumperTuneTrait>>> {
        self._glacier_base.jumper_tune()
    }
    fn jumper_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn JumperTuneTrait>>> {
        self._glacier_base.jumper_tune_mut()
    }
    fn exit_puppet_in_obstacles(&self) -> &bool {
        self._glacier_base.exit_puppet_in_obstacles()
    }
    fn exit_puppet_in_obstacles_mut(&mut self) -> &mut bool {
        self._glacier_base.exit_puppet_in_obstacles_mut()
    }
    fn prober_tune(&self) -> &Option<Arc<Mutex<dyn ProberTuneTrait>>> {
        self._glacier_base.prober_tune()
    }
    fn prober_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProberTuneTrait>>> {
        self._glacier_base.prober_tune_mut()
    }
    fn allow_detour(&self) -> &bool {
        self._glacier_base.allow_detour()
    }
    fn allow_detour_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_detour_mut()
    }
    fn goal_tune(&self) -> &Option<Arc<Mutex<dyn GoalTuneTrait>>> {
        self._glacier_base.goal_tune()
    }
    fn goal_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GoalTuneTrait>>> {
        self._glacier_base.goal_tune_mut()
    }
    fn idle_tune(&self) -> &Option<Arc<Mutex<dyn IdleTuneTrait>>> {
        self._glacier_base.idle_tune()
    }
    fn idle_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn IdleTuneTrait>>> {
        self._glacier_base.idle_tune_mut()
    }
    fn turn_in_place(&self) -> &Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>> {
        self._glacier_base.turn_in_place()
    }
    fn turn_in_place_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>> {
        self._glacier_base.turn_in_place_mut()
    }
    fn repulsion_acceleration_tune(&self) -> &Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>> {
        self._glacier_base.repulsion_acceleration_tune()
    }
    fn repulsion_acceleration_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>> {
        self._glacier_base.repulsion_acceleration_tune_mut()
    }
    fn surface_orient_tune(&self) -> &Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>> {
        self._glacier_base.surface_orient_tune()
    }
    fn surface_orient_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>> {
        self._glacier_base.surface_orient_tune_mut()
    }
    fn sidestep_fraction(&self) -> &f32 {
        self._glacier_base.sidestep_fraction()
    }
    fn sidestep_fraction_mut(&mut self) -> &mut f32 {
        self._glacier_base.sidestep_fraction_mut()
    }
    fn custom_geo_match_flags(&self) -> &u32 {
        self._glacier_base.custom_geo_match_flags()
    }
    fn custom_geo_match_flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.custom_geo_match_flags_mut()
    }
    fn client_motion(&self) -> &bool {
        self._glacier_base.client_motion()
    }
    fn client_motion_mut(&mut self) -> &mut bool {
        self._glacier_base.client_motion_mut()
    }
    fn follower_tune(&self) -> &Option<Arc<Mutex<dyn FollowerTuneTrait>>> {
        self._glacier_base.follower_tune()
    }
    fn follower_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FollowerTuneTrait>>> {
        self._glacier_base.follower_tune_mut()
    }
}

impl super::core::AssetTrait for MoverTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MoverTuneOverride {
}

pub static MOVERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVERTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MOVERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        MOVERTUNEOVERRIDE_TYPE_INFO
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


pub static MOVERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FollowerTuneOverride {
    pub _glacier_base: FollowerTune,
}

pub trait FollowerTuneOverrideTrait: FollowerTuneTrait {
}

impl FollowerTuneOverrideTrait for FollowerTuneOverride {
}

impl FollowerTuneTrait for FollowerTuneOverride {
    fn circulate_enable(&self) -> &bool {
        self._glacier_base.circulate_enable()
    }
    fn circulate_enable_mut(&mut self) -> &mut bool {
        self._glacier_base.circulate_enable_mut()
    }
    fn circulate_min_time(&self) -> &f32 {
        self._glacier_base.circulate_min_time()
    }
    fn circulate_min_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.circulate_min_time_mut()
    }
    fn circulate_max_time(&self) -> &f32 {
        self._glacier_base.circulate_max_time()
    }
    fn circulate_max_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.circulate_max_time_mut()
    }
    fn startup_slowness(&self) -> &f32 {
        self._glacier_base.startup_slowness()
    }
    fn startup_slowness_mut(&mut self) -> &mut f32 {
        self._glacier_base.startup_slowness_mut()
    }
    fn startup_bulk(&self) -> &f32 {
        self._glacier_base.startup_bulk()
    }
    fn startup_bulk_mut(&mut self) -> &mut f32 {
        self._glacier_base.startup_bulk_mut()
    }
    fn packing_padding(&self) -> &f32 {
        self._glacier_base.packing_padding()
    }
    fn packing_padding_mut(&mut self) -> &mut f32 {
        self._glacier_base.packing_padding_mut()
    }
}

impl super::core::AssetTrait for FollowerTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FollowerTuneOverride {
}

pub static FOLLOWERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FOLLOWERTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowerTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FOLLOWERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FollowerTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        FOLLOWERTUNEOVERRIDE_TYPE_INFO
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


pub static FOLLOWERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowerTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SurfaceOrientTuneOverride {
    pub _glacier_base: SurfaceOrientTune,
}

pub trait SurfaceOrientTuneOverrideTrait: SurfaceOrientTuneTrait {
}

impl SurfaceOrientTuneOverrideTrait for SurfaceOrientTuneOverride {
}

impl SurfaceOrientTuneTrait for SurfaceOrientTuneOverride {
    fn surface_orient_threshold(&self) -> &f32 {
        self._glacier_base.surface_orient_threshold()
    }
    fn surface_orient_threshold_mut(&mut self) -> &mut f32 {
        self._glacier_base.surface_orient_threshold_mut()
    }
    fn always_vertical_on_auto_gen(&self) -> &bool {
        self._glacier_base.always_vertical_on_auto_gen()
    }
    fn always_vertical_on_auto_gen_mut(&mut self) -> &mut bool {
        self._glacier_base.always_vertical_on_auto_gen_mut()
    }
    fn surface_orient_slerp_time(&self) -> &f32 {
        self._glacier_base.surface_orient_slerp_time()
    }
    fn surface_orient_slerp_time_mut(&mut self) -> &mut f32 {
        self._glacier_base.surface_orient_slerp_time_mut()
    }
}

impl super::core::AssetTrait for SurfaceOrientTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SurfaceOrientTuneOverride {
}

pub static SURFACEORIENTTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SURFACEORIENTTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SurfaceOrientTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SURFACEORIENTTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceOrientTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACEORIENTTUNEOVERRIDE_TYPE_INFO
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


pub static SURFACEORIENTTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("SurfaceOrientTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoObstacleTuneOverride {
    pub _glacier_base: AutoObstacleTune,
}

pub trait AutoObstacleTuneOverrideTrait: AutoObstacleTuneTrait {
}

impl AutoObstacleTuneOverrideTrait for AutoObstacleTuneOverride {
}

impl AutoObstacleTuneTrait for AutoObstacleTuneOverride {
    fn auto_create_obstacle(&self) -> &bool {
        self._glacier_base.auto_create_obstacle()
    }
    fn auto_create_obstacle_mut(&mut self) -> &mut bool {
        self._glacier_base.auto_create_obstacle_mut()
    }
    fn delay(&self) -> &f32 {
        self._glacier_base.delay()
    }
    fn delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.delay_mut()
    }
    fn obstacle_blockage_flags(&self) -> &u32 {
        self._glacier_base.obstacle_blockage_flags()
    }
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.obstacle_blockage_flags_mut()
    }
}

impl super::core::AssetTrait for AutoObstacleTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for AutoObstacleTuneOverride {
}

pub static AUTOOBSTACLETUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOOBSTACLETUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoObstacleTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(AUTOOBSTACLETUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoObstacleTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOOBSTACLETUNEOVERRIDE_TYPE_INFO
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


pub static AUTOOBSTACLETUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("AutoObstacleTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IdleTuneOverride {
    pub _glacier_base: IdleTune,
}

pub trait IdleTuneOverrideTrait: IdleTuneTrait {
}

impl IdleTuneOverrideTrait for IdleTuneOverride {
}

impl IdleTuneTrait for IdleTuneOverride {
    fn tether_dist(&self) -> &f32 {
        self._glacier_base.tether_dist()
    }
    fn tether_dist_mut(&mut self) -> &mut f32 {
        self._glacier_base.tether_dist_mut()
    }
    fn return_delay(&self) -> &f32 {
        self._glacier_base.return_delay()
    }
    fn return_delay_mut(&mut self) -> &mut f32 {
        self._glacier_base.return_delay_mut()
    }
}

impl super::core::AssetTrait for IdleTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for IdleTuneOverride {
}

pub static IDLETUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IDLETUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IdleTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IDLETUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        IDLETUNEOVERRIDE_TYPE_INFO
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


pub static IDLETUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("IdleTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GoalTuneOverride {
    pub _glacier_base: GoalTune,
}

pub trait GoalTuneOverrideTrait: GoalTuneTrait {
}

impl GoalTuneOverrideTrait for GoalTuneOverride {
}

impl GoalTuneTrait for GoalTuneOverride {
    fn use_circular_approach(&self) -> &bool {
        self._glacier_base.use_circular_approach()
    }
    fn use_circular_approach_mut(&mut self) -> &mut bool {
        self._glacier_base.use_circular_approach_mut()
    }
    fn preferred_turning_radius(&self) -> &f32 {
        self._glacier_base.preferred_turning_radius()
    }
    fn preferred_turning_radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.preferred_turning_radius_mut()
    }
}

impl super::core::AssetTrait for GoalTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GoalTuneOverride {
}

pub static GOALTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GOALTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GoalTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GOALTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GoalTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        GOALTUNEOVERRIDE_TYPE_INFO
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


pub static GOALTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GoalTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProberTuneOverride {
    pub _glacier_base: ProberTune,
}

pub trait ProberTuneOverrideTrait: ProberTuneTrait {
}

impl ProberTuneOverrideTrait for ProberTuneOverride {
}

impl ProberTuneTrait for ProberTuneOverride {
    fn probe_for_ground(&self) -> &bool {
        self._glacier_base.probe_for_ground()
    }
    fn probe_for_ground_mut(&mut self) -> &mut bool {
        self._glacier_base.probe_for_ground_mut()
    }
    fn probe_interval(&self) -> &f32 {
        self._glacier_base.probe_interval()
    }
    fn probe_interval_mut(&mut self) -> &mut f32 {
        self._glacier_base.probe_interval_mut()
    }
}

impl super::core::AssetTrait for ProberTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProberTuneOverride {
}

pub static PROBERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROBERTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProberTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROBERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProberTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        PROBERTUNEOVERRIDE_TYPE_INFO
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


pub static PROBERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ProberTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct JumperTuneOverride {
    pub _glacier_base: JumperTune,
}

pub trait JumperTuneOverrideTrait: JumperTuneTrait {
}

impl JumperTuneOverrideTrait for JumperTuneOverride {
}

impl JumperTuneTrait for JumperTuneOverride {
    fn speed(&self) -> &f32 {
        self._glacier_base.speed()
    }
    fn speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.speed_mut()
    }
    fn arc_fraction(&self) -> &f32 {
        self._glacier_base.arc_fraction()
    }
    fn arc_fraction_mut(&mut self) -> &mut f32 {
        self._glacier_base.arc_fraction_mut()
    }
    fn turn_before_jump_angle(&self) -> &f32 {
        self._glacier_base.turn_before_jump_angle()
    }
    fn turn_before_jump_angle_mut(&mut self) -> &mut f32 {
        self._glacier_base.turn_before_jump_angle_mut()
    }
    fn keep_speed_when_swap_to_default(&self) -> &bool {
        self._glacier_base.keep_speed_when_swap_to_default()
    }
    fn keep_speed_when_swap_to_default_mut(&mut self) -> &mut bool {
        self._glacier_base.keep_speed_when_swap_to_default_mut()
    }
    fn only_jump_to_end_point(&self) -> &bool {
        self._glacier_base.only_jump_to_end_point()
    }
    fn only_jump_to_end_point_mut(&mut self) -> &mut bool {
        self._glacier_base.only_jump_to_end_point_mut()
    }
}

impl super::core::AssetTrait for JumperTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for JumperTuneOverride {
}

pub static JUMPERTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(JUMPERTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JumperTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(JUMPERTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JumperTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        JUMPERTUNEOVERRIDE_TYPE_INFO
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


pub static JUMPERTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("JumperTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CautionTuneOverride {
    pub _glacier_base: CautionTune,
}

pub trait CautionTuneOverrideTrait: CautionTuneTrait {
}

impl CautionTuneOverrideTrait for CautionTuneOverride {
}

impl CautionTuneTrait for CautionTuneOverride {
    fn speed_x(&self) -> &f32 {
        self._glacier_base.speed_x()
    }
    fn speed_x_mut(&mut self) -> &mut f32 {
        self._glacier_base.speed_x_mut()
    }
    fn tight_turn_degrees(&self) -> &f32 {
        self._glacier_base.tight_turn_degrees()
    }
    fn tight_turn_degrees_mut(&mut self) -> &mut f32 {
        self._glacier_base.tight_turn_degrees_mut()
    }
}

impl super::core::AssetTrait for CautionTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CautionTuneOverride {
}

pub static CAUTIONTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAUTIONTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CautionTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CAUTIONTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CautionTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        CAUTIONTUNEOVERRIDE_TYPE_INFO
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


pub static CAUTIONTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CautionTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RepulsionAccelerationTuneOverride {
    pub _glacier_base: RepulsionAccelerationTune,
}

pub trait RepulsionAccelerationTuneOverrideTrait: RepulsionAccelerationTuneTrait {
}

impl RepulsionAccelerationTuneOverrideTrait for RepulsionAccelerationTuneOverride {
}

impl RepulsionAccelerationTuneTrait for RepulsionAccelerationTuneOverride {
    fn initial_acc(&self) -> &f32 {
        self._glacier_base.initial_acc()
    }
    fn initial_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.initial_acc_mut()
    }
    fn outer_cushion_acc(&self) -> &f32 {
        self._glacier_base.outer_cushion_acc()
    }
    fn outer_cushion_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.outer_cushion_acc_mut()
    }
    fn inner_cushion_acc(&self) -> &f32 {
        self._glacier_base.inner_cushion_acc()
    }
    fn inner_cushion_acc_mut(&mut self) -> &mut f32 {
        self._glacier_base.inner_cushion_acc_mut()
    }
}

impl super::core::AssetTrait for RepulsionAccelerationTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RepulsionAccelerationTuneOverride {
}

pub static REPULSIONACCELERATIONTUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REPULSIONACCELERATIONTUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RepulsionAccelerationTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(REPULSIONACCELERATIONTUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RepulsionAccelerationTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        REPULSIONACCELERATIONTUNEOVERRIDE_TYPE_INFO
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


pub static REPULSIONACCELERATIONTUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RepulsionAccelerationTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TurnInPlaceTuneOverride {
    pub _glacier_base: TurnInPlaceTune,
}

pub trait TurnInPlaceTuneOverrideTrait: TurnInPlaceTuneTrait {
}

impl TurnInPlaceTuneOverrideTrait for TurnInPlaceTuneOverride {
}

impl TurnInPlaceTuneTrait for TurnInPlaceTuneOverride {
    fn when_moving_angle(&self) -> &f32 {
        self._glacier_base.when_moving_angle()
    }
    fn when_moving_angle_mut(&mut self) -> &mut f32 {
        self._glacier_base.when_moving_angle_mut()
    }
    fn when_stopped_angle(&self) -> &f32 {
        self._glacier_base.when_stopped_angle()
    }
    fn when_stopped_angle_mut(&mut self) -> &mut f32 {
        self._glacier_base.when_stopped_angle_mut()
    }
    fn speed(&self) -> &f32 {
        self._glacier_base.speed()
    }
    fn speed_mut(&mut self) -> &mut f32 {
        self._glacier_base.speed_mut()
    }
    fn accel_angle(&self) -> &f32 {
        self._glacier_base.accel_angle()
    }
    fn accel_angle_mut(&mut self) -> &mut f32 {
        self._glacier_base.accel_angle_mut()
    }
    fn slide_spin_threshold(&self) -> &f32 {
        self._glacier_base.slide_spin_threshold()
    }
    fn slide_spin_threshold_mut(&mut self) -> &mut f32 {
        self._glacier_base.slide_spin_threshold_mut()
    }
    fn enable_u_turn(&self) -> &bool {
        self._glacier_base.enable_u_turn()
    }
    fn enable_u_turn_mut(&mut self) -> &mut bool {
        self._glacier_base.enable_u_turn_mut()
    }
}

impl super::core::AssetTrait for TurnInPlaceTuneOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TurnInPlaceTuneOverride {
}

pub static TURNINPLACETUNEOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTuneOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TURNINPLACETUNE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurnInPlaceTuneOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TURNINPLACETUNEOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnInPlaceTuneOverride {
    fn type_info(&self) -> &'static TypeInfo {
        TURNINPLACETUNEOVERRIDE_TYPE_INFO
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


pub static TURNINPLACETUNEOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTuneOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("TurnInPlaceTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiusDataOverride {
    pub _glacier_base: RadiusData,
}

pub trait RadiusDataOverrideTrait: RadiusDataTrait {
}

impl RadiusDataOverrideTrait for RadiusDataOverride {
}

impl RadiusDataTrait for RadiusDataOverride {
    fn radius(&self) -> &f32 {
        self._glacier_base.radius()
    }
    fn radius_mut(&mut self) -> &mut f32 {
        self._glacier_base.radius_mut()
    }
    fn outer_cushion(&self) -> &f32 {
        self._glacier_base.outer_cushion()
    }
    fn outer_cushion_mut(&mut self) -> &mut f32 {
        self._glacier_base.outer_cushion_mut()
    }
    fn inner_cushion(&self) -> &f32 {
        self._glacier_base.inner_cushion()
    }
    fn inner_cushion_mut(&mut self) -> &mut f32 {
        self._glacier_base.inner_cushion_mut()
    }
}

impl super::core::AssetTrait for RadiusDataOverride {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RadiusDataOverride {
}

pub static RADIUSDATAOVERRIDE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusDataOverride",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RADIUSDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiusDataOverride as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RADIUSDATAOVERRIDE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RadiusDataOverride {
    fn type_info(&self) -> &'static TypeInfo {
        RADIUSDATAOVERRIDE_TYPE_INFO
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


pub static RADIUSDATAOVERRIDE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusDataOverride-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RadiusDataOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MoverTune {
    pub _glacier_base: super::core::Asset,
    pub speed: f32,
    pub max_speed_fraction: f32,
    pub radius_data: Option<Arc<Mutex<dyn RadiusDataTrait>>>,
    pub bulk: f32,
    pub cruise_acc: f32,
    pub start_stop_acc: f32,
    pub repulsor_type: i32,
    pub flock_acc: f32,
    pub max_flock_acc_dist: f32,
    pub path_acc: f32,
    pub caution_tune: Option<Arc<Mutex<dyn CautionTuneTrait>>>,
    pub backpedal_fraction: f32,
    pub plan_layer: u32,
    pub path_sharing_penalty: f32,
    pub obstacle_mode: BlockageMode,
    pub obstacle_blockage_flags: u32,
    pub auto_ob_tune: Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>>,
    pub repulsor_blockage_flags: u32,
    pub repulsor_identity_flags: u32,
    pub link_usage_flags: u32,
    pub path_options: Option<Arc<Mutex<dyn PathCreationOptionsTrait>>>,
    pub jumper_tune: Option<Arc<Mutex<dyn JumperTuneTrait>>>,
    pub exit_puppet_in_obstacles: bool,
    pub prober_tune: Option<Arc<Mutex<dyn ProberTuneTrait>>>,
    pub allow_detour: bool,
    pub goal_tune: Option<Arc<Mutex<dyn GoalTuneTrait>>>,
    pub idle_tune: Option<Arc<Mutex<dyn IdleTuneTrait>>>,
    pub turn_in_place: Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>>,
    pub repulsion_acceleration_tune: Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>>,
    pub surface_orient_tune: Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>>,
    pub sidestep_fraction: f32,
    pub custom_geo_match_flags: u32,
    pub client_motion: bool,
    pub follower_tune: Option<Arc<Mutex<dyn FollowerTuneTrait>>>,
}

pub trait MoverTuneTrait: super::core::AssetTrait {
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn max_speed_fraction(&self) -> &f32;
    fn max_speed_fraction_mut(&mut self) -> &mut f32;
    fn radius_data(&self) -> &Option<Arc<Mutex<dyn RadiusDataTrait>>>;
    fn radius_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RadiusDataTrait>>>;
    fn bulk(&self) -> &f32;
    fn bulk_mut(&mut self) -> &mut f32;
    fn cruise_acc(&self) -> &f32;
    fn cruise_acc_mut(&mut self) -> &mut f32;
    fn start_stop_acc(&self) -> &f32;
    fn start_stop_acc_mut(&mut self) -> &mut f32;
    fn repulsor_type(&self) -> &i32;
    fn repulsor_type_mut(&mut self) -> &mut i32;
    fn flock_acc(&self) -> &f32;
    fn flock_acc_mut(&mut self) -> &mut f32;
    fn max_flock_acc_dist(&self) -> &f32;
    fn max_flock_acc_dist_mut(&mut self) -> &mut f32;
    fn path_acc(&self) -> &f32;
    fn path_acc_mut(&mut self) -> &mut f32;
    fn caution_tune(&self) -> &Option<Arc<Mutex<dyn CautionTuneTrait>>>;
    fn caution_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CautionTuneTrait>>>;
    fn backpedal_fraction(&self) -> &f32;
    fn backpedal_fraction_mut(&mut self) -> &mut f32;
    fn plan_layer(&self) -> &u32;
    fn plan_layer_mut(&mut self) -> &mut u32;
    fn path_sharing_penalty(&self) -> &f32;
    fn path_sharing_penalty_mut(&mut self) -> &mut f32;
    fn obstacle_mode(&self) -> &BlockageMode;
    fn obstacle_mode_mut(&mut self) -> &mut BlockageMode;
    fn obstacle_blockage_flags(&self) -> &u32;
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32;
    fn auto_ob_tune(&self) -> &Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>>;
    fn auto_ob_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>>;
    fn repulsor_blockage_flags(&self) -> &u32;
    fn repulsor_blockage_flags_mut(&mut self) -> &mut u32;
    fn repulsor_identity_flags(&self) -> &u32;
    fn repulsor_identity_flags_mut(&mut self) -> &mut u32;
    fn link_usage_flags(&self) -> &u32;
    fn link_usage_flags_mut(&mut self) -> &mut u32;
    fn path_options(&self) -> &Option<Arc<Mutex<dyn PathCreationOptionsTrait>>>;
    fn path_options_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PathCreationOptionsTrait>>>;
    fn jumper_tune(&self) -> &Option<Arc<Mutex<dyn JumperTuneTrait>>>;
    fn jumper_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn JumperTuneTrait>>>;
    fn exit_puppet_in_obstacles(&self) -> &bool;
    fn exit_puppet_in_obstacles_mut(&mut self) -> &mut bool;
    fn prober_tune(&self) -> &Option<Arc<Mutex<dyn ProberTuneTrait>>>;
    fn prober_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProberTuneTrait>>>;
    fn allow_detour(&self) -> &bool;
    fn allow_detour_mut(&mut self) -> &mut bool;
    fn goal_tune(&self) -> &Option<Arc<Mutex<dyn GoalTuneTrait>>>;
    fn goal_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GoalTuneTrait>>>;
    fn idle_tune(&self) -> &Option<Arc<Mutex<dyn IdleTuneTrait>>>;
    fn idle_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn IdleTuneTrait>>>;
    fn turn_in_place(&self) -> &Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>>;
    fn turn_in_place_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>>;
    fn repulsion_acceleration_tune(&self) -> &Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>>;
    fn repulsion_acceleration_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>>;
    fn surface_orient_tune(&self) -> &Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>>;
    fn surface_orient_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>>;
    fn sidestep_fraction(&self) -> &f32;
    fn sidestep_fraction_mut(&mut self) -> &mut f32;
    fn custom_geo_match_flags(&self) -> &u32;
    fn custom_geo_match_flags_mut(&mut self) -> &mut u32;
    fn client_motion(&self) -> &bool;
    fn client_motion_mut(&mut self) -> &mut bool;
    fn follower_tune(&self) -> &Option<Arc<Mutex<dyn FollowerTuneTrait>>>;
    fn follower_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FollowerTuneTrait>>>;
}

impl MoverTuneTrait for MoverTune {
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }
    fn max_speed_fraction(&self) -> &f32 {
        &self.max_speed_fraction
    }
    fn max_speed_fraction_mut(&mut self) -> &mut f32 {
        &mut self.max_speed_fraction
    }
    fn radius_data(&self) -> &Option<Arc<Mutex<dyn RadiusDataTrait>>> {
        &self.radius_data
    }
    fn radius_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RadiusDataTrait>>> {
        &mut self.radius_data
    }
    fn bulk(&self) -> &f32 {
        &self.bulk
    }
    fn bulk_mut(&mut self) -> &mut f32 {
        &mut self.bulk
    }
    fn cruise_acc(&self) -> &f32 {
        &self.cruise_acc
    }
    fn cruise_acc_mut(&mut self) -> &mut f32 {
        &mut self.cruise_acc
    }
    fn start_stop_acc(&self) -> &f32 {
        &self.start_stop_acc
    }
    fn start_stop_acc_mut(&mut self) -> &mut f32 {
        &mut self.start_stop_acc
    }
    fn repulsor_type(&self) -> &i32 {
        &self.repulsor_type
    }
    fn repulsor_type_mut(&mut self) -> &mut i32 {
        &mut self.repulsor_type
    }
    fn flock_acc(&self) -> &f32 {
        &self.flock_acc
    }
    fn flock_acc_mut(&mut self) -> &mut f32 {
        &mut self.flock_acc
    }
    fn max_flock_acc_dist(&self) -> &f32 {
        &self.max_flock_acc_dist
    }
    fn max_flock_acc_dist_mut(&mut self) -> &mut f32 {
        &mut self.max_flock_acc_dist
    }
    fn path_acc(&self) -> &f32 {
        &self.path_acc
    }
    fn path_acc_mut(&mut self) -> &mut f32 {
        &mut self.path_acc
    }
    fn caution_tune(&self) -> &Option<Arc<Mutex<dyn CautionTuneTrait>>> {
        &self.caution_tune
    }
    fn caution_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CautionTuneTrait>>> {
        &mut self.caution_tune
    }
    fn backpedal_fraction(&self) -> &f32 {
        &self.backpedal_fraction
    }
    fn backpedal_fraction_mut(&mut self) -> &mut f32 {
        &mut self.backpedal_fraction
    }
    fn plan_layer(&self) -> &u32 {
        &self.plan_layer
    }
    fn plan_layer_mut(&mut self) -> &mut u32 {
        &mut self.plan_layer
    }
    fn path_sharing_penalty(&self) -> &f32 {
        &self.path_sharing_penalty
    }
    fn path_sharing_penalty_mut(&mut self) -> &mut f32 {
        &mut self.path_sharing_penalty
    }
    fn obstacle_mode(&self) -> &BlockageMode {
        &self.obstacle_mode
    }
    fn obstacle_mode_mut(&mut self) -> &mut BlockageMode {
        &mut self.obstacle_mode
    }
    fn obstacle_blockage_flags(&self) -> &u32 {
        &self.obstacle_blockage_flags
    }
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32 {
        &mut self.obstacle_blockage_flags
    }
    fn auto_ob_tune(&self) -> &Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>> {
        &self.auto_ob_tune
    }
    fn auto_ob_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn AutoObstacleTuneTrait>>> {
        &mut self.auto_ob_tune
    }
    fn repulsor_blockage_flags(&self) -> &u32 {
        &self.repulsor_blockage_flags
    }
    fn repulsor_blockage_flags_mut(&mut self) -> &mut u32 {
        &mut self.repulsor_blockage_flags
    }
    fn repulsor_identity_flags(&self) -> &u32 {
        &self.repulsor_identity_flags
    }
    fn repulsor_identity_flags_mut(&mut self) -> &mut u32 {
        &mut self.repulsor_identity_flags
    }
    fn link_usage_flags(&self) -> &u32 {
        &self.link_usage_flags
    }
    fn link_usage_flags_mut(&mut self) -> &mut u32 {
        &mut self.link_usage_flags
    }
    fn path_options(&self) -> &Option<Arc<Mutex<dyn PathCreationOptionsTrait>>> {
        &self.path_options
    }
    fn path_options_mut(&mut self) -> &mut Option<Arc<Mutex<dyn PathCreationOptionsTrait>>> {
        &mut self.path_options
    }
    fn jumper_tune(&self) -> &Option<Arc<Mutex<dyn JumperTuneTrait>>> {
        &self.jumper_tune
    }
    fn jumper_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn JumperTuneTrait>>> {
        &mut self.jumper_tune
    }
    fn exit_puppet_in_obstacles(&self) -> &bool {
        &self.exit_puppet_in_obstacles
    }
    fn exit_puppet_in_obstacles_mut(&mut self) -> &mut bool {
        &mut self.exit_puppet_in_obstacles
    }
    fn prober_tune(&self) -> &Option<Arc<Mutex<dyn ProberTuneTrait>>> {
        &self.prober_tune
    }
    fn prober_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ProberTuneTrait>>> {
        &mut self.prober_tune
    }
    fn allow_detour(&self) -> &bool {
        &self.allow_detour
    }
    fn allow_detour_mut(&mut self) -> &mut bool {
        &mut self.allow_detour
    }
    fn goal_tune(&self) -> &Option<Arc<Mutex<dyn GoalTuneTrait>>> {
        &self.goal_tune
    }
    fn goal_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GoalTuneTrait>>> {
        &mut self.goal_tune
    }
    fn idle_tune(&self) -> &Option<Arc<Mutex<dyn IdleTuneTrait>>> {
        &self.idle_tune
    }
    fn idle_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn IdleTuneTrait>>> {
        &mut self.idle_tune
    }
    fn turn_in_place(&self) -> &Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>> {
        &self.turn_in_place
    }
    fn turn_in_place_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TurnInPlaceTuneTrait>>> {
        &mut self.turn_in_place
    }
    fn repulsion_acceleration_tune(&self) -> &Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>> {
        &self.repulsion_acceleration_tune
    }
    fn repulsion_acceleration_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn RepulsionAccelerationTuneTrait>>> {
        &mut self.repulsion_acceleration_tune
    }
    fn surface_orient_tune(&self) -> &Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>> {
        &self.surface_orient_tune
    }
    fn surface_orient_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn SurfaceOrientTuneTrait>>> {
        &mut self.surface_orient_tune
    }
    fn sidestep_fraction(&self) -> &f32 {
        &self.sidestep_fraction
    }
    fn sidestep_fraction_mut(&mut self) -> &mut f32 {
        &mut self.sidestep_fraction
    }
    fn custom_geo_match_flags(&self) -> &u32 {
        &self.custom_geo_match_flags
    }
    fn custom_geo_match_flags_mut(&mut self) -> &mut u32 {
        &mut self.custom_geo_match_flags
    }
    fn client_motion(&self) -> &bool {
        &self.client_motion
    }
    fn client_motion_mut(&mut self) -> &mut bool {
        &mut self.client_motion
    }
    fn follower_tune(&self) -> &Option<Arc<Mutex<dyn FollowerTuneTrait>>> {
        &self.follower_tune
    }
    fn follower_tune_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FollowerTuneTrait>>> {
        &mut self.follower_tune
    }
}

impl super::core::AssetTrait for MoverTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MoverTune {
}

pub static MOVERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, speed),
            },
            FieldInfoData {
                name: "maxSpeedFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, max_speed_fraction),
            },
            FieldInfoData {
                name: "radiusData",
                flags: MemberInfoFlags::new(0),
                field_type: "RadiusData",
                rust_offset: offset_of!(MoverTune, radius_data),
            },
            FieldInfoData {
                name: "bulk",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, bulk),
            },
            FieldInfoData {
                name: "cruiseAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, cruise_acc),
            },
            FieldInfoData {
                name: "startStopAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, start_stop_acc),
            },
            FieldInfoData {
                name: "repulsorType",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MoverTune, repulsor_type),
            },
            FieldInfoData {
                name: "flockAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, flock_acc),
            },
            FieldInfoData {
                name: "maxFlockAccDist",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, max_flock_acc_dist),
            },
            FieldInfoData {
                name: "pathAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, path_acc),
            },
            FieldInfoData {
                name: "cautionTune",
                flags: MemberInfoFlags::new(0),
                field_type: "CautionTune",
                rust_offset: offset_of!(MoverTune, caution_tune),
            },
            FieldInfoData {
                name: "backpedalFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, backpedal_fraction),
            },
            FieldInfoData {
                name: "planLayer",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, plan_layer),
            },
            FieldInfoData {
                name: "pathSharingPenalty",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, path_sharing_penalty),
            },
            FieldInfoData {
                name: "obstacleMode",
                flags: MemberInfoFlags::new(0),
                field_type: "BlockageMode",
                rust_offset: offset_of!(MoverTune, obstacle_mode),
            },
            FieldInfoData {
                name: "obstacleBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, obstacle_blockage_flags),
            },
            FieldInfoData {
                name: "autoObTune",
                flags: MemberInfoFlags::new(0),
                field_type: "AutoObstacleTune",
                rust_offset: offset_of!(MoverTune, auto_ob_tune),
            },
            FieldInfoData {
                name: "repulsorBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, repulsor_blockage_flags),
            },
            FieldInfoData {
                name: "repulsorIdentityFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, repulsor_identity_flags),
            },
            FieldInfoData {
                name: "linkUsageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, link_usage_flags),
            },
            FieldInfoData {
                name: "pathOptions",
                flags: MemberInfoFlags::new(0),
                field_type: "PathCreationOptions",
                rust_offset: offset_of!(MoverTune, path_options),
            },
            FieldInfoData {
                name: "jumperTune",
                flags: MemberInfoFlags::new(0),
                field_type: "JumperTune",
                rust_offset: offset_of!(MoverTune, jumper_tune),
            },
            FieldInfoData {
                name: "exitPuppetInObstacles",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverTune, exit_puppet_in_obstacles),
            },
            FieldInfoData {
                name: "proberTune",
                flags: MemberInfoFlags::new(0),
                field_type: "ProberTune",
                rust_offset: offset_of!(MoverTune, prober_tune),
            },
            FieldInfoData {
                name: "allowDetour",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverTune, allow_detour),
            },
            FieldInfoData {
                name: "goalTune",
                flags: MemberInfoFlags::new(0),
                field_type: "GoalTune",
                rust_offset: offset_of!(MoverTune, goal_tune),
            },
            FieldInfoData {
                name: "idleTune",
                flags: MemberInfoFlags::new(0),
                field_type: "IdleTune",
                rust_offset: offset_of!(MoverTune, idle_tune),
            },
            FieldInfoData {
                name: "turnInPlace",
                flags: MemberInfoFlags::new(0),
                field_type: "TurnInPlaceTune",
                rust_offset: offset_of!(MoverTune, turn_in_place),
            },
            FieldInfoData {
                name: "repulsionAccelerationTune",
                flags: MemberInfoFlags::new(0),
                field_type: "RepulsionAccelerationTune",
                rust_offset: offset_of!(MoverTune, repulsion_acceleration_tune),
            },
            FieldInfoData {
                name: "surfaceOrientTune",
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceOrientTune",
                rust_offset: offset_of!(MoverTune, surface_orient_tune),
            },
            FieldInfoData {
                name: "sidestepFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, sidestep_fraction),
            },
            FieldInfoData {
                name: "customGeoMatchFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, custom_geo_match_flags),
            },
            FieldInfoData {
                name: "clientMotion",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverTune, client_motion),
            },
            FieldInfoData {
                name: "followerTune",
                flags: MemberInfoFlags::new(0),
                field_type: "FollowerTune",
                rust_offset: offset_of!(MoverTune, follower_tune),
            },
        ],
    }),
    array_type: Some(MOVERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MoverTune {
    fn type_info(&self) -> &'static TypeInfo {
        MOVERTUNE_TYPE_INFO
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


pub static MOVERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MoverTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathCreationOptions {
    pub _glacier_base: super::core::Asset,
    pub perform_initial_nav_probe: bool,
}

pub trait PathCreationOptionsTrait: super::core::AssetTrait {
    fn perform_initial_nav_probe(&self) -> &bool;
    fn perform_initial_nav_probe_mut(&mut self) -> &mut bool;
}

impl PathCreationOptionsTrait for PathCreationOptions {
    fn perform_initial_nav_probe(&self) -> &bool {
        &self.perform_initial_nav_probe
    }
    fn perform_initial_nav_probe_mut(&mut self) -> &mut bool {
        &mut self.perform_initial_nav_probe
    }
}

impl super::core::AssetTrait for PathCreationOptions {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PathCreationOptions {
}

pub static PATHCREATIONOPTIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathCreationOptions",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathCreationOptions as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "performInitialNavProbe",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathCreationOptions, perform_initial_nav_probe),
            },
        ],
    }),
    array_type: Some(PATHCREATIONOPTIONS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PathCreationOptions {
    fn type_info(&self) -> &'static TypeInfo {
        PATHCREATIONOPTIONS_TYPE_INFO
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


pub static PATHCREATIONOPTIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathCreationOptions-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathCreationOptions"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FollowerTune {
    pub _glacier_base: super::core::Asset,
    pub circulate_enable: bool,
    pub circulate_min_time: f32,
    pub circulate_max_time: f32,
    pub startup_slowness: f32,
    pub startup_bulk: f32,
    pub packing_padding: f32,
}

pub trait FollowerTuneTrait: super::core::AssetTrait {
    fn circulate_enable(&self) -> &bool;
    fn circulate_enable_mut(&mut self) -> &mut bool;
    fn circulate_min_time(&self) -> &f32;
    fn circulate_min_time_mut(&mut self) -> &mut f32;
    fn circulate_max_time(&self) -> &f32;
    fn circulate_max_time_mut(&mut self) -> &mut f32;
    fn startup_slowness(&self) -> &f32;
    fn startup_slowness_mut(&mut self) -> &mut f32;
    fn startup_bulk(&self) -> &f32;
    fn startup_bulk_mut(&mut self) -> &mut f32;
    fn packing_padding(&self) -> &f32;
    fn packing_padding_mut(&mut self) -> &mut f32;
}

impl FollowerTuneTrait for FollowerTune {
    fn circulate_enable(&self) -> &bool {
        &self.circulate_enable
    }
    fn circulate_enable_mut(&mut self) -> &mut bool {
        &mut self.circulate_enable
    }
    fn circulate_min_time(&self) -> &f32 {
        &self.circulate_min_time
    }
    fn circulate_min_time_mut(&mut self) -> &mut f32 {
        &mut self.circulate_min_time
    }
    fn circulate_max_time(&self) -> &f32 {
        &self.circulate_max_time
    }
    fn circulate_max_time_mut(&mut self) -> &mut f32 {
        &mut self.circulate_max_time
    }
    fn startup_slowness(&self) -> &f32 {
        &self.startup_slowness
    }
    fn startup_slowness_mut(&mut self) -> &mut f32 {
        &mut self.startup_slowness
    }
    fn startup_bulk(&self) -> &f32 {
        &self.startup_bulk
    }
    fn startup_bulk_mut(&mut self) -> &mut f32 {
        &mut self.startup_bulk
    }
    fn packing_padding(&self) -> &f32 {
        &self.packing_padding
    }
    fn packing_padding_mut(&mut self) -> &mut f32 {
        &mut self.packing_padding
    }
}

impl super::core::AssetTrait for FollowerTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FollowerTune {
}

pub static FOLLOWERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowerTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "circulate_enable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FollowerTune, circulate_enable),
            },
            FieldInfoData {
                name: "circulate_minTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, circulate_min_time),
            },
            FieldInfoData {
                name: "circulate_maxTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, circulate_max_time),
            },
            FieldInfoData {
                name: "startupSlowness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, startup_slowness),
            },
            FieldInfoData {
                name: "startupBulk",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, startup_bulk),
            },
            FieldInfoData {
                name: "packingPadding",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, packing_padding),
            },
        ],
    }),
    array_type: Some(FOLLOWERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FollowerTune {
    fn type_info(&self) -> &'static TypeInfo {
        FOLLOWERTUNE_TYPE_INFO
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


pub static FOLLOWERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowerTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowerTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SurfaceOrientTune {
    pub _glacier_base: super::core::Asset,
    pub surface_orient_threshold: f32,
    pub always_vertical_on_auto_gen: bool,
    pub surface_orient_slerp_time: f32,
}

pub trait SurfaceOrientTuneTrait: super::core::AssetTrait {
    fn surface_orient_threshold(&self) -> &f32;
    fn surface_orient_threshold_mut(&mut self) -> &mut f32;
    fn always_vertical_on_auto_gen(&self) -> &bool;
    fn always_vertical_on_auto_gen_mut(&mut self) -> &mut bool;
    fn surface_orient_slerp_time(&self) -> &f32;
    fn surface_orient_slerp_time_mut(&mut self) -> &mut f32;
}

impl SurfaceOrientTuneTrait for SurfaceOrientTune {
    fn surface_orient_threshold(&self) -> &f32 {
        &self.surface_orient_threshold
    }
    fn surface_orient_threshold_mut(&mut self) -> &mut f32 {
        &mut self.surface_orient_threshold
    }
    fn always_vertical_on_auto_gen(&self) -> &bool {
        &self.always_vertical_on_auto_gen
    }
    fn always_vertical_on_auto_gen_mut(&mut self) -> &mut bool {
        &mut self.always_vertical_on_auto_gen
    }
    fn surface_orient_slerp_time(&self) -> &f32 {
        &self.surface_orient_slerp_time
    }
    fn surface_orient_slerp_time_mut(&mut self) -> &mut f32 {
        &mut self.surface_orient_slerp_time
    }
}

impl super::core::AssetTrait for SurfaceOrientTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for SurfaceOrientTune {
}

pub static SURFACEORIENTTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SurfaceOrientTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "surfaceOrientThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SurfaceOrientTune, surface_orient_threshold),
            },
            FieldInfoData {
                name: "alwaysVerticalOnAutoGen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SurfaceOrientTune, always_vertical_on_auto_gen),
            },
            FieldInfoData {
                name: "surfaceOrientSlerpTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SurfaceOrientTune, surface_orient_slerp_time),
            },
        ],
    }),
    array_type: Some(SURFACEORIENTTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SurfaceOrientTune {
    fn type_info(&self) -> &'static TypeInfo {
        SURFACEORIENTTUNE_TYPE_INFO
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


pub static SURFACEORIENTTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SurfaceOrientTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("SurfaceOrientTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct AutoObstacleTune {
    pub _glacier_base: super::core::Asset,
    pub auto_create_obstacle: bool,
    pub delay: f32,
    pub obstacle_blockage_flags: u32,
}

pub trait AutoObstacleTuneTrait: super::core::AssetTrait {
    fn auto_create_obstacle(&self) -> &bool;
    fn auto_create_obstacle_mut(&mut self) -> &mut bool;
    fn delay(&self) -> &f32;
    fn delay_mut(&mut self) -> &mut f32;
    fn obstacle_blockage_flags(&self) -> &u32;
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32;
}

impl AutoObstacleTuneTrait for AutoObstacleTune {
    fn auto_create_obstacle(&self) -> &bool {
        &self.auto_create_obstacle
    }
    fn auto_create_obstacle_mut(&mut self) -> &mut bool {
        &mut self.auto_create_obstacle
    }
    fn delay(&self) -> &f32 {
        &self.delay
    }
    fn delay_mut(&mut self) -> &mut f32 {
        &mut self.delay
    }
    fn obstacle_blockage_flags(&self) -> &u32 {
        &self.obstacle_blockage_flags
    }
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32 {
        &mut self.obstacle_blockage_flags
    }
}

impl super::core::AssetTrait for AutoObstacleTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for AutoObstacleTune {
}

pub static AUTOOBSTACLETUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoObstacleTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "autoCreateObstacle",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoObstacleTune, auto_create_obstacle),
            },
            FieldInfoData {
                name: "delay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoObstacleTune, delay),
            },
            FieldInfoData {
                name: "obstacleBlockageFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AutoObstacleTune, obstacle_blockage_flags),
            },
        ],
    }),
    array_type: Some(AUTOOBSTACLETUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AutoObstacleTune {
    fn type_info(&self) -> &'static TypeInfo {
        AUTOOBSTACLETUNE_TYPE_INFO
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


pub static AUTOOBSTACLETUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AutoObstacleTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("AutoObstacleTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IdleTune {
    pub _glacier_base: super::core::Asset,
    pub tether_dist: f32,
    pub return_delay: f32,
}

pub trait IdleTuneTrait: super::core::AssetTrait {
    fn tether_dist(&self) -> &f32;
    fn tether_dist_mut(&mut self) -> &mut f32;
    fn return_delay(&self) -> &f32;
    fn return_delay_mut(&mut self) -> &mut f32;
}

impl IdleTuneTrait for IdleTune {
    fn tether_dist(&self) -> &f32 {
        &self.tether_dist
    }
    fn tether_dist_mut(&mut self) -> &mut f32 {
        &mut self.tether_dist
    }
    fn return_delay(&self) -> &f32 {
        &self.return_delay
    }
    fn return_delay_mut(&mut self) -> &mut f32 {
        &mut self.return_delay
    }
}

impl super::core::AssetTrait for IdleTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for IdleTune {
}

pub static IDLETUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IdleTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "tetherDist",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IdleTune, tether_dist),
            },
            FieldInfoData {
                name: "returnDelay",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IdleTune, return_delay),
            },
        ],
    }),
    array_type: Some(IDLETUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IdleTune {
    fn type_info(&self) -> &'static TypeInfo {
        IDLETUNE_TYPE_INFO
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


pub static IDLETUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IdleTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("IdleTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GoalTune {
    pub _glacier_base: super::core::Asset,
    pub use_circular_approach: bool,
    pub preferred_turning_radius: f32,
}

pub trait GoalTuneTrait: super::core::AssetTrait {
    fn use_circular_approach(&self) -> &bool;
    fn use_circular_approach_mut(&mut self) -> &mut bool;
    fn preferred_turning_radius(&self) -> &f32;
    fn preferred_turning_radius_mut(&mut self) -> &mut f32;
}

impl GoalTuneTrait for GoalTune {
    fn use_circular_approach(&self) -> &bool {
        &self.use_circular_approach
    }
    fn use_circular_approach_mut(&mut self) -> &mut bool {
        &mut self.use_circular_approach
    }
    fn preferred_turning_radius(&self) -> &f32 {
        &self.preferred_turning_radius
    }
    fn preferred_turning_radius_mut(&mut self) -> &mut f32 {
        &mut self.preferred_turning_radius
    }
}

impl super::core::AssetTrait for GoalTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GoalTune {
}

pub static GOALTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GoalTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "useCircularApproach",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GoalTune, use_circular_approach),
            },
            FieldInfoData {
                name: "preferredTurningRadius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GoalTune, preferred_turning_radius),
            },
        ],
    }),
    array_type: Some(GOALTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GoalTune {
    fn type_info(&self) -> &'static TypeInfo {
        GOALTUNE_TYPE_INFO
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


pub static GOALTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GoalTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GoalTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProberTune {
    pub _glacier_base: super::core::Asset,
    pub probe_for_ground: bool,
    pub probe_interval: f32,
}

pub trait ProberTuneTrait: super::core::AssetTrait {
    fn probe_for_ground(&self) -> &bool;
    fn probe_for_ground_mut(&mut self) -> &mut bool;
    fn probe_interval(&self) -> &f32;
    fn probe_interval_mut(&mut self) -> &mut f32;
}

impl ProberTuneTrait for ProberTune {
    fn probe_for_ground(&self) -> &bool {
        &self.probe_for_ground
    }
    fn probe_for_ground_mut(&mut self) -> &mut bool {
        &mut self.probe_for_ground
    }
    fn probe_interval(&self) -> &f32 {
        &self.probe_interval
    }
    fn probe_interval_mut(&mut self) -> &mut f32 {
        &mut self.probe_interval
    }
}

impl super::core::AssetTrait for ProberTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ProberTune {
}

pub static PROBERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProberTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "probeForGround",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProberTune, probe_for_ground),
            },
            FieldInfoData {
                name: "probeInterval",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ProberTune, probe_interval),
            },
        ],
    }),
    array_type: Some(PROBERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProberTune {
    fn type_info(&self) -> &'static TypeInfo {
        PROBERTUNE_TYPE_INFO
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


pub static PROBERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProberTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ProberTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct JumperTune {
    pub _glacier_base: super::core::Asset,
    pub speed: f32,
    pub arc_fraction: f32,
    pub turn_before_jump_angle: f32,
    pub keep_speed_when_swap_to_default: bool,
    pub only_jump_to_end_point: bool,
}

pub trait JumperTuneTrait: super::core::AssetTrait {
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn arc_fraction(&self) -> &f32;
    fn arc_fraction_mut(&mut self) -> &mut f32;
    fn turn_before_jump_angle(&self) -> &f32;
    fn turn_before_jump_angle_mut(&mut self) -> &mut f32;
    fn keep_speed_when_swap_to_default(&self) -> &bool;
    fn keep_speed_when_swap_to_default_mut(&mut self) -> &mut bool;
    fn only_jump_to_end_point(&self) -> &bool;
    fn only_jump_to_end_point_mut(&mut self) -> &mut bool;
}

impl JumperTuneTrait for JumperTune {
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }
    fn arc_fraction(&self) -> &f32 {
        &self.arc_fraction
    }
    fn arc_fraction_mut(&mut self) -> &mut f32 {
        &mut self.arc_fraction
    }
    fn turn_before_jump_angle(&self) -> &f32 {
        &self.turn_before_jump_angle
    }
    fn turn_before_jump_angle_mut(&mut self) -> &mut f32 {
        &mut self.turn_before_jump_angle
    }
    fn keep_speed_when_swap_to_default(&self) -> &bool {
        &self.keep_speed_when_swap_to_default
    }
    fn keep_speed_when_swap_to_default_mut(&mut self) -> &mut bool {
        &mut self.keep_speed_when_swap_to_default
    }
    fn only_jump_to_end_point(&self) -> &bool {
        &self.only_jump_to_end_point
    }
    fn only_jump_to_end_point_mut(&mut self) -> &mut bool {
        &mut self.only_jump_to_end_point
    }
}

impl super::core::AssetTrait for JumperTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for JumperTune {
}

pub static JUMPERTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JumperTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumperTune, speed),
            },
            FieldInfoData {
                name: "arcFraction",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumperTune, arc_fraction),
            },
            FieldInfoData {
                name: "turnBeforeJumpAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumperTune, turn_before_jump_angle),
            },
            FieldInfoData {
                name: "keepSpeedWhenSwapToDefault",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JumperTune, keep_speed_when_swap_to_default),
            },
            FieldInfoData {
                name: "onlyJumpToEndPoint",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JumperTune, only_jump_to_end_point),
            },
        ],
    }),
    array_type: Some(JUMPERTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for JumperTune {
    fn type_info(&self) -> &'static TypeInfo {
        JUMPERTUNE_TYPE_INFO
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


pub static JUMPERTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumperTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("JumperTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CautionTune {
    pub _glacier_base: super::core::Asset,
    pub speed_x: f32,
    pub tight_turn_degrees: f32,
}

pub trait CautionTuneTrait: super::core::AssetTrait {
    fn speed_x(&self) -> &f32;
    fn speed_x_mut(&mut self) -> &mut f32;
    fn tight_turn_degrees(&self) -> &f32;
    fn tight_turn_degrees_mut(&mut self) -> &mut f32;
}

impl CautionTuneTrait for CautionTune {
    fn speed_x(&self) -> &f32 {
        &self.speed_x
    }
    fn speed_x_mut(&mut self) -> &mut f32 {
        &mut self.speed_x
    }
    fn tight_turn_degrees(&self) -> &f32 {
        &self.tight_turn_degrees
    }
    fn tight_turn_degrees_mut(&mut self) -> &mut f32 {
        &mut self.tight_turn_degrees
    }
}

impl super::core::AssetTrait for CautionTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for CautionTune {
}

pub static CAUTIONTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CautionTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "speedX",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CautionTune, speed_x),
            },
            FieldInfoData {
                name: "tightTurnDegrees",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CautionTune, tight_turn_degrees),
            },
        ],
    }),
    array_type: Some(CAUTIONTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CautionTune {
    fn type_info(&self) -> &'static TypeInfo {
        CAUTIONTUNE_TYPE_INFO
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


pub static CAUTIONTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CautionTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CautionTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RepulsionAccelerationTune {
    pub _glacier_base: super::core::Asset,
    pub initial_acc: f32,
    pub outer_cushion_acc: f32,
    pub inner_cushion_acc: f32,
}

pub trait RepulsionAccelerationTuneTrait: super::core::AssetTrait {
    fn initial_acc(&self) -> &f32;
    fn initial_acc_mut(&mut self) -> &mut f32;
    fn outer_cushion_acc(&self) -> &f32;
    fn outer_cushion_acc_mut(&mut self) -> &mut f32;
    fn inner_cushion_acc(&self) -> &f32;
    fn inner_cushion_acc_mut(&mut self) -> &mut f32;
}

impl RepulsionAccelerationTuneTrait for RepulsionAccelerationTune {
    fn initial_acc(&self) -> &f32 {
        &self.initial_acc
    }
    fn initial_acc_mut(&mut self) -> &mut f32 {
        &mut self.initial_acc
    }
    fn outer_cushion_acc(&self) -> &f32 {
        &self.outer_cushion_acc
    }
    fn outer_cushion_acc_mut(&mut self) -> &mut f32 {
        &mut self.outer_cushion_acc
    }
    fn inner_cushion_acc(&self) -> &f32 {
        &self.inner_cushion_acc
    }
    fn inner_cushion_acc_mut(&mut self) -> &mut f32 {
        &mut self.inner_cushion_acc
    }
}

impl super::core::AssetTrait for RepulsionAccelerationTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RepulsionAccelerationTune {
}

pub static REPULSIONACCELERATIONTUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RepulsionAccelerationTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "initialAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RepulsionAccelerationTune, initial_acc),
            },
            FieldInfoData {
                name: "outerCushionAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RepulsionAccelerationTune, outer_cushion_acc),
            },
            FieldInfoData {
                name: "innerCushionAcc",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RepulsionAccelerationTune, inner_cushion_acc),
            },
        ],
    }),
    array_type: Some(REPULSIONACCELERATIONTUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RepulsionAccelerationTune {
    fn type_info(&self) -> &'static TypeInfo {
        REPULSIONACCELERATIONTUNE_TYPE_INFO
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


pub static REPULSIONACCELERATIONTUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RepulsionAccelerationTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RepulsionAccelerationTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TurnInPlaceTune {
    pub _glacier_base: super::core::Asset,
    pub when_moving_angle: f32,
    pub when_stopped_angle: f32,
    pub speed: f32,
    pub accel_angle: f32,
    pub slide_spin_threshold: f32,
    pub enable_u_turn: bool,
}

pub trait TurnInPlaceTuneTrait: super::core::AssetTrait {
    fn when_moving_angle(&self) -> &f32;
    fn when_moving_angle_mut(&mut self) -> &mut f32;
    fn when_stopped_angle(&self) -> &f32;
    fn when_stopped_angle_mut(&mut self) -> &mut f32;
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn accel_angle(&self) -> &f32;
    fn accel_angle_mut(&mut self) -> &mut f32;
    fn slide_spin_threshold(&self) -> &f32;
    fn slide_spin_threshold_mut(&mut self) -> &mut f32;
    fn enable_u_turn(&self) -> &bool;
    fn enable_u_turn_mut(&mut self) -> &mut bool;
}

impl TurnInPlaceTuneTrait for TurnInPlaceTune {
    fn when_moving_angle(&self) -> &f32 {
        &self.when_moving_angle
    }
    fn when_moving_angle_mut(&mut self) -> &mut f32 {
        &mut self.when_moving_angle
    }
    fn when_stopped_angle(&self) -> &f32 {
        &self.when_stopped_angle
    }
    fn when_stopped_angle_mut(&mut self) -> &mut f32 {
        &mut self.when_stopped_angle
    }
    fn speed(&self) -> &f32 {
        &self.speed
    }
    fn speed_mut(&mut self) -> &mut f32 {
        &mut self.speed
    }
    fn accel_angle(&self) -> &f32 {
        &self.accel_angle
    }
    fn accel_angle_mut(&mut self) -> &mut f32 {
        &mut self.accel_angle
    }
    fn slide_spin_threshold(&self) -> &f32 {
        &self.slide_spin_threshold
    }
    fn slide_spin_threshold_mut(&mut self) -> &mut f32 {
        &mut self.slide_spin_threshold
    }
    fn enable_u_turn(&self) -> &bool {
        &self.enable_u_turn
    }
    fn enable_u_turn_mut(&mut self) -> &mut bool {
        &mut self.enable_u_turn
    }
}

impl super::core::AssetTrait for TurnInPlaceTune {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for TurnInPlaceTune {
}

pub static TURNINPLACETUNE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTune",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurnInPlaceTune as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "whenMovingAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, when_moving_angle),
            },
            FieldInfoData {
                name: "whenStoppedAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, when_stopped_angle),
            },
            FieldInfoData {
                name: "speed",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, speed),
            },
            FieldInfoData {
                name: "accelAngle",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, accel_angle),
            },
            FieldInfoData {
                name: "slideSpinThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, slide_spin_threshold),
            },
            FieldInfoData {
                name: "enableUTurn",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TurnInPlaceTune, enable_u_turn),
            },
        ],
    }),
    array_type: Some(TURNINPLACETUNE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TurnInPlaceTune {
    fn type_info(&self) -> &'static TypeInfo {
        TURNINPLACETUNE_TYPE_INFO
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


pub static TURNINPLACETUNE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TurnInPlaceTune-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("TurnInPlaceTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RadiusData {
    pub _glacier_base: super::core::Asset,
    pub radius: f32,
    pub outer_cushion: f32,
    pub inner_cushion: f32,
}

pub trait RadiusDataTrait: super::core::AssetTrait {
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn outer_cushion(&self) -> &f32;
    fn outer_cushion_mut(&mut self) -> &mut f32;
    fn inner_cushion(&self) -> &f32;
    fn inner_cushion_mut(&mut self) -> &mut f32;
}

impl RadiusDataTrait for RadiusData {
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn outer_cushion(&self) -> &f32 {
        &self.outer_cushion
    }
    fn outer_cushion_mut(&mut self) -> &mut f32 {
        &mut self.outer_cushion
    }
    fn inner_cushion(&self) -> &f32 {
        &self.inner_cushion
    }
    fn inner_cushion_mut(&mut self) -> &mut f32 {
        &mut self.inner_cushion
    }
}

impl super::core::AssetTrait for RadiusData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for RadiusData {
}

pub static RADIUSDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusData",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiusData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiusData, radius),
            },
            FieldInfoData {
                name: "outerCushion",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiusData, outer_cushion),
            },
            FieldInfoData {
                name: "innerCushion",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiusData, inner_cushion),
            },
        ],
    }),
    array_type: Some(RADIUSDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RadiusData {
    fn type_info(&self) -> &'static TypeInfo {
        RADIUSDATA_TYPE_INFO
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


pub static RADIUSDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RadiusData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RadiusData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum BlockageMode {
    #[default]
    BLOCKED_IF_ANY_MATCH = 0,
    BLOCKED_IF_ALL_MATCH = 1,
}

pub static BLOCKAGEMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockageMode",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(BLOCKAGEMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for BlockageMode {
    fn type_info(&self) -> &'static TypeInfo {
        BLOCKAGEMODE_TYPE_INFO
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


pub static BLOCKAGEMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BlockageMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("BlockageMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OrientMode {
    #[default]
    ORIENT_IN_TRAVEL_DIR = 0,
    ORIENT_STRICTLY_IN_TRAVEL_DIR = 1,
    ORIENT_TARGET = 2,
    ORIENT_IN_DIR = 3,
}

pub static ORIENTMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrientMode",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(ORIENTMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OrientMode {
    fn type_info(&self) -> &'static TypeInfo {
        ORIENTMODE_TYPE_INFO
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


pub static ORIENTMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OrientMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("OrientMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FollowMoverSpec {
    pub formation: FollowFormation,
    pub follow_distance: f32,
    pub arc_spread: f32,
}

pub trait FollowMoverSpecTrait: TypeObject {
    fn formation(&self) -> &FollowFormation;
    fn formation_mut(&mut self) -> &mut FollowFormation;
    fn follow_distance(&self) -> &f32;
    fn follow_distance_mut(&mut self) -> &mut f32;
    fn arc_spread(&self) -> &f32;
    fn arc_spread_mut(&mut self) -> &mut f32;
}

impl FollowMoverSpecTrait for FollowMoverSpec {
    fn formation(&self) -> &FollowFormation {
        &self.formation
    }
    fn formation_mut(&mut self) -> &mut FollowFormation {
        &mut self.formation
    }
    fn follow_distance(&self) -> &f32 {
        &self.follow_distance
    }
    fn follow_distance_mut(&mut self) -> &mut f32 {
        &mut self.follow_distance
    }
    fn arc_spread(&self) -> &f32 {
        &self.arc_spread
    }
    fn arc_spread_mut(&mut self) -> &mut f32 {
        &mut self.arc_spread
    }
}

pub static FOLLOWMOVERSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowMoverSpec",
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowMoverSpec as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "formation",
                flags: MemberInfoFlags::new(0),
                field_type: "FollowFormation",
                rust_offset: offset_of!(FollowMoverSpec, formation),
            },
            FieldInfoData {
                name: "followDistance",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowMoverSpec, follow_distance),
            },
            FieldInfoData {
                name: "arcSpread",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowMoverSpec, arc_spread),
            },
        ],
    }),
    array_type: Some(FOLLOWMOVERSPEC_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for FollowMoverSpec {
    fn type_info(&self) -> &'static TypeInfo {
        FOLLOWMOVERSPEC_TYPE_INFO
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


pub static FOLLOWMOVERSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowMoverSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowMoverSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum FollowFormation {
    #[default]
    FORMATION_CIRCLE = 0,
    FORMATION_BLOB = 1,
}

pub static FOLLOWFORMATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowFormation",
    flags: MemberInfoFlags::new(49429),
    module: "Pathfinding",
    data: TypeInfoData::Enum,
    array_type: Some(FOLLOWFORMATION_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for FollowFormation {
    fn type_info(&self) -> &'static TypeInfo {
        FOLLOWFORMATION_TYPE_INFO
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


pub static FOLLOWFORMATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FollowFormation-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowFormation"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StopSpec {
    pub stop_immediately: bool,
}

pub trait StopSpecTrait: TypeObject {
    fn stop_immediately(&self) -> &bool;
    fn stop_immediately_mut(&mut self) -> &mut bool;
}

impl StopSpecTrait for StopSpec {
    fn stop_immediately(&self) -> &bool {
        &self.stop_immediately
    }
    fn stop_immediately_mut(&mut self) -> &mut bool {
        &mut self.stop_immediately
    }
}

pub static STOPSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSpec",
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StopSpec as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "stopImmediately",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StopSpec, stop_immediately),
            },
        ],
    }),
    array_type: Some(STOPSPEC_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StopSpec {
    fn type_info(&self) -> &'static TypeInfo {
        STOPSPEC_TYPE_INFO
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


pub static STOPSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StopSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("StopSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GotoPosSpec {
    pub allowed_to_stop_dist: f32,
    pub desired_stop_dist: f32,
    pub stop_at_goal: bool,
    pub push_through_crowd_at_goal: bool,
    pub orient_at_goal_enable: bool,
    pub orient_at_goal_dir: super::core::Vec3,
    pub try_flank: bool,
}

pub trait GotoPosSpecTrait: TypeObject {
    fn allowed_to_stop_dist(&self) -> &f32;
    fn allowed_to_stop_dist_mut(&mut self) -> &mut f32;
    fn desired_stop_dist(&self) -> &f32;
    fn desired_stop_dist_mut(&mut self) -> &mut f32;
    fn stop_at_goal(&self) -> &bool;
    fn stop_at_goal_mut(&mut self) -> &mut bool;
    fn push_through_crowd_at_goal(&self) -> &bool;
    fn push_through_crowd_at_goal_mut(&mut self) -> &mut bool;
    fn orient_at_goal_enable(&self) -> &bool;
    fn orient_at_goal_enable_mut(&mut self) -> &mut bool;
    fn orient_at_goal_dir(&self) -> &super::core::Vec3;
    fn orient_at_goal_dir_mut(&mut self) -> &mut super::core::Vec3;
    fn try_flank(&self) -> &bool;
    fn try_flank_mut(&mut self) -> &mut bool;
}

impl GotoPosSpecTrait for GotoPosSpec {
    fn allowed_to_stop_dist(&self) -> &f32 {
        &self.allowed_to_stop_dist
    }
    fn allowed_to_stop_dist_mut(&mut self) -> &mut f32 {
        &mut self.allowed_to_stop_dist
    }
    fn desired_stop_dist(&self) -> &f32 {
        &self.desired_stop_dist
    }
    fn desired_stop_dist_mut(&mut self) -> &mut f32 {
        &mut self.desired_stop_dist
    }
    fn stop_at_goal(&self) -> &bool {
        &self.stop_at_goal
    }
    fn stop_at_goal_mut(&mut self) -> &mut bool {
        &mut self.stop_at_goal
    }
    fn push_through_crowd_at_goal(&self) -> &bool {
        &self.push_through_crowd_at_goal
    }
    fn push_through_crowd_at_goal_mut(&mut self) -> &mut bool {
        &mut self.push_through_crowd_at_goal
    }
    fn orient_at_goal_enable(&self) -> &bool {
        &self.orient_at_goal_enable
    }
    fn orient_at_goal_enable_mut(&mut self) -> &mut bool {
        &mut self.orient_at_goal_enable
    }
    fn orient_at_goal_dir(&self) -> &super::core::Vec3 {
        &self.orient_at_goal_dir
    }
    fn orient_at_goal_dir_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.orient_at_goal_dir
    }
    fn try_flank(&self) -> &bool {
        &self.try_flank
    }
    fn try_flank_mut(&mut self) -> &mut bool {
        &mut self.try_flank
    }
}

pub static GOTOPOSSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GotoPosSpec",
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GotoPosSpec as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "allowedToStopDist",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GotoPosSpec, allowed_to_stop_dist),
            },
            FieldInfoData {
                name: "desiredStopDist",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GotoPosSpec, desired_stop_dist),
            },
            FieldInfoData {
                name: "stopAtGoal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, stop_at_goal),
            },
            FieldInfoData {
                name: "pushThroughCrowdAtGoal",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, push_through_crowd_at_goal),
            },
            FieldInfoData {
                name: "orientAtGoalEnable",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, orient_at_goal_enable),
            },
            FieldInfoData {
                name: "orientAtGoalDir",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(GotoPosSpec, orient_at_goal_dir),
            },
            FieldInfoData {
                name: "tryFlank",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, try_flank),
            },
        ],
    }),
    array_type: Some(GOTOPOSSPEC_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for GotoPosSpec {
    fn type_info(&self) -> &'static TypeInfo {
        GOTOPOSSPEC_TYPE_INFO
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


pub static GOTOPOSSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GotoPosSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GotoPosSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathSpec {
}

pub trait PathSpecTrait: TypeObject {
}

impl PathSpecTrait for PathSpec {
}

pub static PATHSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathSpec",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathSpec as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PATHSPEC_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathSpec {
    fn type_info(&self) -> &'static TypeInfo {
        PATHSPEC_TYPE_INFO
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


pub static PATHSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PathfindingRuntimeResource {
}

pub trait PathfindingRuntimeResourceTrait: TypeObject {
}

impl PathfindingRuntimeResourceTrait for PathfindingRuntimeResource {
}

pub static PATHFINDINGRUNTIMERESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResource",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingRuntimeResource as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PATHFINDINGRUNTIMERESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PathfindingRuntimeResource {
    fn type_info(&self) -> &'static TypeInfo {
        PATHFINDINGRUNTIMERESOURCE_TYPE_INFO
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


pub static PATHFINDINGRUNTIMERESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathfindingRuntimeResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerObstacleControllerEntity {
    pub _glacier_base: ObstacleControllerEntity,
}

pub trait ServerObstacleControllerEntityTrait: ObstacleControllerEntityTrait {
}

impl ServerObstacleControllerEntityTrait for ServerObstacleControllerEntity {
}

impl ObstacleControllerEntityTrait for ServerObstacleControllerEntity {
}

impl super::entity::EntityTrait for ServerObstacleControllerEntity {
}

impl super::entity::EntityBusPeerTrait for ServerObstacleControllerEntity {
}

pub static SERVEROBSTACLECONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObstacleControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBSTACLECONTROLLERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerObstacleControllerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVEROBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerObstacleControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVEROBSTACLECONTROLLERENTITY_TYPE_INFO
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


pub static SERVEROBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerObstacleControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ServerObstacleControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ServerNavPowerSystemEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ServerNavPowerSystemEntityTrait: super::entity::EntityTrait {
}

impl ServerNavPowerSystemEntityTrait for ServerNavPowerSystemEntity {
}

impl super::entity::EntityTrait for ServerNavPowerSystemEntity {
}

impl super::entity::EntityBusPeerTrait for ServerNavPowerSystemEntity {
}

pub static SERVERNAVPOWERSYSTEMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerSystemEntity",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerNavPowerSystemEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SERVERNAVPOWERSYSTEMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ServerNavPowerSystemEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SERVERNAVPOWERSYSTEMENTITY_TYPE_INFO
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


pub static SERVERNAVPOWERSYSTEMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ServerNavPowerSystemEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ServerNavPowerSystemEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObstacleControllerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ObstacleControllerEntityTrait: super::entity::EntityTrait {
}

impl ObstacleControllerEntityTrait for ObstacleControllerEntity {
}

impl super::entity::EntityTrait for ObstacleControllerEntity {
}

impl super::entity::EntityBusPeerTrait for ObstacleControllerEntity {
}

pub static OBSTACLECONTROLLERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntity",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObstacleControllerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObstacleControllerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        OBSTACLECONTROLLERENTITY_TYPE_INFO
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


pub static OBSTACLECONTROLLERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObstacleControllerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct NavPowerPathSpec {
    pub _glacier_base: PathSpec,
}

pub trait NavPowerPathSpecTrait: PathSpecTrait {
}

impl NavPowerPathSpecTrait for NavPowerPathSpec {
}

impl PathSpecTrait for NavPowerPathSpec {
}

pub static NAVPOWERPATHSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavPowerPathSpec",
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PATHSPEC_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NavPowerPathSpec as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(NAVPOWERPATHSPEC_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for NavPowerPathSpec {
    fn type_info(&self) -> &'static TypeInfo {
        NAVPOWERPATHSPEC_TYPE_INFO
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


pub static NAVPOWERPATHSPEC_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "NavPowerPathSpec-Array",
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("NavPowerPathSpec"),
    array_type: None,
    alignment: 8,
};


