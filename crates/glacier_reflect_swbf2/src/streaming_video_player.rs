use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_streaming_video_player_types(registry: &mut TypeRegistry) {
    registry.register_type(STREAMINGVIDEODYNAMICSTATE_TYPE_INFO);
    registry.register_type(STREAMINGVIDEODYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOSTATICSTATE_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOHANDLE_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOPLAYERENTITY_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOPLAYERENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamingVideoDynamicState {
}

pub const STREAMINGVIDEODYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(STREAMINGVIDEODYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StreamingVideoDynamicState {
    fn type_info() -> &'static TypeInfo {
        STREAMINGVIDEODYNAMICSTATE_TYPE_INFO
    }
}


pub const STREAMINGVIDEODYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamingVideoStaticState {
    pub url: String,
    pub texture: super::render_base::TextureResourceHandle,
    pub field_flag_changed0: u8,
}

pub const STREAMINGVIDEOSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoStaticState",
    flags: MemberInfoFlags::new(73),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Url",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(StreamingVideoStaticState, url),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(StreamingVideoStaticState, texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(StreamingVideoStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(STREAMINGVIDEOSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StreamingVideoStaticState {
    fn type_info() -> &'static TypeInfo {
        STREAMINGVIDEOSTATICSTATE_TYPE_INFO
    }
}


pub const STREAMINGVIDEOSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamingVideoHandle {
}

pub const STREAMINGVIDEOHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoHandle",
    flags: MemberInfoFlags::new(73),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(STREAMINGVIDEOHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StreamingVideoHandle {
    fn type_info() -> &'static TypeInfo {
        STREAMINGVIDEOHANDLE_TYPE_INFO
    }
}


pub const STREAMINGVIDEOHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StreamingVideoPlayerEntity {
}

pub const STREAMINGVIDEOPLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntity",
    flags: MemberInfoFlags::new(101),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STREAMINGVIDEOPLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StreamingVideoPlayerEntity {
    fn type_info() -> &'static TypeInfo {
        STREAMINGVIDEOPLAYERENTITY_TYPE_INFO
    }
}


pub const STREAMINGVIDEOPLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoPlayerEntity-Array"),
    array_type: None,
    alignment: 8,
};


