use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_movie_entity_types(registry: &mut TypeRegistry) {
    registry.register_type(MOVIEENTITYDATA_TYPE_INFO);
    registry.register_type(MOVIEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMOVIEENTITY_TYPE_INFO);
    registry.register_type(CLIENTMOVIEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Debug)]
pub struct MovieEntityData {
    pub movie: super::movie_base::MovieTextureBaseAsset,
    pub external_time: f32,
    pub is_normal_map: bool,
    pub is_looping: bool,
    pub pre_buffer: bool,
    pub volume: f32,
    pub renderable_count: u32,
    pub thread_count: u32,
}

pub const MOVIEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieEntityData",
    flags: MemberInfoFlags::new(101),
    module: "MovieEntity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: MOVIETEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, movie),
            },
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, external_time),
            },
            FieldInfoData {
                name: "IsNormalMap",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, is_normal_map),
            },
            FieldInfoData {
                name: "IsLooping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, is_looping),
            },
            FieldInfoData {
                name: "PreBuffer",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, pre_buffer),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, volume),
            },
            FieldInfoData {
                name: "RenderableCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, renderable_count),
            },
            FieldInfoData {
                name: "ThreadCount",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieEntityData, thread_count),
            },
        ],
    }),
    array_type: Some(MOVIEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieEntityData {
    fn type_info() -> &'static TypeInfo {
        MOVIEENTITYDATA_TYPE_INFO
    }
}


pub const MOVIEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieEntity",
    data: TypeInfoData::Array("MovieEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMovieEntity {
}

pub const CLIENTMOVIEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieEntity",
    flags: MemberInfoFlags::new(101),
    module: "MovieEntity",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMOVIEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMovieEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTMOVIEENTITY_TYPE_INFO
    }
}


pub const CLIENTMOVIEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieEntity",
    data: TypeInfoData::Array("ClientMovieEntity-Array"),
    array_type: None,
    alignment: 8,
};


