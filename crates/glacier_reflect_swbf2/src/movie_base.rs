use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum MovieTexture2AssetFormat {
    #[default]
    MovieTexture2AssetFormat_Original = 1,
    MovieTexture2AssetFormat_Vp6EncoderFix = 2,
}

pub static MOVIETEXTURE2ASSETFORMAT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2AssetFormat",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(MOVIETEXTURE2ASSETFORMAT_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for MovieTexture2AssetFormat {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIETEXTURE2ASSETFORMAT_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIETEXTURE2ASSETFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2AssetFormat-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTexture2AssetFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieTexture2Asset {
    pub _glacier_base: MovieTextureBaseAsset,
    pub chunk_guid: glacier_util::guid::Guid,
    pub chunk_size: u32,
    pub subtitle_chunk_guid: glacier_util::guid::Guid,
    pub subtitle_chunk_size: u32,
    pub has_localized_audio_tracks: bool,
    pub movie_filename: String,
    pub subtitle_filename: String,
    pub flipped: bool,
    pub use_stereo: bool,
}

pub trait MovieTexture2AssetTrait: MovieTextureBaseAssetTrait {
    fn chunk_guid(&self) -> &glacier_util::guid::Guid;
    fn chunk_size(&self) -> &u32;
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid;
    fn subtitle_chunk_size(&self) -> &u32;
    fn has_localized_audio_tracks(&self) -> &bool;
    fn movie_filename(&self) -> &String;
    fn subtitle_filename(&self) -> &String;
    fn flipped(&self) -> &bool;
    fn use_stereo(&self) -> &bool;
}

impl MovieTexture2AssetTrait for MovieTexture2Asset {
    fn chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.chunk_guid
    }
    fn chunk_size(&self) -> &u32 {
        &self.chunk_size
    }
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.subtitle_chunk_guid
    }
    fn subtitle_chunk_size(&self) -> &u32 {
        &self.subtitle_chunk_size
    }
    fn has_localized_audio_tracks(&self) -> &bool {
        &self.has_localized_audio_tracks
    }
    fn movie_filename(&self) -> &String {
        &self.movie_filename
    }
    fn subtitle_filename(&self) -> &String {
        &self.subtitle_filename
    }
    fn flipped(&self) -> &bool {
        &self.flipped
    }
    fn use_stereo(&self) -> &bool {
        &self.use_stereo
    }
}

impl MovieTextureBaseAssetTrait for MovieTexture2Asset {
}

impl super::core::AssetTrait for MovieTexture2Asset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MovieTexture2Asset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOVIETEXTURE2ASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2Asset",
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVIETEXTUREBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTexture2Asset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTexture2Asset, chunk_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTexture2Asset, chunk_size),
            },
            FieldInfoData {
                name: "SubtitleChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_chunk_guid),
            },
            FieldInfoData {
                name: "SubtitleChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_chunk_size),
            },
            FieldInfoData {
                name: "HasLocalizedAudioTracks",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTexture2Asset, has_localized_audio_tracks),
            },
            FieldInfoData {
                name: "MovieFilename",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieTexture2Asset, movie_filename),
            },
            FieldInfoData {
                name: "SubtitleFilename",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_filename),
            },
            FieldInfoData {
                name: "Flipped",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTexture2Asset, flipped),
            },
            FieldInfoData {
                name: "UseStereo",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTexture2Asset, use_stereo),
            },
        ],
    }),
    array_type: Some(MOVIETEXTURE2ASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTexture2Asset {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIETEXTURE2ASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIETEXTURE2ASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2Asset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTexture2Asset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait AudioStreamTrait: TypeObject {
    fn name(&self) -> &String;
    fn codec(&self) -> &AudioCodecType;
    fn language(&self) -> &String;
    fn length(&self) -> &f32;
    fn sample_rate(&self) -> &u32;
    fn bit_rate(&self) -> &u32;
    fn channels(&self) -> &u32;
    fn audio_filename(&self) -> &String;
}

impl AudioStreamTrait for AudioStream {
    fn name(&self) -> &String {
        &self.name
    }
    fn codec(&self) -> &AudioCodecType {
        &self.codec
    }
    fn language(&self) -> &String {
        &self.language
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn sample_rate(&self) -> &u32 {
        &self.sample_rate
    }
    fn bit_rate(&self) -> &u32 {
        &self.bit_rate
    }
    fn channels(&self) -> &u32 {
        &self.channels
    }
    fn audio_filename(&self) -> &String {
        &self.audio_filename
    }
}

pub static AUDIOSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioStream",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioStream as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AudioStream, name),
            },
            FieldInfoData {
                name: "Codec",
                flags: MemberInfoFlags::new(0),
                field_type: "AudioCodecType",
                rust_offset: offset_of!(AudioStream, codec),
            },
            FieldInfoData {
                name: "Language",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AudioStream, language),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioStream, length),
            },
            FieldInfoData {
                name: "SampleRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioStream, sample_rate),
            },
            FieldInfoData {
                name: "BitRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioStream, bit_rate),
            },
            FieldInfoData {
                name: "Channels",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioStream, channels),
            },
            FieldInfoData {
                name: "AudioFilename",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AudioStream, audio_filename),
            },
        ],
    }),
    array_type: Some(AUDIOSTREAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for AudioStream {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOSTREAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUDIOSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("AudioStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait VideoStreamTrait: TypeObject {
    fn name(&self) -> &String;
    fn r#type(&self) -> &VideoStreamType;
    fn codec(&self) -> &VideoCodecType;
    fn width(&self) -> &i32;
    fn height(&self) -> &i32;
    fn length(&self) -> &f32;
    fn bit_rate(&self) -> &u32;
    fn frames(&self) -> &u32;
    fn fps(&self) -> &f32;
}

impl VideoStreamTrait for VideoStream {
    fn name(&self) -> &String {
        &self.name
    }
    fn r#type(&self) -> &VideoStreamType {
        &self.r#type
    }
    fn codec(&self) -> &VideoCodecType {
        &self.codec
    }
    fn width(&self) -> &i32 {
        &self.width
    }
    fn height(&self) -> &i32 {
        &self.height
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn bit_rate(&self) -> &u32 {
        &self.bit_rate
    }
    fn frames(&self) -> &u32 {
        &self.frames
    }
    fn fps(&self) -> &f32 {
        &self.fps
    }
}

pub static VIDEOSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStream",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VideoStream as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VideoStream, name),
            },
            FieldInfoData {
                name: "type",
                flags: MemberInfoFlags::new(0),
                field_type: "VideoStreamType",
                rust_offset: offset_of!(VideoStream, r#type),
            },
            FieldInfoData {
                name: "Codec",
                flags: MemberInfoFlags::new(0),
                field_type: "VideoCodecType",
                rust_offset: offset_of!(VideoStream, codec),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VideoStream, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VideoStream, height),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VideoStream, length),
            },
            FieldInfoData {
                name: "BitRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VideoStream, bit_rate),
            },
            FieldInfoData {
                name: "Frames",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VideoStream, frames),
            },
            FieldInfoData {
                name: "Fps",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VideoStream, fps),
            },
        ],
    }),
    array_type: Some(VIDEOSTREAM_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for VideoStream {
    fn type_info(&self) -> &'static TypeInfo {
        VIDEOSTREAM_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VIDEOSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStream-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum AudioCodecType {
    #[default]
    AudioCodecType_Vorbis = 0,
    AudioCodecType_AAC = 1,
    AudioCodecType_Other = 2,
}

pub static AUDIOCODECTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCodecType",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(AUDIOCODECTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for AudioCodecType {
    fn type_info(&self) -> &'static TypeInfo {
        AUDIOCODECTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static AUDIOCODECTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCodecType-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("AudioCodecType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VideoCodecType {
    #[default]
    VideoCodecType_VP6 = 0,
    VideoCodecType_VP8 = 1,
    VideoCodecType_VP9 = 2,
    VideoCodecType_H264 = 3,
    VideoCodecType_JPEG = 4,
    VideoCodecType_Other = 5,
}

pub static VIDEOCODECTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoCodecType",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIDEOCODECTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VideoCodecType {
    fn type_info(&self) -> &'static TypeInfo {
        VIDEOCODECTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VIDEOCODECTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoCodecType-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoCodecType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum VideoStreamType {
    #[default]
    VideoStreamType_MainVideo = 0,
    VideoStreamType_Alpha = 1,
}

pub static VIDEOSTREAMTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStreamType",
    flags: MemberInfoFlags::new(49429),
    module: "MovieBase",
    data: TypeInfoData::Enum,
    array_type: Some(VIDEOSTREAMTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for VideoStreamType {
    fn type_info(&self) -> &'static TypeInfo {
        VIDEOSTREAMTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VIDEOSTREAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStreamType-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoStreamType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieTextureAsset {
    pub _glacier_base: MovieTextureBaseAsset,
    pub chunk_guid: glacier_util::guid::Guid,
    pub chunk_size: u32,
    pub subtitle_chunk_guid: glacier_util::guid::Guid,
    pub subtitle_chunk_size: u32,
    pub has_localized_audio_tracks: bool,
    pub override_background_music: bool,
    pub has_vp6: bool,
    pub has_vp8: bool,
}

pub trait MovieTextureAssetTrait: MovieTextureBaseAssetTrait {
    fn chunk_guid(&self) -> &glacier_util::guid::Guid;
    fn chunk_size(&self) -> &u32;
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid;
    fn subtitle_chunk_size(&self) -> &u32;
    fn has_localized_audio_tracks(&self) -> &bool;
    fn override_background_music(&self) -> &bool;
    fn has_vp6(&self) -> &bool;
    fn has_vp8(&self) -> &bool;
}

impl MovieTextureAssetTrait for MovieTextureAsset {
    fn chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.chunk_guid
    }
    fn chunk_size(&self) -> &u32 {
        &self.chunk_size
    }
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.subtitle_chunk_guid
    }
    fn subtitle_chunk_size(&self) -> &u32 {
        &self.subtitle_chunk_size
    }
    fn has_localized_audio_tracks(&self) -> &bool {
        &self.has_localized_audio_tracks
    }
    fn override_background_music(&self) -> &bool {
        &self.override_background_music
    }
    fn has_vp6(&self) -> &bool {
        &self.has_vp6
    }
    fn has_vp8(&self) -> &bool {
        &self.has_vp8
    }
}

impl MovieTextureBaseAssetTrait for MovieTextureAsset {
}

impl super::core::AssetTrait for MovieTextureAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MovieTextureAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOVIETEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureAsset",
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVIETEXTUREBASEASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTextureAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTextureAsset, chunk_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTextureAsset, chunk_size),
            },
            FieldInfoData {
                name: "SubtitleChunkGuid",
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTextureAsset, subtitle_chunk_guid),
            },
            FieldInfoData {
                name: "SubtitleChunkSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTextureAsset, subtitle_chunk_size),
            },
            FieldInfoData {
                name: "HasLocalizedAudioTracks",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, has_localized_audio_tracks),
            },
            FieldInfoData {
                name: "OverrideBackgroundMusic",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, override_background_music),
            },
            FieldInfoData {
                name: "HasVp6",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, has_vp6),
            },
            FieldInfoData {
                name: "HasVp8",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, has_vp8),
            },
        ],
    }),
    array_type: Some(MOVIETEXTUREASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTextureAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIETEXTUREASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIETEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTextureAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieTextureBaseAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait MovieTextureBaseAssetTrait: super::core::AssetTrait {
}

impl MovieTextureBaseAssetTrait for MovieTextureBaseAsset {
}

impl super::core::AssetTrait for MovieTextureBaseAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for MovieTextureBaseAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static MOVIETEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureBaseAsset",
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTextureBaseAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MOVIETEXTUREBASEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieTextureBaseAsset {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIETEXTUREBASEASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIETEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureBaseAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTextureBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieDynamicState {
    pub external_time: f32,
    pub volume: f32,
    pub play: bool,
    pub enable_state_stream_destroy_dependency_bug_workaround: bool,
    pub field_flag_override0: u8,
    pub field_flag_changed0: u8,
}

pub trait MovieDynamicStateTrait: TypeObject {
    fn external_time(&self) -> &f32;
    fn volume(&self) -> &f32;
    fn play(&self) -> &bool;
    fn enable_state_stream_destroy_dependency_bug_workaround(&self) -> &bool;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_changed0(&self) -> &u8;
}

impl MovieDynamicStateTrait for MovieDynamicState {
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn volume(&self) -> &f32 {
        &self.volume
    }
    fn play(&self) -> &bool {
        &self.play
    }
    fn enable_state_stream_destroy_dependency_bug_workaround(&self) -> &bool {
        &self.enable_state_stream_destroy_dependency_bug_workaround
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static MOVIEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExternalTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieDynamicState, external_time),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieDynamicState, volume),
            },
            FieldInfoData {
                name: "Play",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieDynamicState, play),
            },
            FieldInfoData {
                name: "EnableStateStreamDestroyDependencyBugWorkaround",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieDynamicState, enable_state_stream_destroy_dependency_bug_workaround),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MovieDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MovieDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MOVIEDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for MovieDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIEDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieStaticState {
    pub movie_texture: Option<Arc<Mutex<dyn MovieTextureBaseAssetTrait>>>,
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

pub trait MovieStaticStateTrait: TypeObject {
    fn movie_texture(&self) -> &Option<Arc<Mutex<dyn MovieTextureBaseAssetTrait>>>;
    fn movie_name(&self) -> &String;
    fn subtitle_name(&self) -> &String;
    fn audio_track_id(&self) -> &String;
    fn enable_subtitles(&self) -> &bool;
    fn r#loop(&self) -> &bool;
    fn preload(&self) -> &bool;
    fn use_sim_time(&self) -> &bool;
    fn pause_on_ending(&self) -> &bool;
    fn shader_block_handles(&self) -> &Vec<super::render_base::ShaderParameterBlockHandle>;
    fn shader_block_textures(&self) -> &Vec<super::render_base::ShaderBlockTexture>;
    fn texture_count(&self) -> &i32;
    fn decoder_threads(&self) -> &i32;
    fn stop_frame(&self) -> &i32;
    fn language_override(&self) -> &super::core::LanguageFormat;
    fn if_i_delete_this_other_stuff_breaks(&self) -> &super::render_base::ShaderBlockBool;
    fn field_flag_changed0(&self) -> &u16;
}

impl MovieStaticStateTrait for MovieStaticState {
    fn movie_texture(&self) -> &Option<Arc<Mutex<dyn MovieTextureBaseAssetTrait>>> {
        &self.movie_texture
    }
    fn movie_name(&self) -> &String {
        &self.movie_name
    }
    fn subtitle_name(&self) -> &String {
        &self.subtitle_name
    }
    fn audio_track_id(&self) -> &String {
        &self.audio_track_id
    }
    fn enable_subtitles(&self) -> &bool {
        &self.enable_subtitles
    }
    fn r#loop(&self) -> &bool {
        &self.r#loop
    }
    fn preload(&self) -> &bool {
        &self.preload
    }
    fn use_sim_time(&self) -> &bool {
        &self.use_sim_time
    }
    fn pause_on_ending(&self) -> &bool {
        &self.pause_on_ending
    }
    fn shader_block_handles(&self) -> &Vec<super::render_base::ShaderParameterBlockHandle> {
        &self.shader_block_handles
    }
    fn shader_block_textures(&self) -> &Vec<super::render_base::ShaderBlockTexture> {
        &self.shader_block_textures
    }
    fn texture_count(&self) -> &i32 {
        &self.texture_count
    }
    fn decoder_threads(&self) -> &i32 {
        &self.decoder_threads
    }
    fn stop_frame(&self) -> &i32 {
        &self.stop_frame
    }
    fn language_override(&self) -> &super::core::LanguageFormat {
        &self.language_override
    }
    fn if_i_delete_this_other_stuff_breaks(&self) -> &super::render_base::ShaderBlockBool {
        &self.if_i_delete_this_other_stuff_breaks
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
}

pub static MOVIESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieStaticState",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MovieTexture",
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(MovieStaticState, movie_texture),
            },
            FieldInfoData {
                name: "MovieName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieStaticState, movie_name),
            },
            FieldInfoData {
                name: "SubtitleName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieStaticState, subtitle_name),
            },
            FieldInfoData {
                name: "AudioTrackId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieStaticState, audio_track_id),
            },
            FieldInfoData {
                name: "EnableSubtitles",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, enable_subtitles),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, r#loop),
            },
            FieldInfoData {
                name: "Preload",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, preload),
            },
            FieldInfoData {
                name: "UseSimTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, use_sim_time),
            },
            FieldInfoData {
                name: "PauseOnEnding",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, pause_on_ending),
            },
            FieldInfoData {
                name: "ShaderBlockHandles",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderParameterBlockHandle-Array",
                rust_offset: offset_of!(MovieStaticState, shader_block_handles),
            },
            FieldInfoData {
                name: "ShaderBlockTextures",
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockTexture-Array",
                rust_offset: offset_of!(MovieStaticState, shader_block_textures),
            },
            FieldInfoData {
                name: "TextureCount",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieStaticState, texture_count),
            },
            FieldInfoData {
                name: "DecoderThreads",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieStaticState, decoder_threads),
            },
            FieldInfoData {
                name: "StopFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieStaticState, stop_frame),
            },
            FieldInfoData {
                name: "LanguageOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(MovieStaticState, language_override),
            },
            FieldInfoData {
                name: "IfIDeleteThisOtherStuffBreaks",
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockBool",
                rust_offset: offset_of!(MovieStaticState, if_i_delete_this_other_stuff_breaks),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(MovieStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(MOVIESTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIESTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieHandle {
}

pub trait MovieHandleTrait: TypeObject {
}

impl MovieHandleTrait for MovieHandle {
}

pub static MOVIEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieHandle",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MOVIEHANDLE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for MovieHandle {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIEHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MOVIEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UICancelAllSubtitlesMessage {
}

pub trait UICancelAllSubtitlesMessageTrait: TypeObject {
}

impl UICancelAllSubtitlesMessageTrait for UICancelAllSubtitlesMessage {
}

pub static UICANCELALLSUBTITLESMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICancelAllSubtitlesMessage",
    flags: MemberInfoFlags::new(36937),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICancelAllSubtitlesMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UICancelAllSubtitlesMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UICANCELALLSUBTITLESMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UICancelSubtitleMessage {
}

pub trait UICancelSubtitleMessageTrait: TypeObject {
}

impl UICancelSubtitleMessageTrait for UICancelSubtitleMessage {
}

pub static UICANCELSUBTITLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICancelSubtitleMessage",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICancelSubtitleMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UICancelSubtitleMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UICANCELSUBTITLEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UISubtitleMessage {
}

pub trait UISubtitleMessageTrait: TypeObject {
}

impl UISubtitleMessageTrait for UISubtitleMessage {
}

pub static UISUBTITLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISubtitleMessage",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISubtitleMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UISubtitleMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UISUBTITLEMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct UIPlayVideoMessage {
}

pub trait UIPlayVideoMessageTrait: TypeObject {
}

impl UIPlayVideoMessageTrait for UIPlayVideoMessage {
}

pub static UIPLAYVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIPlayVideoMessage",
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIPlayVideoMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIPlayVideoMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIPLAYVIDEOMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

