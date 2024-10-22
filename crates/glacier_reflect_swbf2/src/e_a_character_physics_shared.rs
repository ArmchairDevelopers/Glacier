use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_e_a_character_physics_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(EACHARACTERPHYSICSSTATETESTENTITYDATA_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSSTATETESTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITIONS_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITIONS_ARRAY_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITION_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTPOSITION_ARRAY_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTDATA_TYPE_INFO);
    registry.register_type(EACHARACTERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct EACharacterPhysicsStateTestEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
}

pub trait EACharacterPhysicsStateTestEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl EACharacterPhysicsStateTestEntityDataTrait for EACharacterPhysicsStateTestEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::EntityDataTrait for EACharacterPhysicsStateTestEntityData {
}

impl super::entity::GameObjectDataTrait for EACharacterPhysicsStateTestEntityData {
}

impl super::core::DataBusPeerTrait for EACharacterPhysicsStateTestEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EACharacterPhysicsStateTestEntityData {
}

impl super::core::DataContainerTrait for EACharacterPhysicsStateTestEntityData {
}

pub static EACHARACTERPHYSICSSTATETESTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsStateTestEntityData",
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EACharacterPhysicsStateTestEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(EACharacterPhysicsStateTestEntityData, realm),
            },
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSSTATETESTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EACharacterPhysicsStateTestEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        EACHARACTERPHYSICSSTATETESTENTITYDATA_TYPE_INFO
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


pub static EACHARACTERPHYSICSSTATETESTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsStateTestEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsStateTestEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EACharacterPhysicsComponentPositions {
}

pub trait EACharacterPhysicsComponentPositionsTrait: TypeObject {
}

impl EACharacterPhysicsComponentPositionsTrait for EACharacterPhysicsComponentPositions {
}

pub static EACHARACTERPHYSICSCOMPONENTPOSITIONS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPositions",
    flags: MemberInfoFlags::new(73),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EACharacterPhysicsComponentPositions as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSCOMPONENTPOSITIONS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EACharacterPhysicsComponentPositions {
    fn type_info(&self) -> &'static TypeInfo {
        EACHARACTERPHYSICSCOMPONENTPOSITIONS_TYPE_INFO
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


pub static EACHARACTERPHYSICSCOMPONENTPOSITIONS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPositions-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsComponentPositions"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EACharacterPhysicsComponentPosition {
    pub orientation: super::core::Quat,
    pub translation: super::core::Vec3,
}

pub trait EACharacterPhysicsComponentPositionTrait: TypeObject {
    fn orientation(&self) -> &super::core::Quat;
    fn orientation_mut(&mut self) -> &mut super::core::Quat;
    fn translation(&self) -> &super::core::Vec3;
    fn translation_mut(&mut self) -> &mut super::core::Vec3;
}

impl EACharacterPhysicsComponentPositionTrait for EACharacterPhysicsComponentPosition {
    fn orientation(&self) -> &super::core::Quat {
        &self.orientation
    }
    fn orientation_mut(&mut self) -> &mut super::core::Quat {
        &mut self.orientation
    }
    fn translation(&self) -> &super::core::Vec3 {
        &self.translation
    }
    fn translation_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.translation
    }
}

pub static EACHARACTERPHYSICSCOMPONENTPOSITION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPosition",
    flags: MemberInfoFlags::new(36937),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EACharacterPhysicsComponentPosition as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Orientation",
                flags: MemberInfoFlags::new(0),
                field_type: "Quat",
                rust_offset: offset_of!(EACharacterPhysicsComponentPosition, orientation),
            },
            FieldInfoData {
                name: "Translation",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(EACharacterPhysicsComponentPosition, translation),
            },
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSCOMPONENTPOSITION_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EACharacterPhysicsComponentPosition {
    fn type_info(&self) -> &'static TypeInfo {
        EACHARACTERPHYSICSCOMPONENTPOSITION_TYPE_INFO
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


pub static EACHARACTERPHYSICSCOMPONENTPOSITION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentPosition-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsComponentPosition"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EACharacterPhysicsComponentData {
    pub _glacier_base: super::entity::GameComponentData,
    pub realm: super::core::Realm,
    pub client_authoritative: bool,
    pub material_pair: super::entity::MaterialDecl,
    pub physics_blueprint: Option<Arc<Mutex<dyn super::entity::ObjectBlueprintTrait>>>,
    pub bone_materials: Vec<super::entity::MaterialDecl>,
}

pub trait EACharacterPhysicsComponentDataTrait: super::entity::GameComponentDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn client_authoritative(&self) -> &bool;
    fn client_authoritative_mut(&mut self) -> &mut bool;
    fn material_pair(&self) -> &super::entity::MaterialDecl;
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl;
    fn physics_blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectBlueprintTrait>>>;
    fn physics_blueprint_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::ObjectBlueprintTrait>>>;
    fn bone_materials(&self) -> &Vec<super::entity::MaterialDecl>;
    fn bone_materials_mut(&mut self) -> &mut Vec<super::entity::MaterialDecl>;
}

impl EACharacterPhysicsComponentDataTrait for EACharacterPhysicsComponentData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn client_authoritative(&self) -> &bool {
        &self.client_authoritative
    }
    fn client_authoritative_mut(&mut self) -> &mut bool {
        &mut self.client_authoritative
    }
    fn material_pair(&self) -> &super::entity::MaterialDecl {
        &self.material_pair
    }
    fn material_pair_mut(&mut self) -> &mut super::entity::MaterialDecl {
        &mut self.material_pair
    }
    fn physics_blueprint(&self) -> &Option<Arc<Mutex<dyn super::entity::ObjectBlueprintTrait>>> {
        &self.physics_blueprint
    }
    fn physics_blueprint_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::entity::ObjectBlueprintTrait>>> {
        &mut self.physics_blueprint
    }
    fn bone_materials(&self) -> &Vec<super::entity::MaterialDecl> {
        &self.bone_materials
    }
    fn bone_materials_mut(&mut self) -> &mut Vec<super::entity::MaterialDecl> {
        &mut self.bone_materials
    }
}

impl super::entity::GameComponentDataTrait for EACharacterPhysicsComponentData {
}

impl super::entity::ComponentDataTrait for EACharacterPhysicsComponentData {
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

impl super::entity::GameObjectDataTrait for EACharacterPhysicsComponentData {
}

impl super::core::DataBusPeerTrait for EACharacterPhysicsComponentData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EACharacterPhysicsComponentData {
}

impl super::core::DataContainerTrait for EACharacterPhysicsComponentData {
}

pub static EACHARACTERPHYSICSCOMPONENTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentData",
    flags: MemberInfoFlags::new(101),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMECOMPONENTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EACharacterPhysicsComponentData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(EACharacterPhysicsComponentData, realm),
            },
            FieldInfoData {
                name: "ClientAuthoritative",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EACharacterPhysicsComponentData, client_authoritative),
            },
            FieldInfoData {
                name: "MaterialPair",
                flags: MemberInfoFlags::new(0),
                field_type: "MaterialDecl",
                rust_offset: offset_of!(EACharacterPhysicsComponentData, material_pair),
            },
            FieldInfoData {
                name: "PhysicsBlueprint",
                flags: MemberInfoFlags::new(0),
                field_type: "ObjectBlueprint",
                rust_offset: offset_of!(EACharacterPhysicsComponentData, physics_blueprint),
            },
            FieldInfoData {
                name: "BoneMaterials",
                flags: MemberInfoFlags::new(144),
                field_type: "MaterialDecl-Array",
                rust_offset: offset_of!(EACharacterPhysicsComponentData, bone_materials),
            },
        ],
    }),
    array_type: Some(EACHARACTERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for EACharacterPhysicsComponentData {
    fn type_info(&self) -> &'static TypeInfo {
        EACHARACTERPHYSICSCOMPONENTDATA_TYPE_INFO
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


pub static EACHARACTERPHYSICSCOMPONENTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EACharacterPhysicsComponentData-Array",
    flags: MemberInfoFlags::new(145),
    module: "EACharacterPhysicsShared",
    data: TypeInfoData::Array("EACharacterPhysicsComponentData"),
    array_type: None,
    alignment: 8,
};


