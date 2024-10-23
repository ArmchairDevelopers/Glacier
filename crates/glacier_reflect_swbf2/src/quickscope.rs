use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_quickscope_types(registry: &mut TypeRegistry) {
    registry.register_type(QUICKSCOPECONTROLENTITY_TYPE_INFO);
    registry.register_type(QUICKSCOPECONTROLENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct QuickscopeControlEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait QuickscopeControlEntityTrait: super::entity::EntityTrait {
}

impl QuickscopeControlEntityTrait for QuickscopeControlEntity {
}

impl super::entity::EntityTrait for QuickscopeControlEntity {
}

impl super::entity::EntityBusPeerTrait for QuickscopeControlEntity {
}

pub static QUICKSCOPECONTROLENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntity",
    name_hash: 1600559798,
    flags: MemberInfoFlags::new(101),
    module: "Quickscope",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(QuickscopeControlEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuickscopeControlEntity as Default>::default())),
            create_boxed: || Box::new(<QuickscopeControlEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(QUICKSCOPECONTROLENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for QuickscopeControlEntity {
    fn type_info(&self) -> &'static TypeInfo {
        QUICKSCOPECONTROLENTITY_TYPE_INFO
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


pub static QUICKSCOPECONTROLENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuickscopeControlEntity-Array",
    name_hash: 3688460034,
    flags: MemberInfoFlags::new(145),
    module: "Quickscope",
    data: TypeInfoData::Array("QuickscopeControlEntity"),
    array_type: None,
    alignment: 8,
};


