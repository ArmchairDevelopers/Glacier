use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct ObstacleControllerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub active_at_start: bool,
    pub obstacle_data: Option<LockedTypeObject /* ObstacleDat */>,
}

pub trait ObstacleControllerEntityDataTrait: super::entity::EntityDataTrait {
    fn active_at_start(&self) -> &bool;
    fn active_at_start_mut(&mut self) -> &mut bool;
    fn obstacle_data(&self) -> &Option<LockedTypeObject /* ObstacleDat */>;
    fn obstacle_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ObstacleDat */>;
}

impl ObstacleControllerEntityDataTrait for ObstacleControllerEntityData {
    fn active_at_start(&self) -> &bool {
        &self.active_at_start
    }
    fn active_at_start_mut(&mut self) -> &mut bool {
        &mut self.active_at_start
    }
    fn obstacle_data(&self) -> &Option<LockedTypeObject /* ObstacleDat */> {
        &self.obstacle_data
    }
    fn obstacle_data_mut(&mut self) -> &mut Option<LockedTypeObject /* ObstacleDat */> {
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
    name_hash: 4074285907,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(ObstacleControllerEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObstacleControllerEntityData as Default>::default())),
            create_boxed: || Box::new(<ObstacleControllerEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ActiveAtStart",
                name_hash: 3222823772,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ObstacleControllerEntityData, active_at_start),
            },
            FieldInfoData {
                name: "ObstacleData",
                name_hash: 3091927988,
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
    name_hash: 2598409831,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleControllerEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingNavMeshVolumeData {
    pub _glacier_base: super::entity::OBBData,
    pub category: Option<LockedTypeObject /* super::entity::PathfindingObjectCategoryAsset */>,
}

pub trait PathfindingNavMeshVolumeDataTrait: super::entity::OBBDataTrait {
    fn category(&self) -> &Option<LockedTypeObject /* super::entity::PathfindingObjectCategoryAsset */>;
    fn category_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PathfindingObjectCategoryAsset */>;
}

impl PathfindingNavMeshVolumeDataTrait for PathfindingNavMeshVolumeData {
    fn category(&self) -> &Option<LockedTypeObject /* super::entity::PathfindingObjectCategoryAsset */> {
        &self.category
    }
    fn category_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::PathfindingObjectCategoryAsset */> {
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
    name_hash: 313278143,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBBDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathfindingNavMeshVolumeData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingNavMeshVolumeData as Default>::default())),
            create_boxed: || Box::new(<PathfindingNavMeshVolumeData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Category",
                name_hash: 3455858997,
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
    name_hash: 2501948427,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathfindingNavMeshVolumeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ObstacleDat {
    pub _glacier_base: super::core::DataContainer,
    pub layer_mask: u32,
    pub penalty_mult: f32,
    pub obstacle_blockage_flags: u32,
    pub user_data: Option<LockedTypeObject /* CustomObstacleData */>,
    pub obstacle_name: String,
}

pub trait ObstacleDatTrait: super::core::DataContainerTrait {
    fn layer_mask(&self) -> &u32;
    fn layer_mask_mut(&mut self) -> &mut u32;
    fn penalty_mult(&self) -> &f32;
    fn penalty_mult_mut(&mut self) -> &mut f32;
    fn obstacle_blockage_flags(&self) -> &u32;
    fn obstacle_blockage_flags_mut(&mut self) -> &mut u32;
    fn user_data(&self) -> &Option<LockedTypeObject /* CustomObstacleData */>;
    fn user_data_mut(&mut self) -> &mut Option<LockedTypeObject /* CustomObstacleData */>;
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
    fn user_data(&self) -> &Option<LockedTypeObject /* CustomObstacleData */> {
        &self.user_data
    }
    fn user_data_mut(&mut self) -> &mut Option<LockedTypeObject /* CustomObstacleData */> {
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
    name_hash: 614296885,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(ObstacleDat, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObstacleDat as Default>::default())),
            create_boxed: || Box::new(<ObstacleDat as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LayerMask",
                name_hash: 3033747218,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ObstacleDat, layer_mask),
            },
            FieldInfoData {
                name: "PenaltyMult",
                name_hash: 5088702,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ObstacleDat, penalty_mult),
            },
            FieldInfoData {
                name: "ObstacleBlockageFlags",
                name_hash: 4018582833,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ObstacleDat, obstacle_blockage_flags),
            },
            FieldInfoData {
                name: "UserData",
                name_hash: 307929860,
                flags: MemberInfoFlags::new(0),
                field_type: "CustomObstacleData",
                rust_offset: offset_of!(ObstacleDat, user_data),
            },
            FieldInfoData {
                name: "ObstacleName",
                name_hash: 3092425859,
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
    name_hash: 790818177,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleDat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3582537383,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        super_class_offset: offset_of!(CustomObstacleData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomObstacleData as Default>::default())),
            create_boxed: || Box::new(<CustomObstacleData as Default>::default()),
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
    name_hash: 2480898067,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CustomObstacleData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathLinkEntityData {
    pub _glacier_base: super::entity::SpatialEntityData,
    pub points: Vec<BoxedTypeObject /* super::core::Vec3 */>,
    pub direction: PathLinkDirection,
    pub link_dat: Option<LockedTypeObject /* super::pathfinding_shared::LinkDat */>,
    pub active_at_start: bool,
    pub deferred_creation: bool,
}

pub trait PathLinkEntityDataTrait: super::entity::SpatialEntityDataTrait {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */>;
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */>;
    fn direction(&self) -> &PathLinkDirection;
    fn direction_mut(&mut self) -> &mut PathLinkDirection;
    fn link_dat(&self) -> &Option<LockedTypeObject /* super::pathfinding_shared::LinkDat */>;
    fn link_dat_mut(&mut self) -> &mut Option<LockedTypeObject /* super::pathfinding_shared::LinkDat */>;
    fn active_at_start(&self) -> &bool;
    fn active_at_start_mut(&mut self) -> &mut bool;
    fn deferred_creation(&self) -> &bool;
    fn deferred_creation_mut(&mut self) -> &mut bool;
}

impl PathLinkEntityDataTrait for PathLinkEntityData {
    fn points(&self) -> &Vec<BoxedTypeObject /* super::core::Vec3 */> {
        &self.points
    }
    fn points_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::Vec3 */> {
        &mut self.points
    }
    fn direction(&self) -> &PathLinkDirection {
        &self.direction
    }
    fn direction_mut(&mut self) -> &mut PathLinkDirection {
        &mut self.direction
    }
    fn link_dat(&self) -> &Option<LockedTypeObject /* super::pathfinding_shared::LinkDat */> {
        &self.link_dat
    }
    fn link_dat_mut(&mut self) -> &mut Option<LockedTypeObject /* super::pathfinding_shared::LinkDat */> {
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
    name_hash: 3540157539,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::SPATIALENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(PathLinkEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathLinkEntityData as Default>::default())),
            create_boxed: || Box::new(<PathLinkEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Points",
                name_hash: 3383606106,
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(PathLinkEntityData, points),
            },
            FieldInfoData {
                name: "Direction",
                name_hash: 2698949952,
                flags: MemberInfoFlags::new(0),
                field_type: "PathLinkDirection",
                rust_offset: offset_of!(PathLinkEntityData, direction),
            },
            FieldInfoData {
                name: "LinkDat",
                name_hash: 994385204,
                flags: MemberInfoFlags::new(0),
                field_type: "LinkDat",
                rust_offset: offset_of!(PathLinkEntityData, link_dat),
            },
            FieldInfoData {
                name: "ActiveAtStart",
                name_hash: 3222823772,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PathLinkEntityData, active_at_start),
            },
            FieldInfoData {
                name: "DeferredCreation",
                name_hash: 1362187151,
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
    name_hash: 3245766743,
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
    name_hash: 2783877997,
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
    name_hash: 1525325913,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathLinkDirection"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1654086183,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MoverFollowLeaderEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverFollowLeaderEntityData as Default>::default())),
            create_boxed: || Box::new(<MoverFollowLeaderEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "FollowingParameters",
                name_hash: 3970002222,
                flags: MemberInfoFlags::new(0),
                field_type: "FollowMoverSpec",
                rust_offset: offset_of!(MoverFollowLeaderEntityData, following_parameters),
            },
            FieldInfoData {
                name: "FlockId",
                name_hash: 2134404357,
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
    name_hash: 3099610003,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverFollowLeaderEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 949120492,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MoverFollowWaypointsEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverFollowWaypointsEntityData as Default>::default())),
            create_boxed: || Box::new(<MoverFollowWaypointsEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TypeOfRoute",
                name_hash: 2152665933,
                flags: MemberInfoFlags::new(0),
                field_type: "RouteType",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, type_of_route),
            },
            FieldInfoData {
                name: "StopAtWaypoints",
                name_hash: 955850104,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, stop_at_waypoints),
            },
            FieldInfoData {
                name: "StartAtGeometricallyClosestWaypoint",
                name_hash: 4268018707,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, start_at_geometrically_closest_waypoint),
            },
            FieldInfoData {
                name: "IntermediateAllowedStopDistOverride",
                name_hash: 3157963210,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, intermediate_allowed_stop_dist_override),
            },
            FieldInfoData {
                name: "DestinationAllowedStopDistOverride",
                name_hash: 727976711,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverFollowWaypointsEntityData, destination_allowed_stop_dist_override),
            },
            FieldInfoData {
                name: "DestinationSetOrientation",
                name_hash: 1725856333,
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
    name_hash: 4158049752,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverFollowWaypointsEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MoverComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub r#type: EntityMoverType,
    pub mover_tune: Option<LockedTypeObject /* MoverTune */>,
    pub goal_plan_failure_treshold: f32,
    pub goal_height_failure_treshold: f32,
    pub radius_data: Option<LockedTypeObject /* RadiusData */>,
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
    fn mover_tune(&self) -> &Option<LockedTypeObject /* MoverTune */>;
    fn mover_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* MoverTune */>;
    fn goal_plan_failure_treshold(&self) -> &f32;
    fn goal_plan_failure_treshold_mut(&mut self) -> &mut f32;
    fn goal_height_failure_treshold(&self) -> &f32;
    fn goal_height_failure_treshold_mut(&mut self) -> &mut f32;
    fn radius_data(&self) -> &Option<LockedTypeObject /* RadiusData */>;
    fn radius_data_mut(&mut self) -> &mut Option<LockedTypeObject /* RadiusData */>;
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
    fn mover_tune(&self) -> &Option<LockedTypeObject /* MoverTune */> {
        &self.mover_tune
    }
    fn mover_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* MoverTune */> {
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
    fn radius_data(&self) -> &Option<LockedTypeObject /* RadiusData */> {
        &self.radius_data
    }
    fn radius_data_mut(&mut self) -> &mut Option<LockedTypeObject /* RadiusData */> {
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
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
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
    name_hash: 244471929,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        super_class_offset: offset_of!(MoverComponentData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverComponentData as Default>::default())),
            create_boxed: || Box::new(<MoverComponentData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "type",
                name_hash: 2087944093,
                flags: MemberInfoFlags::new(0),
                field_type: "EntityMoverType",
                rust_offset: offset_of!(MoverComponentData, r#type),
            },
            FieldInfoData {
                name: "moverTune",
                name_hash: 692827628,
                flags: MemberInfoFlags::new(0),
                field_type: "MoverTune",
                rust_offset: offset_of!(MoverComponentData, mover_tune),
            },
            FieldInfoData {
                name: "goalPlanFailureTreshold",
                name_hash: 2311718028,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverComponentData, goal_plan_failure_treshold),
            },
            FieldInfoData {
                name: "goalHeightFailureTreshold",
                name_hash: 4032880576,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverComponentData, goal_height_failure_treshold),
            },
            FieldInfoData {
                name: "radiusData",
                name_hash: 2329287437,
                flags: MemberInfoFlags::new(0),
                field_type: "RadiusData",
                rust_offset: offset_of!(MoverComponentData, radius_data),
            },
            FieldInfoData {
                name: "EnablePuppetMode",
                name_hash: 771422003,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverComponentData, enable_puppet_mode),
            },
            FieldInfoData {
                name: "MoveSpeedModifier",
                name_hash: 2007712260,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverComponentData, move_speed_modifier),
            },
            FieldInfoData {
                name: "DesiredMovementAngleGameState",
                name_hash: 771020982,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_movement_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredRelativeMovementAngleGameState",
                name_hash: 649121666,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_relative_movement_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredMovementSpeedGameState",
                name_hash: 1997979984,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_movement_speed_game_state),
            },
            FieldInfoData {
                name: "DesiredFacingAngleGameState",
                name_hash: 2094415761,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_facing_angle_game_state),
            },
            FieldInfoData {
                name: "DesiredRelativeFacingAngleGameState",
                name_hash: 802929509,
                flags: MemberInfoFlags::new(0),
                field_type: "AntRef",
                rust_offset: offset_of!(MoverComponentData, desired_relative_facing_angle_game_state),
            },
            FieldInfoData {
                name: "DistanceToGoalGameState",
                name_hash: 2064512801,
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
    name_hash: 3741005645,
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
    name_hash: 1398586277,
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
    name_hash: 489706513,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("EntityMoverType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn radius_data(&self) -> &Option<LockedTypeObject /* RadiusData */> {
        self._glacier_base.radius_data()
    }
    fn radius_data_mut(&mut self) -> &mut Option<LockedTypeObject /* RadiusData */> {
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
    fn caution_tune(&self) -> &Option<LockedTypeObject /* CautionTune */> {
        self._glacier_base.caution_tune()
    }
    fn caution_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* CautionTune */> {
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
    fn auto_ob_tune(&self) -> &Option<LockedTypeObject /* AutoObstacleTune */> {
        self._glacier_base.auto_ob_tune()
    }
    fn auto_ob_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* AutoObstacleTune */> {
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
    fn path_options(&self) -> &Option<LockedTypeObject /* PathCreationOptions */> {
        self._glacier_base.path_options()
    }
    fn path_options_mut(&mut self) -> &mut Option<LockedTypeObject /* PathCreationOptions */> {
        self._glacier_base.path_options_mut()
    }
    fn jumper_tune(&self) -> &Option<LockedTypeObject /* JumperTune */> {
        self._glacier_base.jumper_tune()
    }
    fn jumper_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* JumperTune */> {
        self._glacier_base.jumper_tune_mut()
    }
    fn exit_puppet_in_obstacles(&self) -> &bool {
        self._glacier_base.exit_puppet_in_obstacles()
    }
    fn exit_puppet_in_obstacles_mut(&mut self) -> &mut bool {
        self._glacier_base.exit_puppet_in_obstacles_mut()
    }
    fn prober_tune(&self) -> &Option<LockedTypeObject /* ProberTune */> {
        self._glacier_base.prober_tune()
    }
    fn prober_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* ProberTune */> {
        self._glacier_base.prober_tune_mut()
    }
    fn allow_detour(&self) -> &bool {
        self._glacier_base.allow_detour()
    }
    fn allow_detour_mut(&mut self) -> &mut bool {
        self._glacier_base.allow_detour_mut()
    }
    fn goal_tune(&self) -> &Option<LockedTypeObject /* GoalTune */> {
        self._glacier_base.goal_tune()
    }
    fn goal_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* GoalTune */> {
        self._glacier_base.goal_tune_mut()
    }
    fn idle_tune(&self) -> &Option<LockedTypeObject /* IdleTune */> {
        self._glacier_base.idle_tune()
    }
    fn idle_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* IdleTune */> {
        self._glacier_base.idle_tune_mut()
    }
    fn turn_in_place(&self) -> &Option<LockedTypeObject /* TurnInPlaceTune */> {
        self._glacier_base.turn_in_place()
    }
    fn turn_in_place_mut(&mut self) -> &mut Option<LockedTypeObject /* TurnInPlaceTune */> {
        self._glacier_base.turn_in_place_mut()
    }
    fn repulsion_acceleration_tune(&self) -> &Option<LockedTypeObject /* RepulsionAccelerationTune */> {
        self._glacier_base.repulsion_acceleration_tune()
    }
    fn repulsion_acceleration_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* RepulsionAccelerationTune */> {
        self._glacier_base.repulsion_acceleration_tune_mut()
    }
    fn surface_orient_tune(&self) -> &Option<LockedTypeObject /* SurfaceOrientTune */> {
        self._glacier_base.surface_orient_tune()
    }
    fn surface_orient_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* SurfaceOrientTune */> {
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
    fn follower_tune(&self) -> &Option<LockedTypeObject /* FollowerTune */> {
        self._glacier_base.follower_tune()
    }
    fn follower_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* FollowerTune */> {
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
    name_hash: 1834116984,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVERTUNE_TYPE_INFO),
        super_class_offset: offset_of!(MoverTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverTuneOverride as Default>::default())),
            create_boxed: || Box::new(<MoverTuneOverride as Default>::default()),
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
    name_hash: 274661708,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1724786941,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(FOLLOWERTUNE_TYPE_INFO),
        super_class_offset: offset_of!(FollowerTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowerTuneOverride as Default>::default())),
            create_boxed: || Box::new(<FollowerTuneOverride as Default>::default()),
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
    name_hash: 657528265,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowerTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3485706437,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SURFACEORIENTTUNE_TYPE_INFO),
        super_class_offset: offset_of!(SurfaceOrientTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SurfaceOrientTuneOverride as Default>::default())),
            create_boxed: || Box::new(<SurfaceOrientTuneOverride as Default>::default()),
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
    name_hash: 2188665073,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("SurfaceOrientTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1811668949,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(AUTOOBSTACLETUNE_TYPE_INFO),
        super_class_offset: offset_of!(AutoObstacleTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoObstacleTuneOverride as Default>::default())),
            create_boxed: || Box::new(<AutoObstacleTuneOverride as Default>::default()),
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
    name_hash: 4216102881,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("AutoObstacleTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 812374751,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IDLETUNE_TYPE_INFO),
        super_class_offset: offset_of!(IdleTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IdleTuneOverride as Default>::default())),
            create_boxed: || Box::new(<IdleTuneOverride as Default>::default()),
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
    name_hash: 1287155691,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("IdleTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1156171454,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GOALTUNE_TYPE_INFO),
        super_class_offset: offset_of!(GoalTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GoalTuneOverride as Default>::default())),
            create_boxed: || Box::new(<GoalTuneOverride as Default>::default()),
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
    name_hash: 3700225802,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GoalTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2425817955,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROBERTUNE_TYPE_INFO),
        super_class_offset: offset_of!(ProberTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProberTuneOverride as Default>::default())),
            create_boxed: || Box::new(<ProberTuneOverride as Default>::default()),
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
    name_hash: 1410613079,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ProberTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 290711534,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(JUMPERTUNE_TYPE_INFO),
        super_class_offset: offset_of!(JumperTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JumperTuneOverride as Default>::default())),
            create_boxed: || Box::new(<JumperTuneOverride as Default>::default()),
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
    name_hash: 2012741850,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("JumperTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2799845616,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CAUTIONTUNE_TYPE_INFO),
        super_class_offset: offset_of!(CautionTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CautionTuneOverride as Default>::default())),
            create_boxed: || Box::new(<CautionTuneOverride as Default>::default()),
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
    name_hash: 1363975876,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CautionTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 656770044,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(REPULSIONACCELERATIONTUNE_TYPE_INFO),
        super_class_offset: offset_of!(RepulsionAccelerationTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RepulsionAccelerationTuneOverride as Default>::default())),
            create_boxed: || Box::new(<RepulsionAccelerationTuneOverride as Default>::default()),
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
    name_hash: 2574592968,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RepulsionAccelerationTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2280142714,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TURNINPLACETUNE_TYPE_INFO),
        super_class_offset: offset_of!(TurnInPlaceTuneOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurnInPlaceTuneOverride as Default>::default())),
            create_boxed: || Box::new(<TurnInPlaceTuneOverride as Default>::default()),
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
    name_hash: 4055697998,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("TurnInPlaceTuneOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3735253081,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RADIUSDATA_TYPE_INFO),
        super_class_offset: offset_of!(RadiusDataOverride, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiusDataOverride as Default>::default())),
            create_boxed: || Box::new(<RadiusDataOverride as Default>::default()),
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
    name_hash: 4022222445,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RadiusDataOverride"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MoverTune {
    pub _glacier_base: super::core::Asset,
    pub speed: f32,
    pub max_speed_fraction: f32,
    pub radius_data: Option<LockedTypeObject /* RadiusData */>,
    pub bulk: f32,
    pub cruise_acc: f32,
    pub start_stop_acc: f32,
    pub repulsor_type: i32,
    pub flock_acc: f32,
    pub max_flock_acc_dist: f32,
    pub path_acc: f32,
    pub caution_tune: Option<LockedTypeObject /* CautionTune */>,
    pub backpedal_fraction: f32,
    pub plan_layer: u32,
    pub path_sharing_penalty: f32,
    pub obstacle_mode: BlockageMode,
    pub obstacle_blockage_flags: u32,
    pub auto_ob_tune: Option<LockedTypeObject /* AutoObstacleTune */>,
    pub repulsor_blockage_flags: u32,
    pub repulsor_identity_flags: u32,
    pub link_usage_flags: u32,
    pub path_options: Option<LockedTypeObject /* PathCreationOptions */>,
    pub jumper_tune: Option<LockedTypeObject /* JumperTune */>,
    pub exit_puppet_in_obstacles: bool,
    pub prober_tune: Option<LockedTypeObject /* ProberTune */>,
    pub allow_detour: bool,
    pub goal_tune: Option<LockedTypeObject /* GoalTune */>,
    pub idle_tune: Option<LockedTypeObject /* IdleTune */>,
    pub turn_in_place: Option<LockedTypeObject /* TurnInPlaceTune */>,
    pub repulsion_acceleration_tune: Option<LockedTypeObject /* RepulsionAccelerationTune */>,
    pub surface_orient_tune: Option<LockedTypeObject /* SurfaceOrientTune */>,
    pub sidestep_fraction: f32,
    pub custom_geo_match_flags: u32,
    pub client_motion: bool,
    pub follower_tune: Option<LockedTypeObject /* FollowerTune */>,
}

pub trait MoverTuneTrait: super::core::AssetTrait {
    fn speed(&self) -> &f32;
    fn speed_mut(&mut self) -> &mut f32;
    fn max_speed_fraction(&self) -> &f32;
    fn max_speed_fraction_mut(&mut self) -> &mut f32;
    fn radius_data(&self) -> &Option<LockedTypeObject /* RadiusData */>;
    fn radius_data_mut(&mut self) -> &mut Option<LockedTypeObject /* RadiusData */>;
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
    fn caution_tune(&self) -> &Option<LockedTypeObject /* CautionTune */>;
    fn caution_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* CautionTune */>;
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
    fn auto_ob_tune(&self) -> &Option<LockedTypeObject /* AutoObstacleTune */>;
    fn auto_ob_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* AutoObstacleTune */>;
    fn repulsor_blockage_flags(&self) -> &u32;
    fn repulsor_blockage_flags_mut(&mut self) -> &mut u32;
    fn repulsor_identity_flags(&self) -> &u32;
    fn repulsor_identity_flags_mut(&mut self) -> &mut u32;
    fn link_usage_flags(&self) -> &u32;
    fn link_usage_flags_mut(&mut self) -> &mut u32;
    fn path_options(&self) -> &Option<LockedTypeObject /* PathCreationOptions */>;
    fn path_options_mut(&mut self) -> &mut Option<LockedTypeObject /* PathCreationOptions */>;
    fn jumper_tune(&self) -> &Option<LockedTypeObject /* JumperTune */>;
    fn jumper_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* JumperTune */>;
    fn exit_puppet_in_obstacles(&self) -> &bool;
    fn exit_puppet_in_obstacles_mut(&mut self) -> &mut bool;
    fn prober_tune(&self) -> &Option<LockedTypeObject /* ProberTune */>;
    fn prober_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* ProberTune */>;
    fn allow_detour(&self) -> &bool;
    fn allow_detour_mut(&mut self) -> &mut bool;
    fn goal_tune(&self) -> &Option<LockedTypeObject /* GoalTune */>;
    fn goal_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* GoalTune */>;
    fn idle_tune(&self) -> &Option<LockedTypeObject /* IdleTune */>;
    fn idle_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* IdleTune */>;
    fn turn_in_place(&self) -> &Option<LockedTypeObject /* TurnInPlaceTune */>;
    fn turn_in_place_mut(&mut self) -> &mut Option<LockedTypeObject /* TurnInPlaceTune */>;
    fn repulsion_acceleration_tune(&self) -> &Option<LockedTypeObject /* RepulsionAccelerationTune */>;
    fn repulsion_acceleration_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* RepulsionAccelerationTune */>;
    fn surface_orient_tune(&self) -> &Option<LockedTypeObject /* SurfaceOrientTune */>;
    fn surface_orient_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* SurfaceOrientTune */>;
    fn sidestep_fraction(&self) -> &f32;
    fn sidestep_fraction_mut(&mut self) -> &mut f32;
    fn custom_geo_match_flags(&self) -> &u32;
    fn custom_geo_match_flags_mut(&mut self) -> &mut u32;
    fn client_motion(&self) -> &bool;
    fn client_motion_mut(&mut self) -> &mut bool;
    fn follower_tune(&self) -> &Option<LockedTypeObject /* FollowerTune */>;
    fn follower_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* FollowerTune */>;
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
    fn radius_data(&self) -> &Option<LockedTypeObject /* RadiusData */> {
        &self.radius_data
    }
    fn radius_data_mut(&mut self) -> &mut Option<LockedTypeObject /* RadiusData */> {
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
    fn caution_tune(&self) -> &Option<LockedTypeObject /* CautionTune */> {
        &self.caution_tune
    }
    fn caution_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* CautionTune */> {
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
    fn auto_ob_tune(&self) -> &Option<LockedTypeObject /* AutoObstacleTune */> {
        &self.auto_ob_tune
    }
    fn auto_ob_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* AutoObstacleTune */> {
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
    fn path_options(&self) -> &Option<LockedTypeObject /* PathCreationOptions */> {
        &self.path_options
    }
    fn path_options_mut(&mut self) -> &mut Option<LockedTypeObject /* PathCreationOptions */> {
        &mut self.path_options
    }
    fn jumper_tune(&self) -> &Option<LockedTypeObject /* JumperTune */> {
        &self.jumper_tune
    }
    fn jumper_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* JumperTune */> {
        &mut self.jumper_tune
    }
    fn exit_puppet_in_obstacles(&self) -> &bool {
        &self.exit_puppet_in_obstacles
    }
    fn exit_puppet_in_obstacles_mut(&mut self) -> &mut bool {
        &mut self.exit_puppet_in_obstacles
    }
    fn prober_tune(&self) -> &Option<LockedTypeObject /* ProberTune */> {
        &self.prober_tune
    }
    fn prober_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* ProberTune */> {
        &mut self.prober_tune
    }
    fn allow_detour(&self) -> &bool {
        &self.allow_detour
    }
    fn allow_detour_mut(&mut self) -> &mut bool {
        &mut self.allow_detour
    }
    fn goal_tune(&self) -> &Option<LockedTypeObject /* GoalTune */> {
        &self.goal_tune
    }
    fn goal_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* GoalTune */> {
        &mut self.goal_tune
    }
    fn idle_tune(&self) -> &Option<LockedTypeObject /* IdleTune */> {
        &self.idle_tune
    }
    fn idle_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* IdleTune */> {
        &mut self.idle_tune
    }
    fn turn_in_place(&self) -> &Option<LockedTypeObject /* TurnInPlaceTune */> {
        &self.turn_in_place
    }
    fn turn_in_place_mut(&mut self) -> &mut Option<LockedTypeObject /* TurnInPlaceTune */> {
        &mut self.turn_in_place
    }
    fn repulsion_acceleration_tune(&self) -> &Option<LockedTypeObject /* RepulsionAccelerationTune */> {
        &self.repulsion_acceleration_tune
    }
    fn repulsion_acceleration_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* RepulsionAccelerationTune */> {
        &mut self.repulsion_acceleration_tune
    }
    fn surface_orient_tune(&self) -> &Option<LockedTypeObject /* SurfaceOrientTune */> {
        &self.surface_orient_tune
    }
    fn surface_orient_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* SurfaceOrientTune */> {
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
    fn follower_tune(&self) -> &Option<LockedTypeObject /* FollowerTune */> {
        &self.follower_tune
    }
    fn follower_tune_mut(&mut self) -> &mut Option<LockedTypeObject /* FollowerTune */> {
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
    name_hash: 1542577100,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(MoverTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MoverTune as Default>::default())),
            create_boxed: || Box::new(<MoverTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "speed",
                name_hash: 195170146,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, speed),
            },
            FieldInfoData {
                name: "maxSpeedFraction",
                name_hash: 2040357276,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, max_speed_fraction),
            },
            FieldInfoData {
                name: "radiusData",
                name_hash: 2329287437,
                flags: MemberInfoFlags::new(0),
                field_type: "RadiusData",
                rust_offset: offset_of!(MoverTune, radius_data),
            },
            FieldInfoData {
                name: "bulk",
                name_hash: 2087745877,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, bulk),
            },
            FieldInfoData {
                name: "cruiseAcc",
                name_hash: 1078760799,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, cruise_acc),
            },
            FieldInfoData {
                name: "startStopAcc",
                name_hash: 2510977404,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, start_stop_acc),
            },
            FieldInfoData {
                name: "repulsorType",
                name_hash: 920462093,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MoverTune, repulsor_type),
            },
            FieldInfoData {
                name: "flockAcc",
                name_hash: 2352868233,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, flock_acc),
            },
            FieldInfoData {
                name: "maxFlockAccDist",
                name_hash: 982235479,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, max_flock_acc_dist),
            },
            FieldInfoData {
                name: "pathAcc",
                name_hash: 2509679017,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, path_acc),
            },
            FieldInfoData {
                name: "cautionTune",
                name_hash: 921725796,
                flags: MemberInfoFlags::new(0),
                field_type: "CautionTune",
                rust_offset: offset_of!(MoverTune, caution_tune),
            },
            FieldInfoData {
                name: "backpedalFraction",
                name_hash: 886185208,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, backpedal_fraction),
            },
            FieldInfoData {
                name: "planLayer",
                name_hash: 3763557365,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, plan_layer),
            },
            FieldInfoData {
                name: "pathSharingPenalty",
                name_hash: 4045868059,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, path_sharing_penalty),
            },
            FieldInfoData {
                name: "obstacleMode",
                name_hash: 779133799,
                flags: MemberInfoFlags::new(0),
                field_type: "BlockageMode",
                rust_offset: offset_of!(MoverTune, obstacle_mode),
            },
            FieldInfoData {
                name: "obstacleBlockageFlags",
                name_hash: 919106705,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, obstacle_blockage_flags),
            },
            FieldInfoData {
                name: "autoObTune",
                name_hash: 2971355021,
                flags: MemberInfoFlags::new(0),
                field_type: "AutoObstacleTune",
                rust_offset: offset_of!(MoverTune, auto_ob_tune),
            },
            FieldInfoData {
                name: "repulsorBlockageFlags",
                name_hash: 3405987872,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, repulsor_blockage_flags),
            },
            FieldInfoData {
                name: "repulsorIdentityFlags",
                name_hash: 1340624092,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, repulsor_identity_flags),
            },
            FieldInfoData {
                name: "linkUsageFlags",
                name_hash: 3609013663,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, link_usage_flags),
            },
            FieldInfoData {
                name: "pathOptions",
                name_hash: 630706232,
                flags: MemberInfoFlags::new(0),
                field_type: "PathCreationOptions",
                rust_offset: offset_of!(MoverTune, path_options),
            },
            FieldInfoData {
                name: "jumperTune",
                name_hash: 2897061050,
                flags: MemberInfoFlags::new(0),
                field_type: "JumperTune",
                rust_offset: offset_of!(MoverTune, jumper_tune),
            },
            FieldInfoData {
                name: "exitPuppetInObstacles",
                name_hash: 2124082244,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverTune, exit_puppet_in_obstacles),
            },
            FieldInfoData {
                name: "proberTune",
                name_hash: 250530807,
                flags: MemberInfoFlags::new(0),
                field_type: "ProberTune",
                rust_offset: offset_of!(MoverTune, prober_tune),
            },
            FieldInfoData {
                name: "allowDetour",
                name_hash: 2849958817,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverTune, allow_detour),
            },
            FieldInfoData {
                name: "goalTune",
                name_hash: 1105509994,
                flags: MemberInfoFlags::new(0),
                field_type: "GoalTune",
                rust_offset: offset_of!(MoverTune, goal_tune),
            },
            FieldInfoData {
                name: "idleTune",
                name_hash: 3846787211,
                flags: MemberInfoFlags::new(0),
                field_type: "IdleTune",
                rust_offset: offset_of!(MoverTune, idle_tune),
            },
            FieldInfoData {
                name: "turnInPlace",
                name_hash: 1949712132,
                flags: MemberInfoFlags::new(0),
                field_type: "TurnInPlaceTune",
                rust_offset: offset_of!(MoverTune, turn_in_place),
            },
            FieldInfoData {
                name: "repulsionAccelerationTune",
                name_hash: 3852249256,
                flags: MemberInfoFlags::new(0),
                field_type: "RepulsionAccelerationTune",
                rust_offset: offset_of!(MoverTune, repulsion_acceleration_tune),
            },
            FieldInfoData {
                name: "surfaceOrientTune",
                name_hash: 818762129,
                flags: MemberInfoFlags::new(0),
                field_type: "SurfaceOrientTune",
                rust_offset: offset_of!(MoverTune, surface_orient_tune),
            },
            FieldInfoData {
                name: "sidestepFraction",
                name_hash: 4246454150,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MoverTune, sidestep_fraction),
            },
            FieldInfoData {
                name: "customGeoMatchFlags",
                name_hash: 1557106327,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MoverTune, custom_geo_match_flags),
            },
            FieldInfoData {
                name: "clientMotion",
                name_hash: 517136994,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MoverTune, client_motion),
            },
            FieldInfoData {
                name: "followerTune",
                name_hash: 50403177,
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
    name_hash: 1803115640,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("MoverTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1956273969,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(PathCreationOptions, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathCreationOptions as Default>::default())),
            create_boxed: || Box::new(<PathCreationOptions as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "performInitialNavProbe",
                name_hash: 1665269273,
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
    name_hash: 4165841285,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathCreationOptions"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 275952521,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(FollowerTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowerTune as Default>::default())),
            create_boxed: || Box::new(<FollowerTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "circulate_enable",
                name_hash: 1554199081,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FollowerTune, circulate_enable),
            },
            FieldInfoData {
                name: "circulate_minTime",
                name_hash: 3188684663,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, circulate_min_time),
            },
            FieldInfoData {
                name: "circulate_maxTime",
                name_hash: 2909864553,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, circulate_max_time),
            },
            FieldInfoData {
                name: "startupSlowness",
                name_hash: 1448093164,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, startup_slowness),
            },
            FieldInfoData {
                name: "startupBulk",
                name_hash: 78057808,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowerTune, startup_bulk),
            },
            FieldInfoData {
                name: "packingPadding",
                name_hash: 3684606445,
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
    name_hash: 4134807869,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowerTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 29097777,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(SurfaceOrientTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SurfaceOrientTune as Default>::default())),
            create_boxed: || Box::new(<SurfaceOrientTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "surfaceOrientThreshold",
                name_hash: 977706220,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SurfaceOrientTune, surface_orient_threshold),
            },
            FieldInfoData {
                name: "alwaysVerticalOnAutoGen",
                name_hash: 2676807908,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(SurfaceOrientTune, always_vertical_on_auto_gen),
            },
            FieldInfoData {
                name: "surfaceOrientSlerpTime",
                name_hash: 3064391382,
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
    name_hash: 2150060421,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("SurfaceOrientTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2015506977,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(AutoObstacleTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AutoObstacleTune as Default>::default())),
            create_boxed: || Box::new(<AutoObstacleTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "autoCreateObstacle",
                name_hash: 3874203471,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(AutoObstacleTune, auto_create_obstacle),
            },
            FieldInfoData {
                name: "delay",
                name_hash: 164121680,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AutoObstacleTune, delay),
            },
            FieldInfoData {
                name: "obstacleBlockageFlags",
                name_hash: 919106705,
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
    name_hash: 3046440085,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("AutoObstacleTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3221224747,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(IdleTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IdleTune as Default>::default())),
            create_boxed: || Box::new(<IdleTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "tetherDist",
                name_hash: 928052789,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IdleTune, tether_dist),
            },
            FieldInfoData {
                name: "returnDelay",
                name_hash: 2530899130,
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
    name_hash: 4195802271,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("IdleTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3640584458,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(GoalTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GoalTune as Default>::default())),
            create_boxed: || Box::new(<GoalTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "useCircularApproach",
                name_hash: 2180638241,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GoalTune, use_circular_approach),
            },
            FieldInfoData {
                name: "preferredTurningRadius",
                name_hash: 1960754693,
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
    name_hash: 563340350,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GoalTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1346422807,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(ProberTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProberTune as Default>::default())),
            create_boxed: || Box::new(<ProberTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "probeForGround",
                name_hash: 4090396721,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ProberTune, probe_for_ground),
            },
            FieldInfoData {
                name: "probeInterval",
                name_hash: 202224048,
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
    name_hash: 2871734435,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ProberTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3671264858,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(JumperTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<JumperTune as Default>::default())),
            create_boxed: || Box::new(<JumperTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "speed",
                name_hash: 195170146,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumperTune, speed),
            },
            FieldInfoData {
                name: "arcFraction",
                name_hash: 2593586175,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumperTune, arc_fraction),
            },
            FieldInfoData {
                name: "turnBeforeJumpAngle",
                name_hash: 101195714,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(JumperTune, turn_before_jump_angle),
            },
            FieldInfoData {
                name: "keepSpeedWhenSwapToDefault",
                name_hash: 1747788968,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(JumperTune, keep_speed_when_swap_to_default),
            },
            FieldInfoData {
                name: "onlyJumpToEndPoint",
                name_hash: 1197284363,
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
    name_hash: 1956774382,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("JumperTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3727209284,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(CautionTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CautionTune as Default>::default())),
            create_boxed: || Box::new(<CautionTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "speedX",
                name_hash: 2145647610,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(CautionTune, speed_x),
            },
            FieldInfoData {
                name: "tightTurnDegrees",
                name_hash: 2100308377,
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
    name_hash: 2715056112,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("CautionTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 359479112,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(RepulsionAccelerationTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RepulsionAccelerationTune as Default>::default())),
            create_boxed: || Box::new(<RepulsionAccelerationTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "initialAcc",
                name_hash: 2605525690,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RepulsionAccelerationTune, initial_acc),
            },
            FieldInfoData {
                name: "outerCushionAcc",
                name_hash: 3915308888,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RepulsionAccelerationTune, outer_cushion_acc),
            },
            FieldInfoData {
                name: "innerCushionAcc",
                name_hash: 1393137471,
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
    name_hash: 1688971772,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("RepulsionAccelerationTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1049107150,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(TurnInPlaceTune, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TurnInPlaceTune as Default>::default())),
            create_boxed: || Box::new(<TurnInPlaceTune as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "whenMovingAngle",
                name_hash: 1081446244,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, when_moving_angle),
            },
            FieldInfoData {
                name: "whenStoppedAngle",
                name_hash: 510562649,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, when_stopped_angle),
            },
            FieldInfoData {
                name: "speed",
                name_hash: 195170146,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, speed),
            },
            FieldInfoData {
                name: "accelAngle",
                name_hash: 4061790700,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, accel_angle),
            },
            FieldInfoData {
                name: "slideSpinThreshold",
                name_hash: 946897537,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TurnInPlaceTune, slide_spin_threshold),
            },
            FieldInfoData {
                name: "enableUTurn",
                name_hash: 938843116,
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
    name_hash: 2512082042,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("TurnInPlaceTune"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 228328877,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(RadiusData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RadiusData as Default>::default())),
            create_boxed: || Box::new(<RadiusData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "radius",
                name_hash: 2128941053,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiusData, radius),
            },
            FieldInfoData {
                name: "outerCushion",
                name_hash: 2737213913,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RadiusData, outer_cushion),
            },
            FieldInfoData {
                name: "innerCushion",
                name_hash: 3226313694,
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
    name_hash: 3804133913,
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
    name_hash: 2194191180,
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
    name_hash: 2163463160,
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
    name_hash: 3811819309,
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
    name_hash: 2694534041,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("OrientMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1307016050,
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FollowMoverSpec as Default>::default())),
            create_boxed: || Box::new(<FollowMoverSpec as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "formation",
                name_hash: 956344686,
                flags: MemberInfoFlags::new(0),
                field_type: "FollowFormation",
                rust_offset: offset_of!(FollowMoverSpec, formation),
            },
            FieldInfoData {
                name: "followDistance",
                name_hash: 2316861943,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FollowMoverSpec, follow_distance),
            },
            FieldInfoData {
                name: "arcSpread",
                name_hash: 1822299044,
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
    name_hash: 815788614,
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
    name_hash: 2030838399,
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
    name_hash: 2937088075,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("FollowFormation"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 3655463128,
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StopSpec as Default>::default())),
            create_boxed: || Box::new(<StopSpec as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "stopImmediately",
                name_hash: 159081561,
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
    name_hash: 3535293292,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("StopSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1222245407,
    flags: MemberInfoFlags::new(36937),
    module: "Pathfinding",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GotoPosSpec as Default>::default())),
            create_boxed: || Box::new(<GotoPosSpec as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "allowedToStopDist",
                name_hash: 2318324116,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GotoPosSpec, allowed_to_stop_dist),
            },
            FieldInfoData {
                name: "desiredStopDist",
                name_hash: 944514207,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(GotoPosSpec, desired_stop_dist),
            },
            FieldInfoData {
                name: "stopAtGoal",
                name_hash: 3161067693,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, stop_at_goal),
            },
            FieldInfoData {
                name: "pushThroughCrowdAtGoal",
                name_hash: 3871310557,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, push_through_crowd_at_goal),
            },
            FieldInfoData {
                name: "orientAtGoalEnable",
                name_hash: 362167135,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(GotoPosSpec, orient_at_goal_enable),
            },
            FieldInfoData {
                name: "orientAtGoalDir",
                name_hash: 1082089025,
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(GotoPosSpec, orient_at_goal_dir),
            },
            FieldInfoData {
                name: "tryFlank",
                name_hash: 1426047572,
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
    name_hash: 156942507,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("GotoPosSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathSpec {
}

pub trait PathSpecTrait: TypeObject {
}

impl PathSpecTrait for PathSpec {
}

pub static PATHSPEC_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathSpec",
    name_hash: 3654191757,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathSpec as Default>::default())),
            create_boxed: || Box::new(<PathSpec as Default>::default()),
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
    name_hash: 1649733177,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathSpec"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct PathfindingRuntimeResource {
}

pub trait PathfindingRuntimeResourceTrait: TypeObject {
}

impl PathfindingRuntimeResourceTrait for PathfindingRuntimeResource {
}

pub static PATHFINDINGRUNTIMERESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PathfindingRuntimeResource",
    name_hash: 4251460827,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PathfindingRuntimeResource as Default>::default())),
            create_boxed: || Box::new(<PathfindingRuntimeResource as Default>::default()),
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
    name_hash: 453282799,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("PathfindingRuntimeResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1518096326,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(OBSTACLECONTROLLERENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerObstacleControllerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerObstacleControllerEntity as Default>::default())),
            create_boxed: || Box::new(<ServerObstacleControllerEntity as Default>::default()),
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
    name_hash: 1690561906,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ServerObstacleControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 225038520,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ServerNavPowerSystemEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ServerNavPowerSystemEntity as Default>::default())),
            create_boxed: || Box::new(<ServerNavPowerSystemEntity as Default>::default()),
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
    name_hash: 267672076,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ServerNavPowerSystemEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1374582307,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ObstacleControllerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObstacleControllerEntity as Default>::default())),
            create_boxed: || Box::new(<ObstacleControllerEntity as Default>::default()),
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
    name_hash: 3401365911,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("ObstacleControllerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1854955339,
    flags: MemberInfoFlags::new(101),
    module: "Pathfinding",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PATHSPEC_TYPE_INFO),
        super_class_offset: offset_of!(NavPowerPathSpec, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<NavPowerPathSpec as Default>::default())),
            create_boxed: || Box::new(<NavPowerPathSpec as Default>::default()),
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
    name_hash: 695983743,
    flags: MemberInfoFlags::new(145),
    module: "Pathfinding",
    data: TypeInfoData::Array("NavPowerPathSpec"),
    array_type: None,
    alignment: 8,
};


