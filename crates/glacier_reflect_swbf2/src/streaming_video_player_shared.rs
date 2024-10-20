use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_streaming_video_player_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(STREAMINGVIDEOPLAYERENTITYDATA_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOPLAYERENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamingVideoPlayerEntityData {
    pub video_u_r_l: String,
    pub texture_width: i32,
    pub texture_height: i32,
}

pub const STREAMINGVIDEOPLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "StreamingVideoPlayerShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "VideoURL",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StreamingVideoPlayerEntityData, video_u_r_l),
            },
            FieldInfoData {
                name: "TextureWidth",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StreamingVideoPlayerEntityData, texture_width),
            },
            FieldInfoData {
                name: "TextureHeight",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(StreamingVideoPlayerEntityData, texture_height),
            },
        ],
    }),
    array_type: Some(STREAMINGVIDEOPLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StreamingVideoPlayerEntityData {
    fn type_info() -> &'static TypeInfo {
        STREAMINGVIDEOPLAYERENTITYDATA_TYPE_INFO
    }
}


pub const STREAMINGVIDEOPLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayerShared",
    data: TypeInfoData::Array("StreamingVideoPlayerEntityData-Array"),
    array_type: None,
    alignment: 8,
};


