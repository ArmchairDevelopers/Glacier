use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_streaming_video_player_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(STREAMINGVIDEOPLAYERENTITYDATA_TYPE_INFO);
    registry.register_type(STREAMINGVIDEOPLAYERENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct StreamingVideoPlayerEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub video_u_r_l: String,
    pub texture_width: i32,
    pub texture_height: i32,
}

pub trait StreamingVideoPlayerEntityDataTrait: super::entity::EntityDataTrait {
    fn video_u_r_l(&self) -> &String;
    fn texture_width(&self) -> &i32;
    fn texture_height(&self) -> &i32;
}

impl StreamingVideoPlayerEntityDataTrait for StreamingVideoPlayerEntityData {
    fn video_u_r_l(&self) -> &String {
        &self.video_u_r_l
    }
    fn texture_width(&self) -> &i32 {
        &self.texture_width
    }
    fn texture_height(&self) -> &i32 {
        &self.texture_height
    }
}

impl super::entity::EntityDataTrait for StreamingVideoPlayerEntityData {
}

impl super::entity::GameObjectDataTrait for StreamingVideoPlayerEntityData {
}

impl super::core::DataBusPeerTrait for StreamingVideoPlayerEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
}

impl super::core::GameDataContainerTrait for StreamingVideoPlayerEntityData {
}

impl super::core::DataContainerTrait for StreamingVideoPlayerEntityData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static STREAMINGVIDEOPLAYERENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntityData",
    flags: MemberInfoFlags::new(101),
    module: "StreamingVideoPlayerShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StreamingVideoPlayerEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "VideoURL",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StreamingVideoPlayerEntityData, video_u_r_l),
            },
            FieldInfoData {
                name: "TextureWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StreamingVideoPlayerEntityData, texture_width),
            },
            FieldInfoData {
                name: "TextureHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StreamingVideoPlayerEntityData, texture_height),
            },
        ],
    }),
    array_type: Some(STREAMINGVIDEOPLAYERENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StreamingVideoPlayerEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STREAMINGVIDEOPLAYERENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STREAMINGVIDEOPLAYERENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StreamingVideoPlayerEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "StreamingVideoPlayerShared",
    data: TypeInfoData::Array("StreamingVideoPlayerEntityData"),
    array_type: None,
    alignment: 8,
};


