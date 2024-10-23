use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_movie_entity_types(registry: &mut TypeRegistry) {
    registry.register_type(MOVIEENTITYDATA_TYPE_INFO);
    registry.register_type(MOVIEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMOVIEENTITY_TYPE_INFO);
    registry.register_type(CLIENTMOVIEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct MovieEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub movie: Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */>,
    pub external_time: f32,
    pub is_normal_map: bool,
    pub is_looping: bool,
    pub pre_buffer: bool,
    pub volume: f32,
    pub renderable_count: u32,
    pub thread_count: u32,
}

pub trait MovieEntityDataTrait: super::entity::EntityDataTrait {
    fn movie(&self) -> &Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */>;
    fn movie_mut(&mut self) -> &mut Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */>;
    fn external_time(&self) -> &f32;
    fn external_time_mut(&mut self) -> &mut f32;
    fn is_normal_map(&self) -> &bool;
    fn is_normal_map_mut(&mut self) -> &mut bool;
    fn is_looping(&self) -> &bool;
    fn is_looping_mut(&mut self) -> &mut bool;
    fn pre_buffer(&self) -> &bool;
    fn pre_buffer_mut(&mut self) -> &mut bool;
    fn volume(&self) -> &f32;
    fn volume_mut(&mut self) -> &mut f32;
    fn renderable_count(&self) -> &u32;
    fn renderable_count_mut(&mut self) -> &mut u32;
    fn thread_count(&self) -> &u32;
    fn thread_count_mut(&mut self) -> &mut u32;
}

impl MovieEntityDataTrait for MovieEntityData {
    fn movie(&self) -> &Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */> {
        &self.movie
    }
    fn movie_mut(&mut self) -> &mut Option<LockedTypeObject /* super::movie_base::MovieTextureBaseAsset */> {
        &mut self.movie
    }
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        &mut self.external_time
    }
    fn is_normal_map(&self) -> &bool {
        &self.is_normal_map
    }
    fn is_normal_map_mut(&mut self) -> &mut bool {
        &mut self.is_normal_map
    }
    fn is_looping(&self) -> &bool {
        &self.is_looping
    }
    fn is_looping_mut(&mut self) -> &mut bool {
        &mut self.is_looping
    }
    fn pre_buffer(&self) -> &bool {
        &self.pre_buffer
    }
    fn pre_buffer_mut(&mut self) -> &mut bool {
        &mut self.pre_buffer
    }
    fn volume(&self) -> &f32 {
        &self.volume
    }
    fn volume_mut(&mut self) -> &mut f32 {
        &mut self.volume
    }
    fn renderable_count(&self) -> &u32 {
        &self.renderable_count
    }
    fn renderable_count_mut(&mut self) -> &mut u32 {
        &mut self.renderable_count
    }
    fn thread_count(&self) -> &u32 {
        &self.thread_count
    }
    fn thread_count_mut(&mut self) -> &mut u32 {
        &mut self.thread_count
    }
}

impl super::entity::EntityDataTrait for MovieEntityData {
}

impl super::entity::GameObjectDataTrait for MovieEntityData {
}

impl super::core::DataBusPeerTrait for MovieEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MovieEntityData {
}

impl super::core::DataContainerTrait for MovieEntityData {
}

pub static MOVIEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieEntityData",
    name_hash: 3650725078,
    flags: MemberInfoFlags::new(101),
    module: "MovieEntity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(MovieEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieEntityData as Default>::default())),
            create_boxed: || Box::new(<MovieEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Movie",
                name_hash: 210030653,
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(MovieEntityData, movie),
            },
            FieldInfoData {
                name: "ExternalTime",
                name_hash: 2162678253,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieEntityData, external_time),
            },
            FieldInfoData {
                name: "IsNormalMap",
                name_hash: 797341680,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieEntityData, is_normal_map),
            },
            FieldInfoData {
                name: "IsLooping",
                name_hash: 1137411139,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieEntityData, is_looping),
            },
            FieldInfoData {
                name: "PreBuffer",
                name_hash: 1333695138,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieEntityData, pre_buffer),
            },
            FieldInfoData {
                name: "Volume",
                name_hash: 3158011725,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieEntityData, volume),
            },
            FieldInfoData {
                name: "RenderableCount",
                name_hash: 1982187302,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieEntityData, renderable_count),
            },
            FieldInfoData {
                name: "ThreadCount",
                name_hash: 886477064,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieEntityData, thread_count),
            },
        ],
    }),
    array_type: Some(MOVIEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIEENTITYDATA_TYPE_INFO
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


pub static MOVIEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieEntityData-Array",
    name_hash: 3948923490,
    flags: MemberInfoFlags::new(145),
    module: "MovieEntity",
    data: TypeInfoData::Array("MovieEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct ClientMovieEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientMovieEntityTrait: super::entity::EntityTrait {
}

impl ClientMovieEntityTrait for ClientMovieEntity {
}

impl super::entity::EntityTrait for ClientMovieEntity {
}

impl super::entity::EntityBusPeerTrait for ClientMovieEntity {
}

pub static CLIENTMOVIEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieEntity",
    name_hash: 3371971903,
    flags: MemberInfoFlags::new(101),
    module: "MovieEntity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(ClientMovieEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMovieEntity as Default>::default())),
            create_boxed: || Box::new(<ClientMovieEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMOVIEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMovieEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMOVIEENTITY_TYPE_INFO
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


pub static CLIENTMOVIEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieEntity-Array",
    name_hash: 2117044875,
    flags: MemberInfoFlags::new(145),
    module: "MovieEntity",
    data: TypeInfoData::Array("ClientMovieEntity"),
    array_type: None,
    alignment: 8,
};


