use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_movie_base_types(registry: &mut TypeRegistry) {
    registry.register_type(MOVIETEXTURE2ASSETFORMAT_TYPE_INFO);
    registry.register_type(MOVIETEXTURE2ASSETFORMAT_ARRAY_TYPE_INFO);
    registry.register_type(MOVIETEXTURE2ASSET_TYPE_INFO);
    registry.register_type(MOVIETEXTURE2ASSET_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOSTREAM_TYPE_INFO);
    registry.register_type(AUDIOSTREAM_ARRAY_TYPE_INFO);
    registry.register_type(VIDEOSTREAM_TYPE_INFO);
    registry.register_type(VIDEOSTREAM_ARRAY_TYPE_INFO);
    registry.register_type(AUDIOCODECTYPE_TYPE_INFO);
    registry.register_type(AUDIOCODECTYPE_ARRAY_TYPE_INFO);
    registry.register_type(VIDEOCODECTYPE_TYPE_INFO);
    registry.register_type(VIDEOCODECTYPE_ARRAY_TYPE_INFO);
    registry.register_type(VIDEOSTREAMTYPE_TYPE_INFO);
    registry.register_type(VIDEOSTREAMTYPE_ARRAY_TYPE_INFO);
    registry.register_type(MOVIETEXTUREASSET_TYPE_INFO);
    registry.register_type(MOVIETEXTUREASSET_ARRAY_TYPE_INFO);
    registry.register_type(MOVIETEXTUREBASEASSET_TYPE_INFO);
    registry.register_type(MOVIETEXTUREBASEASSET_ARRAY_TYPE_INFO);
    registry.register_type(MOVIEDYNAMICSTATE_TYPE_INFO);
    registry.register_type(MOVIEDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MOVIESTATICSTATE_TYPE_INFO);
    registry.register_type(MOVIESTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(MOVIEHANDLE_TYPE_INFO);
    registry.register_type(MOVIEHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(UICANCELALLSUBTITLESMESSAGE_TYPE_INFO);
    registry.register_type(UICANCELSUBTITLEMESSAGE_TYPE_INFO);
    registry.register_type(UISUBTITLEMESSAGE_TYPE_INFO);
    registry.register_type(UIPLAYVIDEOMESSAGE_TYPE_INFO);
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum MovieTexture2AssetFormat {
    #[default]
    MovieTexture2AssetFormat_Original = 1,
    MovieTexture2AssetFormat_Vp6EncoderFix = 2,
}

pub const MOVIETEXTURE2ASSETFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2AssetFormat",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(MOVIETEXTURE2ASSETFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MovieTexture2AssetFormat {
    fn type_info() -> &'static TypeInfo {
        MOVIETEXTURE2ASSETFORMAT_TYPE_INFO
    }
}


pub const MOVIETEXTURE2ASSETFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2AssetFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTexture2AssetFormat-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MovieTexture2Asset {
    pub chunk_guid: super::core::Guid,
    pub chunk_size: u32,
    pub subtitle_chunk_guid: super::core::Guid,
    pub subtitle_chunk_size: u32,
    pub has_localized_audio_tracks: bool,
    pub movie_filename: String,
    pub subtitle_filename: String,
    pub flipped: bool,
    pub use_stereo: bool,
}

pub const MOVIETEXTURE2ASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2Asset",
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVIETEXTUREBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, chunk_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, chunk_size),
            },
            FieldInfoData {
                name: "SubtitleChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_chunk_guid),
            },
            FieldInfoData {
                name: "SubtitleChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_chunk_size),
            },
            FieldInfoData {
                name: "HasLocalizedAudioTracks",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, has_localized_audio_tracks),
            },
            FieldInfoData {
                name: "MovieFilename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, movie_filename),
            },
            FieldInfoData {
                name: "SubtitleFilename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_filename),
            },
            FieldInfoData {
                name: "Flipped",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, flipped),
            },
            FieldInfoData {
                name: "UseStereo",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTexture2Asset, use_stereo),
            },
        ],
    }),
    array_type: Some(MOVIETEXTURE2ASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTexture2Asset {
    fn type_info() -> &'static TypeInfo {
        MOVIETEXTURE2ASSET_TYPE_INFO
    }
}


pub const MOVIETEXTURE2ASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2Asset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTexture2Asset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct AudioStream {
    pub name: String,
    pub codec: AudioCodecType,
    pub language: String,
    pub length: f32,
    pub sample_rate: u32,
    pub bit_rate: u32,
    pub channels: u32,
    pub audio_filename: String,
}

pub const AUDIOSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioStream",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, name),
            },
            FieldInfoData {
                name: "Codec",
                flags: MemberInfoFlags::new(0),
                field_type: AUDIOCODECTYPE_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, codec),
            },
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, language),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, length),
            },
            FieldInfoData {
                name: "SampleRate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, sample_rate),
            },
            FieldInfoData {
                name: "BitRate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, bit_rate),
            },
            FieldInfoData {
                name: "Channels",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, channels),
            },
            FieldInfoData {
                name: "AudioFilename",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(AudioStream, audio_filename),
            },
        ],
    }),
    array_type: Some(AUDIOSTREAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioStream {
    fn type_info() -> &'static TypeInfo {
        AUDIOSTREAM_TYPE_INFO
    }
}


pub const AUDIOSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("AudioStream-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct VideoStream {
    pub name: String,
    pub r#type: VideoStreamType,
    pub codec: VideoCodecType,
    pub width: i32,
    pub height: i32,
    pub length: f32,
    pub bit_rate: u32,
    pub frames: u32,
    pub fps: f32,
}

pub const VIDEOSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStream",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, name),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: VIDEOSTREAMTYPE_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, r#type),
            },
            FieldInfoData {
                name: "Codec",
                flags: MemberInfoFlags::new(0),
                field_type: VIDEOCODECTYPE_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, codec),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, height),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, length),
            },
            FieldInfoData {
                name: "BitRate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, bit_rate),
            },
            FieldInfoData {
                name: "Frames",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, frames),
            },
            FieldInfoData {
                name: "Fps",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(VideoStream, fps),
            },
        ],
    }),
    array_type: Some(VIDEOSTREAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VideoStream {
    fn type_info() -> &'static TypeInfo {
        VIDEOSTREAM_TYPE_INFO
    }
}


pub const VIDEOSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoStream-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum AudioCodecType {
    #[default]
    AudioCodecType_Vorbis = 0,
    AudioCodecType_AAC = 1,
    AudioCodecType_Other = 2,
}

pub const AUDIOCODECTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCodecType",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(AUDIOCODECTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AudioCodecType {
    fn type_info() -> &'static TypeInfo {
        AUDIOCODECTYPE_TYPE_INFO
    }
}


pub const AUDIOCODECTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCodecType-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("AudioCodecType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VideoCodecType {
    #[default]
    VideoCodecType_VP6 = 0,
    VideoCodecType_VP8 = 1,
    VideoCodecType_VP9 = 2,
    VideoCodecType_H264 = 3,
    VideoCodecType_JPEG = 4,
    VideoCodecType_Other = 5,
}

pub const VIDEOCODECTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoCodecType",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIDEOCODECTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VideoCodecType {
    fn type_info() -> &'static TypeInfo {
        VIDEOCODECTYPE_TYPE_INFO
    }
}


pub const VIDEOCODECTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoCodecType-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoCodecType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum VideoStreamType {
    #[default]
    VideoStreamType_MainVideo = 0,
    VideoStreamType_Alpha = 1,
}

pub const VIDEOSTREAMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStreamType",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIDEOSTREAMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VideoStreamType {
    fn type_info() -> &'static TypeInfo {
        VIDEOSTREAMTYPE_TYPE_INFO
    }
}


pub const VIDEOSTREAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStreamType-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoStreamType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MovieTextureAsset {
    pub chunk_guid: super::core::Guid,
    pub chunk_size: u32,
    pub subtitle_chunk_guid: super::core::Guid,
    pub subtitle_chunk_size: u32,
    pub has_localized_audio_tracks: bool,
    pub override_background_music: bool,
    pub has_vp6: bool,
    pub has_vp8: bool,
}

pub const MOVIETEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVIETEXTUREBASEASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, chunk_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, chunk_size),
            },
            FieldInfoData {
                name: "SubtitleChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: GUID_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, subtitle_chunk_guid),
            },
            FieldInfoData {
                name: "SubtitleChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, subtitle_chunk_size),
            },
            FieldInfoData {
                name: "HasLocalizedAudioTracks",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, has_localized_audio_tracks),
            },
            FieldInfoData {
                name: "OverrideBackgroundMusic",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, override_background_music),
            },
            FieldInfoData {
                name: "HasVp6",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, has_vp6),
            },
            FieldInfoData {
                name: "HasVp8",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieTextureAsset, has_vp8),
            },
        ],
    }),
    array_type: Some(MOVIETEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTextureAsset {
    fn type_info() -> &'static TypeInfo {
        MOVIETEXTUREASSET_TYPE_INFO
    }
}


pub const MOVIETEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTextureAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MovieTextureBaseAsset {
}

pub const MOVIETEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MOVIETEXTUREBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTextureBaseAsset {
    fn type_info() -> &'static TypeInfo {
        MOVIETEXTUREBASEASSET_TYPE_INFO
    }
}


pub const MOVIETEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTextureBaseAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MovieDynamicState {
    pub external_time: f32,
    pub volume: f32,
    pub play: bool,
    pub enable_state_stream_destroy_dependency_bug_workaround: bool,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub const MOVIEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieDynamicState, external_time),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(MovieDynamicState, volume),
            },
            FieldInfoData {
                name: "Play",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieDynamicState, play),
            },
            FieldInfoData {
                name: "EnableStateStreamDestroyDependencyBugWorkaround",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieDynamicState, enable_state_stream_destroy_dependency_bug_workaround),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MovieDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(MovieDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MOVIEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MovieDynamicState {
    fn type_info() -> &'static TypeInfo {
        MOVIEDYNAMICSTATE_TYPE_INFO
    }
}


pub const MOVIEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MovieStaticState {
    pub movie_texture: MovieTextureBaseAsset,
    pub movie_name: String,
    pub subtitle_name: String,
    pub audio_track_id: String,
    pub enable_subtitles: bool,
    pub r#loop: bool,
    pub preload: bool,
    pub use_sim_time: bool,
    pub pause_on_ending: bool,
    pub shader_block_handles: Vec<super::render_base::ShaderParameterBlockHandle>,
    pub shader_block_textures: Vec<super::render_base::ShaderBlockTexture>,
    pub texture_count: i32,
    pub decoder_threads: i32,
    pub stop_frame: i32,
    pub language_override: super::core::LanguageFormat,
    pub if_i_delete_this_other_stuff_breaks: super::render_base::ShaderBlockBool,
    pub field_flag_changed0: u16,
}

pub const MOVIESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieStaticState",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MovieTexture",
                flags: MemberInfoFlags::new(0),
                field_type: MOVIETEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, movie_texture),
            },
            FieldInfoData {
                name: "MovieName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, movie_name),
            },
            FieldInfoData {
                name: "SubtitleName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, subtitle_name),
            },
            FieldInfoData {
                name: "AudioTrackId",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, audio_track_id),
            },
            FieldInfoData {
                name: "EnableSubtitles",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, enable_subtitles),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, r#loop),
            },
            FieldInfoData {
                name: "Preload",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, preload),
            },
            FieldInfoData {
                name: "UseSimTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, use_sim_time),
            },
            FieldInfoData {
                name: "PauseOnEnding",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, pause_on_ending),
            },
            FieldInfoData {
                name: "ShaderBlockHandles",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERPARAMETERBLOCKHANDLE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, shader_block_handles),
            },
            FieldInfoData {
                name: "ShaderBlockTextures",
                flags: MemberInfoFlags::new(144),
                field_type: SHADERBLOCKTEXTURE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, shader_block_textures),
            },
            FieldInfoData {
                name: "TextureCount",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, texture_count),
            },
            FieldInfoData {
                name: "DecoderThreads",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, decoder_threads),
            },
            FieldInfoData {
                name: "StopFrame",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, stop_frame),
            },
            FieldInfoData {
                name: "LanguageOverride",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, language_override),
            },
            FieldInfoData {
                name: "IfIDeleteThisOtherStuffBreaks",
                flags: MemberInfoFlags::new(0),
                field_type: SHADERBLOCKBOOL_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, if_i_delete_this_other_stuff_breaks),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(MovieStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MOVIESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieStaticState {
    fn type_info() -> &'static TypeInfo {
        MOVIESTATICSTATE_TYPE_INFO
    }
}


pub const MOVIESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MovieHandle {
}

pub const MOVIEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieHandle",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(MOVIEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for MovieHandle {
    fn type_info() -> &'static TypeInfo {
        MOVIEHANDLE_TYPE_INFO
    }
}


pub const MOVIEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UICancelAllSubtitlesMessage {
}

pub const UICANCELALLSUBTITLESMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICancelAllSubtitlesMessage",
    flags: MemberInfoFlags::new(36937),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UICancelAllSubtitlesMessage {
    fn type_info() -> &'static TypeInfo {
        UICANCELALLSUBTITLESMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UICancelSubtitleMessage {
}

pub const UICANCELSUBTITLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICancelSubtitleMessage",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UICancelSubtitleMessage {
    fn type_info() -> &'static TypeInfo {
        UICANCELSUBTITLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UISubtitleMessage {
}

pub const UISUBTITLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISubtitleMessage",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UISubtitleMessage {
    fn type_info() -> &'static TypeInfo {
        UISUBTITLEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIPlayVideoMessage {
}

pub const UIPLAYVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIPlayVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIPlayVideoMessage {
    fn type_info() -> &'static TypeInfo {
        UIPLAYVIDEOMESSAGE_TYPE_INFO
    }
}

