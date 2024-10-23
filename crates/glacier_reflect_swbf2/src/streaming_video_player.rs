use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamingVideoDynamicState {
}

pub trait StreamingVideoDynamicStateTrait: TypeObject {
}

impl StreamingVideoDynamicStateTrait for StreamingVideoDynamicState {
}

pub static STREAMINGVIDEODYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoDynamicState",
    name_hash: 16884618,
    flags: MemberInfoFlags::new(36937),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamingVideoDynamicState as Default>::default())),
            create_boxed: || Box::new(<StreamingVideoDynamicState as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(STREAMINGVIDEODYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StreamingVideoDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINGVIDEODYNAMICSTATE_TYPE_INFO
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


pub static STREAMINGVIDEODYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoDynamicState-Array",
    name_hash: 274235070,
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamingVideoStaticState {
    pub url: String,
    pub texture: super::render_base::TextureResourceHandle,
    pub field_flag_changed0: u8,
}

pub trait StreamingVideoStaticStateTrait: TypeObject {
    fn url(&self) -> &String;
    fn url_mut(&mut self) -> &mut String;
    fn texture(&self) -> &super::render_base::TextureResourceHandle;
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl StreamingVideoStaticStateTrait for StreamingVideoStaticState {
    fn url(&self) -> &String {
        &self.url
    }
    fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
    fn texture(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static STREAMINGVIDEOSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoStaticState",
    name_hash: 3616636487,
    flags: MemberInfoFlags::new(73),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamingVideoStaticState as Default>::default())),
            create_boxed: || Box::new(<StreamingVideoStaticState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Url",
                name_hash: 193455022,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StreamingVideoStaticState, url),
            },
            FieldInfoData {
                name: "Texture",
                name_hash: 3185041626,
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(StreamingVideoStaticState, texture),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(StreamingVideoStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(STREAMINGVIDEOSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StreamingVideoStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINGVIDEOSTATICSTATE_TYPE_INFO
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


pub static STREAMINGVIDEOSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoStaticState-Array",
    name_hash: 3143143283,
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamingVideoHandle {
}

pub trait StreamingVideoHandleTrait: TypeObject {
}

impl StreamingVideoHandleTrait for StreamingVideoHandle {
}

pub static STREAMINGVIDEOHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoHandle",
    name_hash: 3705016098,
    flags: MemberInfoFlags::new(73),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamingVideoHandle as Default>::default())),
            create_boxed: || Box::new(<StreamingVideoHandle as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(STREAMINGVIDEOHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StreamingVideoHandle {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINGVIDEOHANDLE_TYPE_INFO
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


pub static STREAMINGVIDEOHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoHandle-Array",
    name_hash: 1616629142,
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct StreamingVideoPlayerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait StreamingVideoPlayerEntityTrait: super::entity::EntityTrait {
}

impl StreamingVideoPlayerEntityTrait for StreamingVideoPlayerEntity {
}

impl super::entity::EntityTrait for StreamingVideoPlayerEntity {
}

impl super::entity::EntityBusPeerTrait for StreamingVideoPlayerEntity {
}

pub static STREAMINGVIDEOPLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntity",
    name_hash: 2013877344,
    flags: MemberInfoFlags::new(101),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(StreamingVideoPlayerEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamingVideoPlayerEntity as Default>::default())),
            create_boxed: || Box::new(<StreamingVideoPlayerEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(STREAMINGVIDEOPLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StreamingVideoPlayerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINGVIDEOPLAYERENTITY_TYPE_INFO
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


pub static STREAMINGVIDEOPLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntity-Array",
    name_hash: 1673628244,
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayer",
    data: TypeInfoData::Array("StreamingVideoPlayerEntity"),
    array_type: None,
    alignment: 8,
};


