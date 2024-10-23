use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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
    name_hash: 253722691,
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


pub static MOVIETEXTURE2ASSETFORMAT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2AssetFormat-Array",
    name_hash: 3097518967,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTexture2AssetFormat"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn chunk_size(&self) -> &u32;
    fn chunk_size_mut(&mut self) -> &mut u32;
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid;
    fn subtitle_chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn subtitle_chunk_size(&self) -> &u32;
    fn subtitle_chunk_size_mut(&mut self) -> &mut u32;
    fn has_localized_audio_tracks(&self) -> &bool;
    fn has_localized_audio_tracks_mut(&mut self) -> &mut bool;
    fn movie_filename(&self) -> &String;
    fn movie_filename_mut(&mut self) -> &mut String;
    fn subtitle_filename(&self) -> &String;
    fn subtitle_filename_mut(&mut self) -> &mut String;
    fn flipped(&self) -> &bool;
    fn flipped_mut(&mut self) -> &mut bool;
    fn use_stereo(&self) -> &bool;
    fn use_stereo_mut(&mut self) -> &mut bool;
}

impl MovieTexture2AssetTrait for MovieTexture2Asset {
    fn chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.chunk_guid
    }
    fn chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.chunk_guid
    }
    fn chunk_size(&self) -> &u32 {
        &self.chunk_size
    }
    fn chunk_size_mut(&mut self) -> &mut u32 {
        &mut self.chunk_size
    }
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.subtitle_chunk_guid
    }
    fn subtitle_chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.subtitle_chunk_guid
    }
    fn subtitle_chunk_size(&self) -> &u32 {
        &self.subtitle_chunk_size
    }
    fn subtitle_chunk_size_mut(&mut self) -> &mut u32 {
        &mut self.subtitle_chunk_size
    }
    fn has_localized_audio_tracks(&self) -> &bool {
        &self.has_localized_audio_tracks
    }
    fn has_localized_audio_tracks_mut(&mut self) -> &mut bool {
        &mut self.has_localized_audio_tracks
    }
    fn movie_filename(&self) -> &String {
        &self.movie_filename
    }
    fn movie_filename_mut(&mut self) -> &mut String {
        &mut self.movie_filename
    }
    fn subtitle_filename(&self) -> &String {
        &self.subtitle_filename
    }
    fn subtitle_filename_mut(&mut self) -> &mut String {
        &mut self.subtitle_filename
    }
    fn flipped(&self) -> &bool {
        &self.flipped
    }
    fn flipped_mut(&mut self) -> &mut bool {
        &mut self.flipped
    }
    fn use_stereo(&self) -> &bool {
        &self.use_stereo
    }
    fn use_stereo_mut(&mut self) -> &mut bool {
        &mut self.use_stereo
    }
}

impl MovieTextureBaseAssetTrait for MovieTexture2Asset {
}

impl super::core::AssetTrait for MovieTexture2Asset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MovieTexture2Asset {
}

pub static MOVIETEXTURE2ASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2Asset",
    name_hash: 2821566784,
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVIETEXTUREBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(MovieTexture2Asset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTexture2Asset as Default>::default())),
            create_boxed: || Box::new(<MovieTexture2Asset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ChunkGuid",
                name_hash: 3693055745,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTexture2Asset, chunk_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                name_hash: 3692630139,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTexture2Asset, chunk_size),
            },
            FieldInfoData {
                name: "SubtitleChunkGuid",
                name_hash: 3946824677,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_chunk_guid),
            },
            FieldInfoData {
                name: "SubtitleChunkSize",
                name_hash: 3946120031,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_chunk_size),
            },
            FieldInfoData {
                name: "HasLocalizedAudioTracks",
                name_hash: 300816458,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTexture2Asset, has_localized_audio_tracks),
            },
            FieldInfoData {
                name: "MovieFilename",
                name_hash: 2088885052,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieTexture2Asset, movie_filename),
            },
            FieldInfoData {
                name: "SubtitleFilename",
                name_hash: 1908521376,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieTexture2Asset, subtitle_filename),
            },
            FieldInfoData {
                name: "Flipped",
                name_hash: 2146772423,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTexture2Asset, flipped),
            },
            FieldInfoData {
                name: "UseStereo",
                name_hash: 301807836,
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


pub static MOVIETEXTURE2ASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTexture2Asset-Array",
    name_hash: 98584564,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTexture2Asset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String;
    fn codec(&self) -> &AudioCodecType;
    fn codec_mut(&mut self) -> &mut AudioCodecType;
    fn language(&self) -> &String;
    fn language_mut(&mut self) -> &mut String;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn sample_rate(&self) -> &u32;
    fn sample_rate_mut(&mut self) -> &mut u32;
    fn bit_rate(&self) -> &u32;
    fn bit_rate_mut(&mut self) -> &mut u32;
    fn channels(&self) -> &u32;
    fn channels_mut(&mut self) -> &mut u32;
    fn audio_filename(&self) -> &String;
    fn audio_filename_mut(&mut self) -> &mut String;
}

impl AudioStreamTrait for AudioStream {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn codec(&self) -> &AudioCodecType {
        &self.codec
    }
    fn codec_mut(&mut self) -> &mut AudioCodecType {
        &mut self.codec
    }
    fn language(&self) -> &String {
        &self.language
    }
    fn language_mut(&mut self) -> &mut String {
        &mut self.language
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn sample_rate(&self) -> &u32 {
        &self.sample_rate
    }
    fn sample_rate_mut(&mut self) -> &mut u32 {
        &mut self.sample_rate
    }
    fn bit_rate(&self) -> &u32 {
        &self.bit_rate
    }
    fn bit_rate_mut(&mut self) -> &mut u32 {
        &mut self.bit_rate
    }
    fn channels(&self) -> &u32 {
        &self.channels
    }
    fn channels_mut(&mut self) -> &mut u32 {
        &mut self.channels
    }
    fn audio_filename(&self) -> &String {
        &self.audio_filename
    }
    fn audio_filename_mut(&mut self) -> &mut String {
        &mut self.audio_filename
    }
}

pub static AUDIOSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioStream",
    name_hash: 466546895,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<AudioStream as Default>::default())),
            create_boxed: || Box::new(<AudioStream as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AudioStream, name),
            },
            FieldInfoData {
                name: "Codec",
                name_hash: 212395563,
                flags: MemberInfoFlags::new(0),
                field_type: "AudioCodecType",
                rust_offset: offset_of!(AudioStream, codec),
            },
            FieldInfoData {
                name: "Language",
                name_hash: 3872303031,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(AudioStream, language),
            },
            FieldInfoData {
                name: "Length",
                name_hash: 2906827577,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(AudioStream, length),
            },
            FieldInfoData {
                name: "SampleRate",
                name_hash: 604757697,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioStream, sample_rate),
            },
            FieldInfoData {
                name: "BitRate",
                name_hash: 2614557176,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioStream, bit_rate),
            },
            FieldInfoData {
                name: "Channels",
                name_hash: 1585412981,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(AudioStream, channels),
            },
            FieldInfoData {
                name: "AudioFilename",
                name_hash: 605297778,
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


pub static AUDIOSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioStream-Array",
    name_hash: 2818653691,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("AudioStream"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String;
    fn r#type(&self) -> &VideoStreamType;
    fn r#type_mut(&mut self) -> &mut VideoStreamType;
    fn codec(&self) -> &VideoCodecType;
    fn codec_mut(&mut self) -> &mut VideoCodecType;
    fn width(&self) -> &i32;
    fn width_mut(&mut self) -> &mut i32;
    fn height(&self) -> &i32;
    fn height_mut(&mut self) -> &mut i32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn bit_rate(&self) -> &u32;
    fn bit_rate_mut(&mut self) -> &mut u32;
    fn frames(&self) -> &u32;
    fn frames_mut(&mut self) -> &mut u32;
    fn fps(&self) -> &f32;
    fn fps_mut(&mut self) -> &mut f32;
}

impl VideoStreamTrait for VideoStream {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn r#type(&self) -> &VideoStreamType {
        &self.r#type
    }
    fn r#type_mut(&mut self) -> &mut VideoStreamType {
        &mut self.r#type
    }
    fn codec(&self) -> &VideoCodecType {
        &self.codec
    }
    fn codec_mut(&mut self) -> &mut VideoCodecType {
        &mut self.codec
    }
    fn width(&self) -> &i32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut i32 {
        &mut self.width
    }
    fn height(&self) -> &i32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut i32 {
        &mut self.height
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn bit_rate(&self) -> &u32 {
        &self.bit_rate
    }
    fn bit_rate_mut(&mut self) -> &mut u32 {
        &mut self.bit_rate
    }
    fn frames(&self) -> &u32 {
        &self.frames
    }
    fn frames_mut(&mut self) -> &mut u32 {
        &mut self.frames
    }
    fn fps(&self) -> &f32 {
        &self.fps
    }
    fn fps_mut(&mut self) -> &mut f32 {
        &mut self.fps
    }
}

pub static VIDEOSTREAM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStream",
    name_hash: 365984008,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<VideoStream as Default>::default())),
            create_boxed: || Box::new(<VideoStream as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                name_hash: 2088949890,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(VideoStream, name),
            },
            FieldInfoData {
                name: "type",
                name_hash: 2087944093,
                flags: MemberInfoFlags::new(0),
                field_type: "VideoStreamType",
                rust_offset: offset_of!(VideoStream, r#type),
            },
            FieldInfoData {
                name: "Codec",
                name_hash: 212395563,
                flags: MemberInfoFlags::new(0),
                field_type: "VideoCodecType",
                rust_offset: offset_of!(VideoStream, codec),
            },
            FieldInfoData {
                name: "Width",
                name_hash: 226981187,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VideoStream, width),
            },
            FieldInfoData {
                name: "Height",
                name_hash: 3054065626,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(VideoStream, height),
            },
            FieldInfoData {
                name: "Length",
                name_hash: 2906827577,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(VideoStream, length),
            },
            FieldInfoData {
                name: "BitRate",
                name_hash: 2614557176,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VideoStream, bit_rate),
            },
            FieldInfoData {
                name: "Frames",
                name_hash: 2535963883,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(VideoStream, frames),
            },
            FieldInfoData {
                name: "Fps",
                name_hash: 193444064,
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


pub static VIDEOSTREAM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStream-Array",
    name_hash: 530922300,
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
    name_hash: 3551783077,
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


pub static AUDIOCODECTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "AudioCodecType-Array",
    name_hash: 1064648977,
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
    name_hash: 3883851586,
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


pub static VIDEOCODECTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoCodecType-Array",
    name_hash: 1326739190,
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
    name_hash: 203612336,
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


pub static VIDEOSTREAMTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "VideoStreamType-Array",
    name_hash: 3602517508,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("VideoStreamType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn chunk_size(&self) -> &u32;
    fn chunk_size_mut(&mut self) -> &mut u32;
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid;
    fn subtitle_chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid;
    fn subtitle_chunk_size(&self) -> &u32;
    fn subtitle_chunk_size_mut(&mut self) -> &mut u32;
    fn has_localized_audio_tracks(&self) -> &bool;
    fn has_localized_audio_tracks_mut(&mut self) -> &mut bool;
    fn override_background_music(&self) -> &bool;
    fn override_background_music_mut(&mut self) -> &mut bool;
    fn has_vp6(&self) -> &bool;
    fn has_vp6_mut(&mut self) -> &mut bool;
    fn has_vp8(&self) -> &bool;
    fn has_vp8_mut(&mut self) -> &mut bool;
}

impl MovieTextureAssetTrait for MovieTextureAsset {
    fn chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.chunk_guid
    }
    fn chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.chunk_guid
    }
    fn chunk_size(&self) -> &u32 {
        &self.chunk_size
    }
    fn chunk_size_mut(&mut self) -> &mut u32 {
        &mut self.chunk_size
    }
    fn subtitle_chunk_guid(&self) -> &glacier_util::guid::Guid {
        &self.subtitle_chunk_guid
    }
    fn subtitle_chunk_guid_mut(&mut self) -> &mut glacier_util::guid::Guid {
        &mut self.subtitle_chunk_guid
    }
    fn subtitle_chunk_size(&self) -> &u32 {
        &self.subtitle_chunk_size
    }
    fn subtitle_chunk_size_mut(&mut self) -> &mut u32 {
        &mut self.subtitle_chunk_size
    }
    fn has_localized_audio_tracks(&self) -> &bool {
        &self.has_localized_audio_tracks
    }
    fn has_localized_audio_tracks_mut(&mut self) -> &mut bool {
        &mut self.has_localized_audio_tracks
    }
    fn override_background_music(&self) -> &bool {
        &self.override_background_music
    }
    fn override_background_music_mut(&mut self) -> &mut bool {
        &mut self.override_background_music
    }
    fn has_vp6(&self) -> &bool {
        &self.has_vp6
    }
    fn has_vp6_mut(&mut self) -> &mut bool {
        &mut self.has_vp6
    }
    fn has_vp8(&self) -> &bool {
        &self.has_vp8
    }
    fn has_vp8_mut(&mut self) -> &mut bool {
        &mut self.has_vp8
    }
}

impl MovieTextureBaseAssetTrait for MovieTextureAsset {
}

impl super::core::AssetTrait for MovieTextureAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MovieTextureAsset {
}

pub static MOVIETEXTUREASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureAsset",
    name_hash: 831810802,
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(MOVIETEXTUREBASEASSET_TYPE_INFO),
        super_class_offset: offset_of!(MovieTextureAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTextureAsset as Default>::default())),
            create_boxed: || Box::new(<MovieTextureAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ChunkGuid",
                name_hash: 3693055745,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTextureAsset, chunk_guid),
            },
            FieldInfoData {
                name: "ChunkSize",
                name_hash: 3692630139,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTextureAsset, chunk_size),
            },
            FieldInfoData {
                name: "SubtitleChunkGuid",
                name_hash: 3946824677,
                flags: MemberInfoFlags::new(0),
                field_type: "Guid",
                rust_offset: offset_of!(MovieTextureAsset, subtitle_chunk_guid),
            },
            FieldInfoData {
                name: "SubtitleChunkSize",
                name_hash: 3946120031,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(MovieTextureAsset, subtitle_chunk_size),
            },
            FieldInfoData {
                name: "HasLocalizedAudioTracks",
                name_hash: 300816458,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, has_localized_audio_tracks),
            },
            FieldInfoData {
                name: "OverrideBackgroundMusic",
                name_hash: 2360371614,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, override_background_music),
            },
            FieldInfoData {
                name: "HasVp6",
                name_hash: 3059605743,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieTextureAsset, has_vp6),
            },
            FieldInfoData {
                name: "HasVp8",
                name_hash: 3059605729,
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


pub static MOVIETEXTUREASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureAsset-Array",
    name_hash: 899417030,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTextureAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for MovieTextureBaseAsset {
}

pub static MOVIETEXTUREBASEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureBaseAsset",
    name_hash: 2700537223,
    flags: MemberInfoFlags::new(101),
    module: "MovieBase",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(MovieTextureBaseAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieTextureBaseAsset as Default>::default())),
            create_boxed: || Box::new(<MovieTextureBaseAsset as Default>::default()),
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


pub static MOVIETEXTUREBASEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieTextureBaseAsset-Array",
    name_hash: 78949939,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieTextureBaseAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    fn external_time_mut(&mut self) -> &mut f32;
    fn volume(&self) -> &f32;
    fn volume_mut(&mut self) -> &mut f32;
    fn play(&self) -> &bool;
    fn play_mut(&mut self) -> &mut bool;
    fn enable_state_stream_destroy_dependency_bug_workaround(&self) -> &bool;
    fn enable_state_stream_destroy_dependency_bug_workaround_mut(&mut self) -> &mut bool;
    fn field_flag_override0(&self) -> &u8;
    fn field_flag_override0_mut(&mut self) -> &mut u8;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl MovieDynamicStateTrait for MovieDynamicState {
    fn external_time(&self) -> &f32 {
        &self.external_time
    }
    fn external_time_mut(&mut self) -> &mut f32 {
        &mut self.external_time
    }
    fn volume(&self) -> &f32 {
        &self.volume
    }
    fn volume_mut(&mut self) -> &mut f32 {
        &mut self.volume
    }
    fn play(&self) -> &bool {
        &self.play
    }
    fn play_mut(&mut self) -> &mut bool {
        &mut self.play
    }
    fn enable_state_stream_destroy_dependency_bug_workaround(&self) -> &bool {
        &self.enable_state_stream_destroy_dependency_bug_workaround
    }
    fn enable_state_stream_destroy_dependency_bug_workaround_mut(&mut self) -> &mut bool {
        &mut self.enable_state_stream_destroy_dependency_bug_workaround
    }
    fn field_flag_override0(&self) -> &u8 {
        &self.field_flag_override0
    }
    fn field_flag_override0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_override0
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static MOVIEDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDynamicState",
    name_hash: 2303060543,
    flags: MemberInfoFlags::new(36937),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieDynamicState as Default>::default())),
            create_boxed: || Box::new(<MovieDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ExternalTime",
                name_hash: 2162678253,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieDynamicState, external_time),
            },
            FieldInfoData {
                name: "Volume",
                name_hash: 3158011725,
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(MovieDynamicState, volume),
            },
            FieldInfoData {
                name: "Play",
                name_hash: 2089460481,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieDynamicState, play),
            },
            FieldInfoData {
                name: "EnableStateStreamDestroyDependencyBugWorkaround",
                name_hash: 334754288,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieDynamicState, enable_state_stream_destroy_dependency_bug_workaround),
            },
            FieldInfoData {
                name: "FieldFlagOverride0",
                name_hash: 3558987183,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(MovieDynamicState, field_flag_override0),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static MOVIEDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDynamicState-Array",
    name_hash: 2116878219,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MovieStaticState {
    pub movie_texture: Option<LockedTypeObject /* MovieTextureBaseAsset */>,
    pub movie_name: String,
    pub subtitle_name: String,
    pub audio_track_id: String,
    pub enable_subtitles: bool,
    pub r#loop: bool,
    pub preload: bool,
    pub use_sim_time: bool,
    pub pause_on_ending: bool,
    pub shader_block_handles: Vec<BoxedTypeObject /* super::render_base::ShaderParameterBlockHandle */>,
    pub shader_block_textures: Vec<BoxedTypeObject /* super::render_base::ShaderBlockTexture */>,
    pub texture_count: i32,
    pub decoder_threads: i32,
    pub stop_frame: i32,
    pub language_override: super::core::LanguageFormat,
    pub if_i_delete_this_other_stuff_breaks: super::render_base::ShaderBlockBool,
    pub field_flag_changed0: u16,
}

pub trait MovieStaticStateTrait: TypeObject {
    fn movie_texture(&self) -> &Option<LockedTypeObject /* MovieTextureBaseAsset */>;
    fn movie_texture_mut(&mut self) -> &mut Option<LockedTypeObject /* MovieTextureBaseAsset */>;
    fn movie_name(&self) -> &String;
    fn movie_name_mut(&mut self) -> &mut String;
    fn subtitle_name(&self) -> &String;
    fn subtitle_name_mut(&mut self) -> &mut String;
    fn audio_track_id(&self) -> &String;
    fn audio_track_id_mut(&mut self) -> &mut String;
    fn enable_subtitles(&self) -> &bool;
    fn enable_subtitles_mut(&mut self) -> &mut bool;
    fn r#loop(&self) -> &bool;
    fn r#loop_mut(&mut self) -> &mut bool;
    fn preload(&self) -> &bool;
    fn preload_mut(&mut self) -> &mut bool;
    fn use_sim_time(&self) -> &bool;
    fn use_sim_time_mut(&mut self) -> &mut bool;
    fn pause_on_ending(&self) -> &bool;
    fn pause_on_ending_mut(&mut self) -> &mut bool;
    fn shader_block_handles(&self) -> &Vec<BoxedTypeObject /* super::render_base::ShaderParameterBlockHandle */>;
    fn shader_block_handles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::render_base::ShaderParameterBlockHandle */>;
    fn shader_block_textures(&self) -> &Vec<BoxedTypeObject /* super::render_base::ShaderBlockTexture */>;
    fn shader_block_textures_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::render_base::ShaderBlockTexture */>;
    fn texture_count(&self) -> &i32;
    fn texture_count_mut(&mut self) -> &mut i32;
    fn decoder_threads(&self) -> &i32;
    fn decoder_threads_mut(&mut self) -> &mut i32;
    fn stop_frame(&self) -> &i32;
    fn stop_frame_mut(&mut self) -> &mut i32;
    fn language_override(&self) -> &super::core::LanguageFormat;
    fn language_override_mut(&mut self) -> &mut super::core::LanguageFormat;
    fn if_i_delete_this_other_stuff_breaks(&self) -> &super::render_base::ShaderBlockBool;
    fn if_i_delete_this_other_stuff_breaks_mut(&mut self) -> &mut super::render_base::ShaderBlockBool;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl MovieStaticStateTrait for MovieStaticState {
    fn movie_texture(&self) -> &Option<LockedTypeObject /* MovieTextureBaseAsset */> {
        &self.movie_texture
    }
    fn movie_texture_mut(&mut self) -> &mut Option<LockedTypeObject /* MovieTextureBaseAsset */> {
        &mut self.movie_texture
    }
    fn movie_name(&self) -> &String {
        &self.movie_name
    }
    fn movie_name_mut(&mut self) -> &mut String {
        &mut self.movie_name
    }
    fn subtitle_name(&self) -> &String {
        &self.subtitle_name
    }
    fn subtitle_name_mut(&mut self) -> &mut String {
        &mut self.subtitle_name
    }
    fn audio_track_id(&self) -> &String {
        &self.audio_track_id
    }
    fn audio_track_id_mut(&mut self) -> &mut String {
        &mut self.audio_track_id
    }
    fn enable_subtitles(&self) -> &bool {
        &self.enable_subtitles
    }
    fn enable_subtitles_mut(&mut self) -> &mut bool {
        &mut self.enable_subtitles
    }
    fn r#loop(&self) -> &bool {
        &self.r#loop
    }
    fn r#loop_mut(&mut self) -> &mut bool {
        &mut self.r#loop
    }
    fn preload(&self) -> &bool {
        &self.preload
    }
    fn preload_mut(&mut self) -> &mut bool {
        &mut self.preload
    }
    fn use_sim_time(&self) -> &bool {
        &self.use_sim_time
    }
    fn use_sim_time_mut(&mut self) -> &mut bool {
        &mut self.use_sim_time
    }
    fn pause_on_ending(&self) -> &bool {
        &self.pause_on_ending
    }
    fn pause_on_ending_mut(&mut self) -> &mut bool {
        &mut self.pause_on_ending
    }
    fn shader_block_handles(&self) -> &Vec<BoxedTypeObject /* super::render_base::ShaderParameterBlockHandle */> {
        &self.shader_block_handles
    }
    fn shader_block_handles_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::render_base::ShaderParameterBlockHandle */> {
        &mut self.shader_block_handles
    }
    fn shader_block_textures(&self) -> &Vec<BoxedTypeObject /* super::render_base::ShaderBlockTexture */> {
        &self.shader_block_textures
    }
    fn shader_block_textures_mut(&mut self) -> &mut Vec<BoxedTypeObject /* super::render_base::ShaderBlockTexture */> {
        &mut self.shader_block_textures
    }
    fn texture_count(&self) -> &i32 {
        &self.texture_count
    }
    fn texture_count_mut(&mut self) -> &mut i32 {
        &mut self.texture_count
    }
    fn decoder_threads(&self) -> &i32 {
        &self.decoder_threads
    }
    fn decoder_threads_mut(&mut self) -> &mut i32 {
        &mut self.decoder_threads
    }
    fn stop_frame(&self) -> &i32 {
        &self.stop_frame
    }
    fn stop_frame_mut(&mut self) -> &mut i32 {
        &mut self.stop_frame
    }
    fn language_override(&self) -> &super::core::LanguageFormat {
        &self.language_override
    }
    fn language_override_mut(&mut self) -> &mut super::core::LanguageFormat {
        &mut self.language_override
    }
    fn if_i_delete_this_other_stuff_breaks(&self) -> &super::render_base::ShaderBlockBool {
        &self.if_i_delete_this_other_stuff_breaks
    }
    fn if_i_delete_this_other_stuff_breaks_mut(&mut self) -> &mut super::render_base::ShaderBlockBool {
        &mut self.if_i_delete_this_other_stuff_breaks
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static MOVIESTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieStaticState",
    name_hash: 3597615762,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieStaticState as Default>::default())),
            create_boxed: || Box::new(<MovieStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "MovieTexture",
                name_hash: 646397026,
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(MovieStaticState, movie_texture),
            },
            FieldInfoData {
                name: "MovieName",
                name_hash: 1720873978,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieStaticState, movie_name),
            },
            FieldInfoData {
                name: "SubtitleName",
                name_hash: 428940390,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieStaticState, subtitle_name),
            },
            FieldInfoData {
                name: "AudioTrackId",
                name_hash: 3970046769,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(MovieStaticState, audio_track_id),
            },
            FieldInfoData {
                name: "EnableSubtitles",
                name_hash: 2563389299,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, enable_subtitles),
            },
            FieldInfoData {
                name: "Loop",
                name_hash: 2089019673,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, r#loop),
            },
            FieldInfoData {
                name: "Preload",
                name_hash: 3463266116,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, preload),
            },
            FieldInfoData {
                name: "UseSimTime",
                name_hash: 2478855716,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, use_sim_time),
            },
            FieldInfoData {
                name: "PauseOnEnding",
                name_hash: 3821927929,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(MovieStaticState, pause_on_ending),
            },
            FieldInfoData {
                name: "ShaderBlockHandles",
                name_hash: 2334291932,
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderParameterBlockHandle-Array",
                rust_offset: offset_of!(MovieStaticState, shader_block_handles),
            },
            FieldInfoData {
                name: "ShaderBlockTextures",
                name_hash: 3498676617,
                flags: MemberInfoFlags::new(144),
                field_type: "ShaderBlockTexture-Array",
                rust_offset: offset_of!(MovieStaticState, shader_block_textures),
            },
            FieldInfoData {
                name: "TextureCount",
                name_hash: 2156227705,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieStaticState, texture_count),
            },
            FieldInfoData {
                name: "DecoderThreads",
                name_hash: 1438475910,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieStaticState, decoder_threads),
            },
            FieldInfoData {
                name: "StopFrame",
                name_hash: 382959072,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieStaticState, stop_frame),
            },
            FieldInfoData {
                name: "LanguageOverride",
                name_hash: 3061006915,
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(MovieStaticState, language_override),
            },
            FieldInfoData {
                name: "IfIDeleteThisOtherStuffBreaks",
                name_hash: 2148327398,
                flags: MemberInfoFlags::new(0),
                field_type: "ShaderBlockBool",
                rust_offset: offset_of!(MovieStaticState, if_i_delete_this_other_stuff_breaks),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static MOVIESTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieStaticState-Array",
    name_hash: 2085931430,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct MovieHandle {
}

pub trait MovieHandleTrait: TypeObject {
}

impl MovieHandleTrait for MovieHandle {
}

pub static MOVIEHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieHandle",
    name_hash: 1655637271,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieHandle as Default>::default())),
            create_boxed: || Box::new(<MovieHandle as Default>::default()),
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


pub static MOVIEHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieHandle-Array",
    name_hash: 4170395043,
    flags: MemberInfoFlags::new(145),
    module: "MovieBase",
    data: TypeInfoData::Array("MovieHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UICancelAllSubtitlesMessage {
}

pub trait UICancelAllSubtitlesMessageTrait: TypeObject {
}

impl UICancelAllSubtitlesMessageTrait for UICancelAllSubtitlesMessage {
}

pub static UICANCELALLSUBTITLESMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICancelAllSubtitlesMessage",
    name_hash: 4130404386,
    flags: MemberInfoFlags::new(36937),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICancelAllSubtitlesMessage as Default>::default())),
            create_boxed: || Box::new(<UICancelAllSubtitlesMessage as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct UICancelSubtitleMessage {
}

pub trait UICancelSubtitleMessageTrait: TypeObject {
}

impl UICancelSubtitleMessageTrait for UICancelSubtitleMessage {
}

pub static UICANCELSUBTITLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICancelSubtitleMessage",
    name_hash: 45154864,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICancelSubtitleMessage as Default>::default())),
            create_boxed: || Box::new(<UICancelSubtitleMessage as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct UISubtitleMessage {
}

pub trait UISubtitleMessageTrait: TypeObject {
}

impl UISubtitleMessageTrait for UISubtitleMessage {
}

pub static UISUBTITLEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISubtitleMessage",
    name_hash: 3281517334,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISubtitleMessage as Default>::default())),
            create_boxed: || Box::new(<UISubtitleMessage as Default>::default()),
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct UIPlayVideoMessage {
}

pub trait UIPlayVideoMessageTrait: TypeObject {
}

impl UIPlayVideoMessageTrait for UIPlayVideoMessage {
}

pub static UIPLAYVIDEOMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIPlayVideoMessage",
    name_hash: 3582159591,
    flags: MemberInfoFlags::new(73),
    module: "MovieBase",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIPlayVideoMessage as Default>::default())),
            create_boxed: || Box::new(<UIPlayVideoMessage as Default>::default()),
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

