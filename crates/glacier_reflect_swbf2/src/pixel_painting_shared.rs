use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_pixel_painting_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(PIXELPAINTINGBLUEPRINT_TYPE_INFO);
    registry.register_type(PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct PixelPaintingBlueprint {
    pub _glacier_base: super::entity::ObjectBlueprint,
}

pub trait PixelPaintingBlueprintTrait: super::entity::ObjectBlueprintTrait {
}

impl PixelPaintingBlueprintTrait for PixelPaintingBlueprint {
}

impl super::entity::ObjectBlueprintTrait for PixelPaintingBlueprint {
    fn object(&self) -> &Option<Arc<Mutex<dyn super::entity::EntityDataTrait>>> {
        self._glacier_base.object()
    }
}

impl super::entity::BlueprintTrait for PixelPaintingBlueprint {
    fn objects(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.objects()
    }
    fn schematics(&self) -> &Option<Arc<Mutex<dyn super::schematics::SchematicsBaseAssetTrait>>> {
        self._glacier_base.schematics()
    }
}

impl super::entity::EntityBusDataTrait for PixelPaintingBlueprint {
    fn event_connections(&self) -> &Vec<super::entity::EventConnection> {
        self._glacier_base.event_connections()
    }
}

impl super::core::DataBusDataTrait for PixelPaintingBlueprint {
    fn flags(&self) -> &u16 {
        self._glacier_base.flags()
    }
    fn property_connections(&self) -> &Vec<super::core::PropertyConnection> {
        self._glacier_base.property_connections()
    }
    fn link_connections(&self) -> &Vec<super::core::LinkConnection> {
        self._glacier_base.link_connections()
    }
    fn interface(&self) -> &Option<Arc<Mutex<dyn super::core::DynamicDataContainerTrait>>> {
        self._glacier_base.interface()
    }
}

impl super::core::AssetTrait for PixelPaintingBlueprint {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for PixelPaintingBlueprint {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static PIXELPAINTINGBLUEPRINT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelPaintingBlueprint",
    flags: MemberInfoFlags::new(101),
    module: "PixelPaintingShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::OBJECTBLUEPRINT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PixelPaintingBlueprint as Default>::default())),
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
}


pub static PIXELPAINTINGBLUEPRINT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PixelPaintingBlueprint-Array",
    flags: MemberInfoFlags::new(145),
    module: "PixelPaintingShared",
    data: TypeInfoData::Array("PixelPaintingBlueprint"),
    array_type: None,
    alignment: 8,
};


