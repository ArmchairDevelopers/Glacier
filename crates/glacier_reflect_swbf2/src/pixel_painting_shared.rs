use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_pixel_painting_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(PIXELPAINTINGBLUEPRINT_TYPE_INFO);
    registry.register_type(PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct PixelPaintingBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
}

pub trait PixelPaintingBlueprintTrait: super::entity::ObjectBlueprintTrait {
}

impl PixelPaintingBlueprintTrait for PixelPaintingBlueprint {
}

impl super::entity::ObjectBlueprintTrait for PixelPaintingBlueprint {
    fn object(&self) -> &Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object()
    }
    fn object_mut(&mut self) -> &mut Option<LockedTypeObject /* super::entity::EntityData */> {
        self._glacier_base.object_mut()
    }
}

impl super::entity::BlueprintTrait for PixelPaintingBlueprint {
    fn objects(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects()
    }
    fn objects_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.objects_mut()
    }
    fn schematics(&self) -> &Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics()
    }
    fn schematics_mut(&mut self) -> &mut Option<LockedTypeObject /* super::schematics::SchematicsBaseAsset */> {
        self._glacier_base.schematics_mut()
    }
}

impl super::entity::EntityBusDataTrait for PixelPaintingBlueprint {
    fn event_connections(&self) -> &Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections()
    }
    fn event_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::entity::EventConnection */> {
        self._glacier_base.event_connections_mut()
    }
}

impl super::core::DataBusDataTrait for PixelPaintingBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.flags_mut()
    }
    fn property_connections(&self) -> &Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections()
    }
    fn property_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::PropertyConnection */> {
        self._glacier_base.property_connections_mut()
    }
    fn link_connections(&self) -> &Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections()
    }
    fn link_connections_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::core::LinkConnection */> {
        self._glacier_base.link_connections_mut()
    }
    fn interface(&self) -> &Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface()
    }
    fn interface_mut(&mut self) -> &mut Option<LockedTypeObject /* super::core::DynamicDataContainer */> {
        self._glacier_base.interface_mut()
    }
}

impl super::core::AssetTrait for PixelPaintingBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for PixelPaintingBlueprint {
}

pub static PIXELPAINTINGBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelPaintingBlueprint",
    name_hash: 1033948448,
    flags: MemberInfoFlags::new(101),
    module: "PixelPaintingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        super_class_offset: offset_of!(PixelPaintingBlueprint, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PixelPaintingBlueprint as Default>::default())),
            create_boxed: || Box::new(<PixelPaintingBlueprint as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PixelPaintingBlueprint {
    fn type_info(&self) -> &'static TypeInfo {
        PIXELPAINTINGBLUEPRINT_TYPE_INFO
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


pub static PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelPaintingBlueprint-Array",
    name_hash: 3481571988,
    flags: MemberInfoFlags::new(145),
    module: "PixelPaintingShared",
    data: TypeInfoData::Array("PixelPaintingBlueprint"),
    array_type: None,
    alignment: 8,
};


