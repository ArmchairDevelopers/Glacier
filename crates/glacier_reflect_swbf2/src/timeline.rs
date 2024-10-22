use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_timeline_types(registry: &mut TypeRegistry) {
    registry.register_type(VEC4TRACK_TYPE_INFO);
    registry.register_type(VEC4TRACK_ARRAY_TYPE_INFO);
    registry.register_type(VEC3TRACK_TYPE_INFO);
    registry.register_type(VEC3TRACK_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICEVENTTRACK_TYPE_INFO);
    registry.register_type(SCHEMATICEVENTTRACK_ARRAY_TYPE_INFO);
    registry.register_type(RECORDTRACKBASE_TYPE_INFO);
    registry.register_type(RECORDTRACKBASE_ARRAY_TYPE_INFO);
    registry.register_type(BOOLTRACK_TYPE_INFO);
    registry.register_type(BOOLTRACK_ARRAY_TYPE_INFO);
    registry.register_type(BOOKMARKTRACK_TYPE_INFO);
    registry.register_type(BOOKMARKTRACK_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMLAYER_TYPE_INFO);
    registry.register_type(TRANSFORMLAYER_ARRAY_TYPE_INFO);
    registry.register_type(SIMPLETRANSFORMLAYER_TYPE_INFO);
    registry.register_type(SIMPLETRANSFORMLAYER_ARRAY_TYPE_INFO);
    registry.register_type(MASTERTIMELINETRACK_TYPE_INFO);
    registry.register_type(MASTERTIMELINETRACK_ARRAY_TYPE_INFO);
    registry.register_type(LINKTRACK_TYPE_INFO);
    registry.register_type(LINKTRACK_ARRAY_TYPE_INFO);
    registry.register_type(LINKEDMASTERTIMELINETRACK_TYPE_INFO);
    registry.register_type(LINKEDMASTERTIMELINETRACK_ARRAY_TYPE_INFO);
    registry.register_type(LAYEREDTRANSFORMTRACK_TYPE_INFO);
    registry.register_type(LAYEREDTRANSFORMTRACK_ARRAY_TYPE_INFO);
    registry.register_type(IPROPERTYTRACK_TYPE_INFO);
    registry.register_type(IPROPERTYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(INTTRACK_TYPE_INFO);
    registry.register_type(INTTRACK_ARRAY_TYPE_INFO);
    registry.register_type(GROUPTRACK_TYPE_INFO);
    registry.register_type(GROUPTRACK_ARRAY_TYPE_INFO);
    registry.register_type(FLOATTRACK_TYPE_INFO);
    registry.register_type(FLOATTRACK_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALTIMETRACK_TYPE_INFO);
    registry.register_type(EXTERNALTIMETRACK_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRACK_TYPE_INFO);
    registry.register_type(ENTITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(LINKEDPROXYENTITYTRACK_TYPE_INFO);
    registry.register_type(LINKEDPROXYENTITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATEDPROXYENTITYTRACK_TYPE_INFO);
    registry.register_type(TEMPLATEDPROXYENTITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(PROXYENTITYTRACK_TYPE_INFO);
    registry.register_type(PROXYENTITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(DUMMYENTITYTRACK_TYPE_INFO);
    registry.register_type(DUMMYENTITYTRACK_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRACKBASE_TYPE_INFO);
    registry.register_type(ENTITYTRACKBASE_ARRAY_TYPE_INFO);
    registry.register_type(SCHEMATICPINTRACKDATA_TYPE_INFO);
    registry.register_type(SCHEMATICPINTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(RECORDTRACKCHILDRENDATA_TYPE_INFO);
    registry.register_type(RECORDTRACKCHILDRENDATA_ARRAY_TYPE_INFO);
    registry.register_type(RECORDTRACKBASEDATA_TYPE_INFO);
    registry.register_type(RECORDTRACKBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYTRACKBASEDATA_TYPE_INFO);
    registry.register_type(PROPERTYTRACKBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYREADERTRACKBASEDATA_TYPE_INFO);
    registry.register_type(PROPERTYREADERTRACKBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(MASTERTIMELINETRACKDATA_TYPE_INFO);
    registry.register_type(MASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(SLAVETIMELINEKEYFRAMEDATA_TYPE_INFO);
    registry.register_type(SLAVETIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINKTRACKDATA_TYPE_INFO);
    registry.register_type(LINKTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINKEDMASTERTIMELINETRACKDATA_TYPE_INFO);
    registry.register_type(LINKEDMASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINKEDMASTERTIMELINEKEYFRAME_TYPE_INFO);
    registry.register_type(LINKEDMASTERTIMELINEKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(LAYEREDTRANSFORMTRACKDATA_TYPE_INFO);
    registry.register_type(LAYEREDTRANSFORMTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTTRACKDATA_TYPE_INFO);
    registry.register_type(INTTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTKEYFRAME_TYPE_INFO);
    registry.register_type(INTKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(GROUPTRACKROOTDATA_TYPE_INFO);
    registry.register_type(GROUPTRACKROOTDATA_ARRAY_TYPE_INFO);
    registry.register_type(GROUPTRACKDATA_TYPE_INFO);
    registry.register_type(GROUPTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATTRACKDATA_TYPE_INFO);
    registry.register_type(FLOATTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(EXTERNALTIMETRACKDATA_TYPE_INFO);
    registry.register_type(EXTERNALTIMETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(OFFSETTYPE_TYPE_INFO);
    registry.register_type(OFFSETTYPE_ARRAY_TYPE_INFO);
    registry.register_type(EVENTTRACKDATA_TYPE_INFO);
    registry.register_type(EVENTTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(EVENTKEYFRAME_TYPE_INFO);
    registry.register_type(EVENTKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRACKDATA_TYPE_INFO);
    registry.register_type(ENTITYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(LINKEDPROXYENTITYTRACKDATA_TYPE_INFO);
    registry.register_type(LINKEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO);
    registry.register_type(TEMPLATEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROXYENTITYTRACKDATA_TYPE_INFO);
    registry.register_type(PROXYENTITYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(DUMMYENTITYTRACKDATA_TYPE_INFO);
    registry.register_type(DUMMYENTITYTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRACKBASEDATA_TYPE_INFO);
    registry.register_type(ENTITYTRACKBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(ENTITYTRACKSHARINGPOLICY_TYPE_INFO);
    registry.register_type(ENTITYTRACKSHARINGPOLICY_ARRAY_TYPE_INFO);
    registry.register_type(CURVEDATA_TYPE_INFO);
    registry.register_type(CURVEDATA_ARRAY_TYPE_INFO);
    registry.register_type(CURVETYPE_TYPE_INFO);
    registry.register_type(CURVETYPE_ARRAY_TYPE_INFO);
    registry.register_type(INFINITYTYPE_TYPE_INFO);
    registry.register_type(INFINITYTYPE_ARRAY_TYPE_INFO);
    registry.register_type(RGBCOLORTRACKDATA_TYPE_INFO);
    registry.register_type(RGBCOLORTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(RGBCOLORKEYFRAME_TYPE_INFO);
    registry.register_type(RGBCOLORKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(COLORTRACKDATA_TYPE_INFO);
    registry.register_type(COLORTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(COLORKEYFRAME_TYPE_INFO);
    registry.register_type(COLORKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(BOOLTRACKDATA_TYPE_INFO);
    registry.register_type(BOOLTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOLKEYFRAME_TYPE_INFO);
    registry.register_type(BOOLKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(BOOKMARKTRACKDATA_TYPE_INFO);
    registry.register_type(BOOKMARKTRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(BOOKMARKKEYFRAME_TYPE_INFO);
    registry.register_type(BOOKMARKKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(JUMPOFFSETTYPE_TYPE_INFO);
    registry.register_type(JUMPOFFSETTYPE_ARRAY_TYPE_INFO);
    registry.register_type(GUIDETRACKDATA_TYPE_INFO);
    registry.register_type(GUIDETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINETRACKDATA_TYPE_INFO);
    registry.register_type(TIMELINETRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEEXTRADATA_TYPE_INFO);
    registry.register_type(TIMELINEEXTRADATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINETRACKDATACONDITIONSBASE_TYPE_INFO);
    registry.register_type(TIMELINETRACKDATACONDITIONSBASE_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEKEYFRAMEDATA_TYPE_INFO);
    registry.register_type(TIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINETELEPORTHELPER_TYPE_INFO);
    registry.register_type(TIMELINETELEPORTHELPER_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEENTITYDATA_TYPE_INFO);
    registry.register_type(TIMELINEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEDATA_TYPE_INFO);
    registry.register_type(TIMELINEDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEEDITORREINITIALIZESTATE_TYPE_INFO);
    registry.register_type(TIMELINEEDITORREINITIALIZESTATE_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINECLOCK_TYPE_INFO);
    registry.register_type(TIMELINECLOCK_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEFRAMERATE_TYPE_INFO);
    registry.register_type(TIMELINEFRAMERATE_ARRAY_TYPE_INFO);
    registry.register_type(FBXIMPORTDATA_TYPE_INFO);
    registry.register_type(FBXIMPORTDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEBUFFERINGHELPER_TYPE_INFO);
    registry.register_type(TIMELINEBUFFERINGHELPER_ARRAY_TYPE_INFO);
    registry.register_type(VEC4TRACKDATA_TYPE_INFO);
    registry.register_type(VEC4TRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3TRACKDATA_TYPE_INFO);
    registry.register_type(VEC3TRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2TRACKDATA_TYPE_INFO);
    registry.register_type(VEC2TRACKDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMLAYERDATA_TYPE_INFO);
    registry.register_type(TRANSFORMLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(LAYEREDTRANSFORM_BLENDTYPE_TYPE_INFO);
    registry.register_type(LAYEREDTRANSFORM_BLENDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(STARTINGLOCATIONTRANSFORMLAYERDATA_TYPE_INFO);
    registry.register_type(STARTINGLOCATIONTRANSFORMLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(SCALETRANSFORMLAYERDATA_TYPE_INFO);
    registry.register_type(SCALETRANSFORMLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(KEYEDTRANSFORMLAYERDATA_TYPE_INFO);
    registry.register_type(KEYEDTRANSFORMLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(QUATKEYFRAME_TYPE_INFO);
    registry.register_type(QUATKEYFRAME_ARRAY_TYPE_INFO);
    registry.register_type(GROUPTRANSFORMLAYERDATA_TYPE_INFO);
    registry.register_type(GROUPTRANSFORMLAYERDATA_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINETRACK_TYPE_INFO);
    registry.register_type(TIMELINETRACK_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEROOTTRACK_TYPE_INFO);
    registry.register_type(TIMELINEROOTTRACK_ARRAY_TYPE_INFO);
    registry.register_type(TIMELINEENTITY_TYPE_INFO);
    registry.register_type(TIMELINEENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct Vec4Track {
    pub _glacier_base: IPropertyTrack,
}

pub trait Vec4TrackTrait: IPropertyTrackTrait {
}

impl Vec4TrackTrait for Vec4Track {
}

impl IPropertyTrackTrait for Vec4Track {
}

impl TimelineTrackTrait for Vec4Track {
}

pub static VEC4TRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4Track",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec4Track as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEC4TRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4Track {
    fn type_info(&self) -> &'static TypeInfo {
        VEC4TRACK_TYPE_INFO
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


pub static VEC4TRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4Track-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec4Track"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3Track {
    pub _glacier_base: IPropertyTrack,
}

pub trait Vec3TrackTrait: IPropertyTrackTrait {
}

impl Vec3TrackTrait for Vec3Track {
}

impl IPropertyTrackTrait for Vec3Track {
}

impl TimelineTrackTrait for Vec3Track {
}

pub static VEC3TRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Track",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3Track as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEC3TRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3Track {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3TRACK_TYPE_INFO
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


pub static VEC3TRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Track-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec3Track"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicEventTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait SchematicEventTrackTrait: TimelineTrackTrait {
}

impl SchematicEventTrackTrait for SchematicEventTrack {
}

impl TimelineTrackTrait for SchematicEventTrack {
}

pub static SCHEMATICEVENTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicEventTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicEventTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICEVENTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicEventTrack {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICEVENTTRACK_TYPE_INFO
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


pub static SCHEMATICEVENTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicEventTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SchematicEventTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RecordTrackBase {
    pub _glacier_base: LinkTrack,
}

pub trait RecordTrackBaseTrait: LinkTrackTrait {
}

impl RecordTrackBaseTrait for RecordTrackBase {
}

impl LinkTrackTrait for RecordTrackBase {
}

impl TimelineTrackTrait for RecordTrackBase {
}

pub static RECORDTRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RecordTrackBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RECORDTRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RecordTrackBase {
    fn type_info(&self) -> &'static TypeInfo {
        RECORDTRACKBASE_TYPE_INFO
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


pub static RECORDTRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RecordTrackBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoolTrack {
    pub _glacier_base: IPropertyTrack,
}

pub trait BoolTrackTrait: IPropertyTrackTrait {
}

impl BoolTrackTrait for BoolTrack {
}

impl IPropertyTrackTrait for BoolTrack {
}

impl TimelineTrackTrait for BoolTrack {
}

pub static BOOLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoolTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BOOLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BoolTrack {
    fn type_info(&self) -> &'static TypeInfo {
        BOOLTRACK_TYPE_INFO
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


pub static BOOLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BoolTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BookmarkTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait BookmarkTrackTrait: TimelineTrackTrait {
}

impl BookmarkTrackTrait for BookmarkTrack {
}

impl TimelineTrackTrait for BookmarkTrack {
}

pub static BOOKMARKTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BookmarkTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(BOOKMARKTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BookmarkTrack {
    fn type_info(&self) -> &'static TypeInfo {
        BOOKMARKTRACK_TYPE_INFO
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


pub static BOOKMARKTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BookmarkTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransformLayer {
    pub _glacier_base: TimelineTrack,
}

pub trait TransformLayerTrait: TimelineTrackTrait {
}

impl TransformLayerTrait for TransformLayer {
}

impl TimelineTrackTrait for TransformLayer {
}

pub static TRANSFORMLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayer",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformLayer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMLAYER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformLayer {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMLAYER_TYPE_INFO
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


pub static TRANSFORMLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TransformLayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SimpleTransformLayer {
    pub _glacier_base: TransformLayer,
}

pub trait SimpleTransformLayerTrait: TransformLayerTrait {
}

impl SimpleTransformLayerTrait for SimpleTransformLayer {
}

impl TransformLayerTrait for SimpleTransformLayer {
}

impl TimelineTrackTrait for SimpleTransformLayer {
}

pub static SIMPLETRANSFORMLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleTransformLayer",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SimpleTransformLayer as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SIMPLETRANSFORMLAYER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SimpleTransformLayer {
    fn type_info(&self) -> &'static TypeInfo {
        SIMPLETRANSFORMLAYER_TYPE_INFO
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


pub static SIMPLETRANSFORMLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleTransformLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SimpleTransformLayer"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MasterTimelineTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait MasterTimelineTrackTrait: TimelineTrackTrait {
}

impl MasterTimelineTrackTrait for MasterTimelineTrack {
}

impl TimelineTrackTrait for MasterTimelineTrack {
}

pub static MASTERTIMELINETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MasterTimelineTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MASTERTIMELINETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MasterTimelineTrack {
    fn type_info(&self) -> &'static TypeInfo {
        MASTERTIMELINETRACK_TYPE_INFO
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


pub static MASTERTIMELINETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("MasterTimelineTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait LinkTrackTrait: TimelineTrackTrait {
}

impl LinkTrackTrait for LinkTrack {
}

impl TimelineTrackTrait for LinkTrack {
}

pub static LINKTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINKTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinkTrack {
    fn type_info(&self) -> &'static TypeInfo {
        LINKTRACK_TYPE_INFO
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


pub static LINKTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkedMasterTimelineTrack {
    pub _glacier_base: LinkTrack,
}

pub trait LinkedMasterTimelineTrackTrait: LinkTrackTrait {
}

impl LinkedMasterTimelineTrackTrait for LinkedMasterTimelineTrack {
}

impl LinkTrackTrait for LinkedMasterTimelineTrack {
}

impl TimelineTrackTrait for LinkedMasterTimelineTrack {
}

pub static LINKEDMASTERTIMELINETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkedMasterTimelineTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINKEDMASTERTIMELINETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinkedMasterTimelineTrack {
    fn type_info(&self) -> &'static TypeInfo {
        LINKEDMASTERTIMELINETRACK_TYPE_INFO
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


pub static LINKEDMASTERTIMELINETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedMasterTimelineTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LayeredTransformTrack {
    pub _glacier_base: IPropertyTrack,
}

pub trait LayeredTransformTrackTrait: IPropertyTrackTrait {
}

impl LayeredTransformTrackTrait for LayeredTransformTrack {
}

impl IPropertyTrackTrait for LayeredTransformTrack {
}

impl TimelineTrackTrait for LayeredTransformTrack {
}

pub static LAYEREDTRANSFORMTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LayeredTransformTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LAYEREDTRANSFORMTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LayeredTransformTrack {
    fn type_info(&self) -> &'static TypeInfo {
        LAYEREDTRANSFORMTRACK_TYPE_INFO
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


pub static LAYEREDTRANSFORMTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LayeredTransformTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IPropertyTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait IPropertyTrackTrait: TimelineTrackTrait {
}

impl IPropertyTrackTrait for IPropertyTrack {
}

impl TimelineTrackTrait for IPropertyTrack {
}

pub static IPROPERTYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPropertyTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IPropertyTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IPROPERTYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IPropertyTrack {
    fn type_info(&self) -> &'static TypeInfo {
        IPROPERTYTRACK_TYPE_INFO
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


pub static IPROPERTYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPropertyTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IPropertyTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntTrack {
    pub _glacier_base: IPropertyTrack,
}

pub trait IntTrackTrait: IPropertyTrackTrait {
}

impl IntTrackTrait for IntTrack {
}

impl IPropertyTrackTrait for IntTrack {
}

impl TimelineTrackTrait for IntTrack {
}

pub static INTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntTrack {
    fn type_info(&self) -> &'static TypeInfo {
        INTTRACK_TYPE_INFO
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


pub static INTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IntTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait GroupTrackTrait: TimelineTrackTrait {
}

impl GroupTrackTrait for GroupTrack {
}

impl TimelineTrackTrait for GroupTrack {
}

pub static GROUPTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(GROUPTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroupTrack {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPTRACK_TYPE_INFO
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


pub static GROUPTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatTrack {
    pub _glacier_base: IPropertyTrack,
}

pub trait FloatTrackTrait: IPropertyTrackTrait {
}

impl FloatTrackTrait for FloatTrack {
}

impl IPropertyTrackTrait for FloatTrack {
}

impl TimelineTrackTrait for FloatTrack {
}

pub static FLOATTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatTrack {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATTRACK_TYPE_INFO
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


pub static FLOATTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("FloatTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalTimeTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait ExternalTimeTrackTrait: TimelineTrackTrait {
}

impl ExternalTimeTrackTrait for ExternalTimeTrack {
}

impl TimelineTrackTrait for ExternalTimeTrack {
}

pub static EXTERNALTIMETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalTimeTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(EXTERNALTIMETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ExternalTimeTrack {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALTIMETRACK_TYPE_INFO
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


pub static EXTERNALTIMETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ExternalTimeTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntityTrack {
    pub _glacier_base: EntityTrackBase,
}

pub trait EntityTrackTrait: EntityTrackBaseTrait {
}

impl EntityTrackTrait for EntityTrack {
}

impl EntityTrackBaseTrait for EntityTrack {
}

impl TimelineTrackTrait for EntityTrack {
}

pub static ENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYTRACK_TYPE_INFO
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


pub static ENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkedProxyEntityTrack {
    pub _glacier_base: TemplatedProxyEntityTrack,
}

pub trait LinkedProxyEntityTrackTrait: TemplatedProxyEntityTrackTrait {
}

impl LinkedProxyEntityTrackTrait for LinkedProxyEntityTrack {
}

impl TemplatedProxyEntityTrackTrait for LinkedProxyEntityTrack {
}

impl ProxyEntityTrackTrait for LinkedProxyEntityTrack {
}

impl EntityTrackBaseTrait for LinkedProxyEntityTrack {
}

impl TimelineTrackTrait for LinkedProxyEntityTrack {
}

pub static LINKEDPROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEMPLATEDPROXYENTITYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkedProxyEntityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINKEDPROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinkedProxyEntityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        LINKEDPROXYENTITYTRACK_TYPE_INFO
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


pub static LINKEDPROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedProxyEntityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplatedProxyEntityTrack {
    pub _glacier_base: ProxyEntityTrack,
}

pub trait TemplatedProxyEntityTrackTrait: ProxyEntityTrackTrait {
}

impl TemplatedProxyEntityTrackTrait for TemplatedProxyEntityTrack {
}

impl ProxyEntityTrackTrait for TemplatedProxyEntityTrack {
}

impl EntityTrackBaseTrait for TemplatedProxyEntityTrack {
}

impl TimelineTrackTrait for TemplatedProxyEntityTrack {
}

pub static TEMPLATEDPROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROXYENTITYTRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplatedProxyEntityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TEMPLATEDPROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TemplatedProxyEntityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATEDPROXYENTITYTRACK_TYPE_INFO
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


pub static TEMPLATEDPROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TemplatedProxyEntityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProxyEntityTrack {
    pub _glacier_base: EntityTrackBase,
}

pub trait ProxyEntityTrackTrait: EntityTrackBaseTrait {
}

impl ProxyEntityTrackTrait for ProxyEntityTrack {
}

impl EntityTrackBaseTrait for ProxyEntityTrack {
}

impl TimelineTrackTrait for ProxyEntityTrack {
}

pub static PROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProxyEntityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProxyEntityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        PROXYENTITYTRACK_TYPE_INFO
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


pub static PROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ProxyEntityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DummyEntityTrack {
    pub _glacier_base: EntityTrackBase,
}

pub trait DummyEntityTrackTrait: EntityTrackBaseTrait {
}

impl DummyEntityTrackTrait for DummyEntityTrack {
}

impl EntityTrackBaseTrait for DummyEntityTrack {
}

impl TimelineTrackTrait for DummyEntityTrack {
}

pub static DUMMYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DummyEntityTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DUMMYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DummyEntityTrack {
    fn type_info(&self) -> &'static TypeInfo {
        DUMMYENTITYTRACK_TYPE_INFO
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


pub static DUMMYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("DummyEntityTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntityTrackBase {
    pub _glacier_base: TimelineTrack,
}

pub trait EntityTrackBaseTrait: TimelineTrackTrait {
}

impl EntityTrackBaseTrait for EntityTrackBase {
}

impl TimelineTrackTrait for EntityTrackBase {
}

pub static ENTITYTRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityTrackBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityTrackBase {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYTRACKBASE_TYPE_INFO
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


pub static ENTITYTRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SchematicPinTrackData {
    pub _glacier_base: TimelineTrackData,
    pub source_pin_id: i32,
    pub target_pin_id: i32,
    pub target_pin_name_hash: i32,
}

pub trait SchematicPinTrackDataTrait: TimelineTrackDataTrait {
    fn source_pin_id(&self) -> &i32;
    fn source_pin_id_mut(&mut self) -> &mut i32;
    fn target_pin_id(&self) -> &i32;
    fn target_pin_id_mut(&mut self) -> &mut i32;
    fn target_pin_name_hash(&self) -> &i32;
    fn target_pin_name_hash_mut(&mut self) -> &mut i32;
}

impl SchematicPinTrackDataTrait for SchematicPinTrackData {
    fn source_pin_id(&self) -> &i32 {
        &self.source_pin_id
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        &mut self.source_pin_id
    }
    fn target_pin_id(&self) -> &i32 {
        &self.target_pin_id
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        &mut self.target_pin_id
    }
    fn target_pin_name_hash(&self) -> &i32 {
        &self.target_pin_name_hash
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        &mut self.target_pin_name_hash
    }
}

impl TimelineTrackDataTrait for SchematicPinTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for SchematicPinTrackData {
}

impl super::core::DataBusPeerTrait for SchematicPinTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SchematicPinTrackData {
}

impl super::core::DataContainerTrait for SchematicPinTrackData {
}

pub static SCHEMATICPINTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicPinTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SchematicPinTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SourcePinId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SchematicPinTrackData, source_pin_id),
            },
            FieldInfoData {
                name: "TargetPinId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SchematicPinTrackData, target_pin_id),
            },
            FieldInfoData {
                name: "TargetPinNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SchematicPinTrackData, target_pin_name_hash),
            },
        ],
    }),
    array_type: Some(SCHEMATICPINTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicPinTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        SCHEMATICPINTRACKDATA_TYPE_INFO
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


pub static SCHEMATICPINTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicPinTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SchematicPinTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RecordTrackChildrenData {
    pub _glacier_base: RecordTrackBaseData,
}

pub trait RecordTrackChildrenDataTrait: RecordTrackBaseDataTrait {
}

impl RecordTrackChildrenDataTrait for RecordTrackChildrenData {
}

impl RecordTrackBaseDataTrait for RecordTrackChildrenData {
    fn frames_to_skip_per_key(&self) -> &i32 {
        self._glacier_base.frames_to_skip_per_key()
    }
    fn frames_to_skip_per_key_mut(&mut self) -> &mut i32 {
        self._glacier_base.frames_to_skip_per_key_mut()
    }
}

impl LinkTrackDataTrait for RecordTrackChildrenData {
}

impl SchematicPinTrackDataTrait for RecordTrackChildrenData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for RecordTrackChildrenData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for RecordTrackChildrenData {
}

impl super::core::DataBusPeerTrait for RecordTrackChildrenData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RecordTrackChildrenData {
}

impl super::core::DataContainerTrait for RecordTrackChildrenData {
}

pub static RECORDTRACKCHILDRENDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackChildrenData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RECORDTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RecordTrackChildrenData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(RECORDTRACKCHILDRENDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RecordTrackChildrenData {
    fn type_info(&self) -> &'static TypeInfo {
        RECORDTRACKCHILDRENDATA_TYPE_INFO
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


pub static RECORDTRACKCHILDRENDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackChildrenData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RecordTrackChildrenData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RecordTrackBaseData {
    pub _glacier_base: LinkTrackData,
    pub frames_to_skip_per_key: i32,
}

pub trait RecordTrackBaseDataTrait: LinkTrackDataTrait {
    fn frames_to_skip_per_key(&self) -> &i32;
    fn frames_to_skip_per_key_mut(&mut self) -> &mut i32;
}

impl RecordTrackBaseDataTrait for RecordTrackBaseData {
    fn frames_to_skip_per_key(&self) -> &i32 {
        &self.frames_to_skip_per_key
    }
    fn frames_to_skip_per_key_mut(&mut self) -> &mut i32 {
        &mut self.frames_to_skip_per_key
    }
}

impl LinkTrackDataTrait for RecordTrackBaseData {
}

impl SchematicPinTrackDataTrait for RecordTrackBaseData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for RecordTrackBaseData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for RecordTrackBaseData {
}

impl super::core::DataBusPeerTrait for RecordTrackBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RecordTrackBaseData {
}

impl super::core::DataContainerTrait for RecordTrackBaseData {
}

pub static RECORDTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RecordTrackBaseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FramesToSkipPerKey",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(RecordTrackBaseData, frames_to_skip_per_key),
            },
        ],
    }),
    array_type: Some(RECORDTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RecordTrackBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        RECORDTRACKBASEDATA_TYPE_INFO
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


pub static RECORDTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RecordTrackBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PropertyTrackBaseData {
    pub _glacier_base: SchematicPinTrackData,
}

pub trait PropertyTrackBaseDataTrait: SchematicPinTrackDataTrait {
}

impl PropertyTrackBaseDataTrait for PropertyTrackBaseData {
}

impl SchematicPinTrackDataTrait for PropertyTrackBaseData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for PropertyTrackBaseData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for PropertyTrackBaseData {
}

impl super::core::DataBusPeerTrait for PropertyTrackBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PropertyTrackBaseData {
}

impl super::core::DataContainerTrait for PropertyTrackBaseData {
}

pub static PROPERTYTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyTrackBaseData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyTrackBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYTRACKBASEDATA_TYPE_INFO
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


pub static PROPERTYTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("PropertyTrackBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PropertyReaderTrackBaseData {
    pub _glacier_base: SchematicPinTrackData,
    pub realm: super::core::Realm,
}

pub trait PropertyReaderTrackBaseDataTrait: SchematicPinTrackDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl PropertyReaderTrackBaseDataTrait for PropertyReaderTrackBaseData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl SchematicPinTrackDataTrait for PropertyReaderTrackBaseData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for PropertyReaderTrackBaseData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for PropertyReaderTrackBaseData {
}

impl super::core::DataBusPeerTrait for PropertyReaderTrackBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PropertyReaderTrackBaseData {
}

impl super::core::DataContainerTrait for PropertyReaderTrackBaseData {
}

pub static PROPERTYREADERTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyReaderTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyReaderTrackBaseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PropertyReaderTrackBaseData, realm),
            },
        ],
    }),
    array_type: Some(PROPERTYREADERTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyReaderTrackBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYREADERTRACKBASEDATA_TYPE_INFO
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


pub static PROPERTYREADERTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyReaderTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("PropertyReaderTrackBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MasterTimelineTrackData {
    pub _glacier_base: TimelineTrackData,
    pub keyframes: Vec<Option<Arc<Mutex<dyn SlaveTimelineKeyframeDataTrait>>>>,
    pub children: Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>,
}

pub trait MasterTimelineTrackDataTrait: TimelineTrackDataTrait {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn SlaveTimelineKeyframeDataTrait>>>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn SlaveTimelineKeyframeDataTrait>>>>;
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
}

impl MasterTimelineTrackDataTrait for MasterTimelineTrackData {
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn SlaveTimelineKeyframeDataTrait>>>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn SlaveTimelineKeyframeDataTrait>>>> {
        &mut self.keyframes
    }
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &mut self.children
    }
}

impl TimelineTrackDataTrait for MasterTimelineTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for MasterTimelineTrackData {
}

impl super::core::DataBusPeerTrait for MasterTimelineTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MasterTimelineTrackData {
}

impl super::core::DataContainerTrait for MasterTimelineTrackData {
}

pub static MASTERTIMELINETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MasterTimelineTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "SlaveTimelineKeyframeData-Array",
                rust_offset: offset_of!(MasterTimelineTrackData, keyframes),
            },
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "TimelineTrackData-Array",
                rust_offset: offset_of!(MasterTimelineTrackData, children),
            },
        ],
    }),
    array_type: Some(MASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MasterTimelineTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        MASTERTIMELINETRACKDATA_TYPE_INFO
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


pub static MASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("MasterTimelineTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SlaveTimelineKeyframeData {
    pub _glacier_base: TimelineKeyframeData,
    pub time: f32,
    pub length: f32,
    pub guide_time: f32,
    pub timeline_data: Option<Arc<Mutex<dyn TimelineEntityDataTrait>>>,
    pub slave_guid_chain: Vec<glacier_util::guid::Guid>,
}

pub trait SlaveTimelineKeyframeDataTrait: TimelineKeyframeDataTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
    fn guide_time(&self) -> &f32;
    fn guide_time_mut(&mut self) -> &mut f32;
    fn timeline_data(&self) -> &Option<Arc<Mutex<dyn TimelineEntityDataTrait>>>;
    fn timeline_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TimelineEntityDataTrait>>>;
    fn slave_guid_chain(&self) -> &Vec<glacier_util::guid::Guid>;
    fn slave_guid_chain_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid>;
}

impl SlaveTimelineKeyframeDataTrait for SlaveTimelineKeyframeData {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
    fn guide_time(&self) -> &f32 {
        &self.guide_time
    }
    fn guide_time_mut(&mut self) -> &mut f32 {
        &mut self.guide_time
    }
    fn timeline_data(&self) -> &Option<Arc<Mutex<dyn TimelineEntityDataTrait>>> {
        &self.timeline_data
    }
    fn timeline_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TimelineEntityDataTrait>>> {
        &mut self.timeline_data
    }
    fn slave_guid_chain(&self) -> &Vec<glacier_util::guid::Guid> {
        &self.slave_guid_chain
    }
    fn slave_guid_chain_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid> {
        &mut self.slave_guid_chain
    }
}

impl TimelineKeyframeDataTrait for SlaveTimelineKeyframeData {
}

impl super::core::DataContainerTrait for SlaveTimelineKeyframeData {
}

pub static SLAVETIMELINEKEYFRAMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlaveTimelineKeyframeData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SlaveTimelineKeyframeData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlaveTimelineKeyframeData, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlaveTimelineKeyframeData, length),
            },
            FieldInfoData {
                name: "GuideTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(SlaveTimelineKeyframeData, guide_time),
            },
            FieldInfoData {
                name: "TimelineData",
                flags: MemberInfoFlags::new(0),
                field_type: "TimelineEntityData",
                rust_offset: offset_of!(SlaveTimelineKeyframeData, timeline_data),
            },
            FieldInfoData {
                name: "SlaveGuidChain",
                flags: MemberInfoFlags::new(144),
                field_type: "Guid-Array",
                rust_offset: offset_of!(SlaveTimelineKeyframeData, slave_guid_chain),
            },
        ],
    }),
    array_type: Some(SLAVETIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SlaveTimelineKeyframeData {
    fn type_info(&self) -> &'static TypeInfo {
        SLAVETIMELINEKEYFRAMEDATA_TYPE_INFO
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


pub static SLAVETIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlaveTimelineKeyframeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SlaveTimelineKeyframeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkTrackData {
    pub _glacier_base: SchematicPinTrackData,
}

pub trait LinkTrackDataTrait: SchematicPinTrackDataTrait {
}

impl LinkTrackDataTrait for LinkTrackData {
}

impl SchematicPinTrackDataTrait for LinkTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for LinkTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for LinkTrackData {
}

impl super::core::DataBusPeerTrait for LinkTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinkTrackData {
}

impl super::core::DataContainerTrait for LinkTrackData {
}

pub static LINKTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkTrackData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LINKTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        LINKTRACKDATA_TYPE_INFO
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


pub static LINKTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkedMasterTimelineTrackData {
    pub _glacier_base: LinkTrackData,
    pub linked_timeline_data: Option<Arc<Mutex<dyn TimelineEntityDataTrait>>>,
    pub linked_timeline_time_offset: f32,
    pub keyframes: Vec<Option<Arc<Mutex<dyn LinkedMasterTimelineKeyframeTrait>>>>,
}

pub trait LinkedMasterTimelineTrackDataTrait: LinkTrackDataTrait {
    fn linked_timeline_data(&self) -> &Option<Arc<Mutex<dyn TimelineEntityDataTrait>>>;
    fn linked_timeline_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TimelineEntityDataTrait>>>;
    fn linked_timeline_time_offset(&self) -> &f32;
    fn linked_timeline_time_offset_mut(&mut self) -> &mut f32;
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn LinkedMasterTimelineKeyframeTrait>>>>;
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn LinkedMasterTimelineKeyframeTrait>>>>;
}

impl LinkedMasterTimelineTrackDataTrait for LinkedMasterTimelineTrackData {
    fn linked_timeline_data(&self) -> &Option<Arc<Mutex<dyn TimelineEntityDataTrait>>> {
        &self.linked_timeline_data
    }
    fn linked_timeline_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TimelineEntityDataTrait>>> {
        &mut self.linked_timeline_data
    }
    fn linked_timeline_time_offset(&self) -> &f32 {
        &self.linked_timeline_time_offset
    }
    fn linked_timeline_time_offset_mut(&mut self) -> &mut f32 {
        &mut self.linked_timeline_time_offset
    }
    fn keyframes(&self) -> &Vec<Option<Arc<Mutex<dyn LinkedMasterTimelineKeyframeTrait>>>> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn LinkedMasterTimelineKeyframeTrait>>>> {
        &mut self.keyframes
    }
}

impl LinkTrackDataTrait for LinkedMasterTimelineTrackData {
}

impl SchematicPinTrackDataTrait for LinkedMasterTimelineTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for LinkedMasterTimelineTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for LinkedMasterTimelineTrackData {
}

impl super::core::DataBusPeerTrait for LinkedMasterTimelineTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinkedMasterTimelineTrackData {
}

impl super::core::DataContainerTrait for LinkedMasterTimelineTrackData {
}

pub static LINKEDMASTERTIMELINETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkedMasterTimelineTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LinkedTimelineData",
                flags: MemberInfoFlags::new(0),
                field_type: "TimelineEntityData",
                rust_offset: offset_of!(LinkedMasterTimelineTrackData, linked_timeline_data),
            },
            FieldInfoData {
                name: "LinkedTimelineTimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkedMasterTimelineTrackData, linked_timeline_time_offset),
            },
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "LinkedMasterTimelineKeyframe-Array",
                rust_offset: offset_of!(LinkedMasterTimelineTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(LINKEDMASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkedMasterTimelineTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        LINKEDMASTERTIMELINETRACKDATA_TYPE_INFO
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


pub static LINKEDMASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedMasterTimelineTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkedMasterTimelineKeyframe {
    pub _glacier_base: super::core::DataContainer,
    pub time: f32,
    pub length: f32,
}

pub trait LinkedMasterTimelineKeyframeTrait: super::core::DataContainerTrait {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn length(&self) -> &f32;
    fn length_mut(&mut self) -> &mut f32;
}

impl LinkedMasterTimelineKeyframeTrait for LinkedMasterTimelineKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn length(&self) -> &f32 {
        &self.length
    }
    fn length_mut(&mut self) -> &mut f32 {
        &mut self.length
    }
}

impl super::core::DataContainerTrait for LinkedMasterTimelineKeyframe {
}

pub static LINKEDMASTERTIMELINEKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkedMasterTimelineKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkedMasterTimelineKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(LinkedMasterTimelineKeyframe, length),
            },
        ],
    }),
    array_type: Some(LINKEDMASTERTIMELINEKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkedMasterTimelineKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        LINKEDMASTERTIMELINEKEYFRAME_TYPE_INFO
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


pub static LINKEDMASTERTIMELINEKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedMasterTimelineKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LayeredTransformTrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub layer_tracks: Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>>,
    pub use_timeline_space: bool,
    pub transform_space_enabled: bool,
}

pub trait LayeredTransformTrackDataTrait: PropertyTrackBaseDataTrait {
    fn layer_tracks(&self) -> &Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>>;
    fn layer_tracks_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>>;
    fn use_timeline_space(&self) -> &bool;
    fn use_timeline_space_mut(&mut self) -> &mut bool;
    fn transform_space_enabled(&self) -> &bool;
    fn transform_space_enabled_mut(&mut self) -> &mut bool;
}

impl LayeredTransformTrackDataTrait for LayeredTransformTrackData {
    fn layer_tracks(&self) -> &Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>> {
        &self.layer_tracks
    }
    fn layer_tracks_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>> {
        &mut self.layer_tracks
    }
    fn use_timeline_space(&self) -> &bool {
        &self.use_timeline_space
    }
    fn use_timeline_space_mut(&mut self) -> &mut bool {
        &mut self.use_timeline_space
    }
    fn transform_space_enabled(&self) -> &bool {
        &self.transform_space_enabled
    }
    fn transform_space_enabled_mut(&mut self) -> &mut bool {
        &mut self.transform_space_enabled
    }
}

impl PropertyTrackBaseDataTrait for LayeredTransformTrackData {
}

impl SchematicPinTrackDataTrait for LayeredTransformTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for LayeredTransformTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for LayeredTransformTrackData {
}

impl super::core::DataBusPeerTrait for LayeredTransformTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LayeredTransformTrackData {
}

impl super::core::DataContainerTrait for LayeredTransformTrackData {
}

pub static LAYEREDTRANSFORMTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LayeredTransformTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LayerTracks",
                flags: MemberInfoFlags::new(144),
                field_type: "TransformLayerData-Array",
                rust_offset: offset_of!(LayeredTransformTrackData, layer_tracks),
            },
            FieldInfoData {
                name: "UseTimelineSpace",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LayeredTransformTrackData, use_timeline_space),
            },
            FieldInfoData {
                name: "TransformSpaceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LayeredTransformTrackData, transform_space_enabled),
            },
        ],
    }),
    array_type: Some(LAYEREDTRANSFORMTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayeredTransformTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        LAYEREDTRANSFORMTRACKDATA_TYPE_INFO
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


pub static LAYEREDTRANSFORMTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LayeredTransformTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntTrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub keyframes: Vec<IntKeyframe>,
}

pub trait IntTrackDataTrait: PropertyTrackBaseDataTrait {
    fn keyframes(&self) -> &Vec<IntKeyframe>;
    fn keyframes_mut(&mut self) -> &mut Vec<IntKeyframe>;
}

impl IntTrackDataTrait for IntTrackData {
    fn keyframes(&self) -> &Vec<IntKeyframe> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<IntKeyframe> {
        &mut self.keyframes
    }
}

impl PropertyTrackBaseDataTrait for IntTrackData {
}

impl SchematicPinTrackDataTrait for IntTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for IntTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for IntTrackData {
}

impl super::core::DataBusPeerTrait for IntTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for IntTrackData {
}

impl super::core::DataContainerTrait for IntTrackData {
}

pub static INTTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "IntKeyframe-Array",
                rust_offset: offset_of!(IntTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(INTTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        INTTRACKDATA_TYPE_INFO
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


pub static INTTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IntTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntKeyframe {
    pub time: f32,
    pub value: i32,
}

pub trait IntKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn value(&self) -> &i32;
    fn value_mut(&mut self) -> &mut i32;
}

impl IntKeyframeTrait for IntKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn value(&self) -> &i32 {
        &self.value
    }
    fn value_mut(&mut self) -> &mut i32 {
        &mut self.value
    }
}

pub static INTKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(IntKeyframe, time),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IntKeyframe, value),
            },
        ],
    }),
    array_type: Some(INTKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IntKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        INTKEYFRAME_TYPE_INFO
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


pub static INTKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IntKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupTrackRootData {
    pub _glacier_base: super::core::Asset,
    pub children: Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>,
}

pub trait GroupTrackRootDataTrait: super::core::AssetTrait {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
}

impl GroupTrackRootDataTrait for GroupTrackRootData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &mut self.children
    }
}

impl super::core::AssetTrait for GroupTrackRootData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for GroupTrackRootData {
}

pub static GROUPTRACKROOTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackRootData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupTrackRootData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "TimelineTrackData-Array",
                rust_offset: offset_of!(GroupTrackRootData, children),
            },
        ],
    }),
    array_type: Some(GROUPTRACKROOTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupTrackRootData {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPTRACKROOTDATA_TYPE_INFO
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


pub static GROUPTRACKROOTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackRootData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTrackRootData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupTrackData {
    pub _glacier_base: TimelineTrackData,
    pub root_data: Option<Arc<Mutex<dyn GroupTrackRootDataTrait>>>,
}

pub trait GroupTrackDataTrait: TimelineTrackDataTrait {
    fn root_data(&self) -> &Option<Arc<Mutex<dyn GroupTrackRootDataTrait>>>;
    fn root_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GroupTrackRootDataTrait>>>;
}

impl GroupTrackDataTrait for GroupTrackData {
    fn root_data(&self) -> &Option<Arc<Mutex<dyn GroupTrackRootDataTrait>>> {
        &self.root_data
    }
    fn root_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn GroupTrackRootDataTrait>>> {
        &mut self.root_data
    }
}

impl TimelineTrackDataTrait for GroupTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for GroupTrackData {
}

impl super::core::DataBusPeerTrait for GroupTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GroupTrackData {
}

impl super::core::DataContainerTrait for GroupTrackData {
}

pub static GROUPTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "RootData",
                flags: MemberInfoFlags::new(0),
                field_type: "GroupTrackRootData",
                rust_offset: offset_of!(GroupTrackData, root_data),
            },
        ],
    }),
    array_type: Some(GROUPTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPTRACKDATA_TYPE_INFO
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


pub static GROUPTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatTrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub curve_data: Option<Arc<Mutex<dyn CurveDataTrait>>>,
}

pub trait FloatTrackDataTrait: PropertyTrackBaseDataTrait {
    fn curve_data(&self) -> &Option<Arc<Mutex<dyn CurveDataTrait>>>;
    fn curve_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CurveDataTrait>>>;
}

impl FloatTrackDataTrait for FloatTrackData {
    fn curve_data(&self) -> &Option<Arc<Mutex<dyn CurveDataTrait>>> {
        &self.curve_data
    }
    fn curve_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn CurveDataTrait>>> {
        &mut self.curve_data
    }
}

impl PropertyTrackBaseDataTrait for FloatTrackData {
}

impl SchematicPinTrackDataTrait for FloatTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for FloatTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for FloatTrackData {
}

impl super::core::DataBusPeerTrait for FloatTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FloatTrackData {
}

impl super::core::DataContainerTrait for FloatTrackData {
}

pub static FLOATTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CurveData",
                flags: MemberInfoFlags::new(0),
                field_type: "CurveData",
                rust_offset: offset_of!(FloatTrackData, curve_data),
            },
        ],
    }),
    array_type: Some(FLOATTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATTRACKDATA_TYPE_INFO
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


pub static FLOATTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("FloatTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ExternalTimeTrackData {
    pub _glacier_base: TimelineTrackData,
    pub external_time_priority: i32,
    pub external_time_offset_type: OffsetType,
    pub init_time_offset_type: OffsetType,
    pub current_time_offset_type: OffsetType,
}

pub trait ExternalTimeTrackDataTrait: TimelineTrackDataTrait {
    fn external_time_priority(&self) -> &i32;
    fn external_time_priority_mut(&mut self) -> &mut i32;
    fn external_time_offset_type(&self) -> &OffsetType;
    fn external_time_offset_type_mut(&mut self) -> &mut OffsetType;
    fn init_time_offset_type(&self) -> &OffsetType;
    fn init_time_offset_type_mut(&mut self) -> &mut OffsetType;
    fn current_time_offset_type(&self) -> &OffsetType;
    fn current_time_offset_type_mut(&mut self) -> &mut OffsetType;
}

impl ExternalTimeTrackDataTrait for ExternalTimeTrackData {
    fn external_time_priority(&self) -> &i32 {
        &self.external_time_priority
    }
    fn external_time_priority_mut(&mut self) -> &mut i32 {
        &mut self.external_time_priority
    }
    fn external_time_offset_type(&self) -> &OffsetType {
        &self.external_time_offset_type
    }
    fn external_time_offset_type_mut(&mut self) -> &mut OffsetType {
        &mut self.external_time_offset_type
    }
    fn init_time_offset_type(&self) -> &OffsetType {
        &self.init_time_offset_type
    }
    fn init_time_offset_type_mut(&mut self) -> &mut OffsetType {
        &mut self.init_time_offset_type
    }
    fn current_time_offset_type(&self) -> &OffsetType {
        &self.current_time_offset_type
    }
    fn current_time_offset_type_mut(&mut self) -> &mut OffsetType {
        &mut self.current_time_offset_type
    }
}

impl TimelineTrackDataTrait for ExternalTimeTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ExternalTimeTrackData {
}

impl super::core::DataBusPeerTrait for ExternalTimeTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ExternalTimeTrackData {
}

impl super::core::DataContainerTrait for ExternalTimeTrackData {
}

pub static EXTERNALTIMETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ExternalTimeTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExternalTimePriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ExternalTimeTrackData, external_time_priority),
            },
            FieldInfoData {
                name: "ExternalTimeOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: "OffsetType",
                rust_offset: offset_of!(ExternalTimeTrackData, external_time_offset_type),
            },
            FieldInfoData {
                name: "InitTimeOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: "OffsetType",
                rust_offset: offset_of!(ExternalTimeTrackData, init_time_offset_type),
            },
            FieldInfoData {
                name: "CurrentTimeOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: "OffsetType",
                rust_offset: offset_of!(ExternalTimeTrackData, current_time_offset_type),
            },
        ],
    }),
    array_type: Some(EXTERNALTIMETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalTimeTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        EXTERNALTIMETRACKDATA_TYPE_INFO
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


pub static EXTERNALTIMETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ExternalTimeTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum OffsetType {
    #[default]
    OffsetType_AbsoluteTime = 0,
    OffsetType_Relative = 1,
    OffsetType_AsPercent = 2,
}

pub static OFFSETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(OFFSETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OffsetType {
    fn type_info(&self) -> &'static TypeInfo {
        OFFSETTYPE_TYPE_INFO
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


pub static OFFSETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("OffsetType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventTrackData {
    pub _glacier_base: SchematicPinTrackData,
    pub keyframes: Vec<EventKeyframe>,
    pub fire_events_upon_skip: bool,
    pub update_properties_at_events: bool,
    pub anti_event_id: i32,
}

pub trait EventTrackDataTrait: SchematicPinTrackDataTrait {
    fn keyframes(&self) -> &Vec<EventKeyframe>;
    fn keyframes_mut(&mut self) -> &mut Vec<EventKeyframe>;
    fn fire_events_upon_skip(&self) -> &bool;
    fn fire_events_upon_skip_mut(&mut self) -> &mut bool;
    fn update_properties_at_events(&self) -> &bool;
    fn update_properties_at_events_mut(&mut self) -> &mut bool;
    fn anti_event_id(&self) -> &i32;
    fn anti_event_id_mut(&mut self) -> &mut i32;
}

impl EventTrackDataTrait for EventTrackData {
    fn keyframes(&self) -> &Vec<EventKeyframe> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<EventKeyframe> {
        &mut self.keyframes
    }
    fn fire_events_upon_skip(&self) -> &bool {
        &self.fire_events_upon_skip
    }
    fn fire_events_upon_skip_mut(&mut self) -> &mut bool {
        &mut self.fire_events_upon_skip
    }
    fn update_properties_at_events(&self) -> &bool {
        &self.update_properties_at_events
    }
    fn update_properties_at_events_mut(&mut self) -> &mut bool {
        &mut self.update_properties_at_events
    }
    fn anti_event_id(&self) -> &i32 {
        &self.anti_event_id
    }
    fn anti_event_id_mut(&mut self) -> &mut i32 {
        &mut self.anti_event_id
    }
}

impl SchematicPinTrackDataTrait for EventTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for EventTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for EventTrackData {
}

impl super::core::DataBusPeerTrait for EventTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EventTrackData {
}

impl super::core::DataContainerTrait for EventTrackData {
}

pub static EVENTTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "EventKeyframe-Array",
                rust_offset: offset_of!(EventTrackData, keyframes),
            },
            FieldInfoData {
                name: "FireEventsUponSkip",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EventTrackData, fire_events_upon_skip),
            },
            FieldInfoData {
                name: "UpdatePropertiesAtEvents",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EventTrackData, update_properties_at_events),
            },
            FieldInfoData {
                name: "AntiEventId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(EventTrackData, anti_event_id),
            },
        ],
    }),
    array_type: Some(EVENTTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTTRACKDATA_TYPE_INFO
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


pub static EVENTTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EventTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EventKeyframe {
    pub time: f32,
}

pub trait EventKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
}

impl EventKeyframeTrait for EventKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
}

pub static EVENTKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EventKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(EventKeyframe, time),
            },
        ],
    }),
    array_type: Some(EVENTKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        EVENTKEYFRAME_TYPE_INFO
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


pub static EVENTKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EventKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntityTrackData {
    pub _glacier_base: EntityTrackBaseData,
    pub guid_chain: Vec<glacier_util::guid::Guid>,
}

pub trait EntityTrackDataTrait: EntityTrackBaseDataTrait {
    fn guid_chain(&self) -> &Vec<glacier_util::guid::Guid>;
    fn guid_chain_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid>;
}

impl EntityTrackDataTrait for EntityTrackData {
    fn guid_chain(&self) -> &Vec<glacier_util::guid::Guid> {
        &self.guid_chain
    }
    fn guid_chain_mut(&mut self) -> &mut Vec<glacier_util::guid::Guid> {
        &mut self.guid_chain
    }
}

impl EntityTrackBaseDataTrait for EntityTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children_mut()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool {
        self._glacier_base.inherited_to_child_conversation_lines_mut()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn required_mut(&mut self) -> &mut bool {
        self._glacier_base.required_mut()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn handle_deleted_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.handle_deleted_entity_mut()
    }
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy_mut()
    }
}

impl TimelineTrackDataTrait for EntityTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for EntityTrackData {
}

impl super::core::DataBusPeerTrait for EntityTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EntityTrackData {
}

impl super::core::DataContainerTrait for EntityTrackData {
}

pub static ENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GuidChain",
                flags: MemberInfoFlags::new(144),
                field_type: "Guid-Array",
                rust_offset: offset_of!(EntityTrackData, guid_chain),
            },
        ],
    }),
    array_type: Some(ENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYTRACKDATA_TYPE_INFO
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


pub static ENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LinkedProxyEntityTrackData {
    pub _glacier_base: TemplatedProxyEntityTrackData,
    pub source_pin_id: i32,
}

pub trait LinkedProxyEntityTrackDataTrait: TemplatedProxyEntityTrackDataTrait {
    fn source_pin_id(&self) -> &i32;
    fn source_pin_id_mut(&mut self) -> &mut i32;
}

impl LinkedProxyEntityTrackDataTrait for LinkedProxyEntityTrackData {
    fn source_pin_id(&self) -> &i32 {
        &self.source_pin_id
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        &mut self.source_pin_id
    }
}

impl TemplatedProxyEntityTrackDataTrait for LinkedProxyEntityTrackData {
}

impl ProxyEntityTrackDataTrait for LinkedProxyEntityTrackData {
}

impl EntityTrackBaseDataTrait for LinkedProxyEntityTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children_mut()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool {
        self._glacier_base.inherited_to_child_conversation_lines_mut()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn required_mut(&mut self) -> &mut bool {
        self._glacier_base.required_mut()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn handle_deleted_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.handle_deleted_entity_mut()
    }
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy_mut()
    }
}

impl TimelineTrackDataTrait for LinkedProxyEntityTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for LinkedProxyEntityTrackData {
}

impl super::core::DataBusPeerTrait for LinkedProxyEntityTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LinkedProxyEntityTrackData {
}

impl super::core::DataContainerTrait for LinkedProxyEntityTrackData {
}

pub static LINKEDPROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LinkedProxyEntityTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SourcePinId",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LinkedProxyEntityTrackData, source_pin_id),
            },
        ],
    }),
    array_type: Some(LINKEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkedProxyEntityTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        LINKEDPROXYENTITYTRACKDATA_TYPE_INFO
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


pub static LINKEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedProxyEntityTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TemplatedProxyEntityTrackData {
    pub _glacier_base: ProxyEntityTrackData,
}

pub trait TemplatedProxyEntityTrackDataTrait: ProxyEntityTrackDataTrait {
}

impl TemplatedProxyEntityTrackDataTrait for TemplatedProxyEntityTrackData {
}

impl ProxyEntityTrackDataTrait for TemplatedProxyEntityTrackData {
}

impl EntityTrackBaseDataTrait for TemplatedProxyEntityTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children_mut()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool {
        self._glacier_base.inherited_to_child_conversation_lines_mut()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn required_mut(&mut self) -> &mut bool {
        self._glacier_base.required_mut()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn handle_deleted_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.handle_deleted_entity_mut()
    }
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy_mut()
    }
}

impl TimelineTrackDataTrait for TemplatedProxyEntityTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for TemplatedProxyEntityTrackData {
}

impl super::core::DataBusPeerTrait for TemplatedProxyEntityTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TemplatedProxyEntityTrackData {
}

impl super::core::DataContainerTrait for TemplatedProxyEntityTrackData {
}

pub static TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROXYENTITYTRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TemplatedProxyEntityTrackData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TEMPLATEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplatedProxyEntityTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO
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


pub static TEMPLATEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TemplatedProxyEntityTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ProxyEntityTrackData {
    pub _glacier_base: EntityTrackBaseData,
}

pub trait ProxyEntityTrackDataTrait: EntityTrackBaseDataTrait {
}

impl ProxyEntityTrackDataTrait for ProxyEntityTrackData {
}

impl EntityTrackBaseDataTrait for ProxyEntityTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children_mut()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool {
        self._glacier_base.inherited_to_child_conversation_lines_mut()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn required_mut(&mut self) -> &mut bool {
        self._glacier_base.required_mut()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn handle_deleted_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.handle_deleted_entity_mut()
    }
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy_mut()
    }
}

impl TimelineTrackDataTrait for ProxyEntityTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ProxyEntityTrackData {
}

impl super::core::DataBusPeerTrait for ProxyEntityTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ProxyEntityTrackData {
}

impl super::core::DataContainerTrait for ProxyEntityTrackData {
}

pub static PROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ProxyEntityTrackData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(PROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProxyEntityTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        PROXYENTITYTRACKDATA_TYPE_INFO
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


pub static PROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ProxyEntityTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DummyEntityTrackData {
    pub _glacier_base: EntityTrackBaseData,
}

pub trait DummyEntityTrackDataTrait: EntityTrackBaseDataTrait {
}

impl DummyEntityTrackDataTrait for DummyEntityTrackData {
}

impl EntityTrackBaseDataTrait for DummyEntityTrackData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children()
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        self._glacier_base.children_mut()
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        self._glacier_base.inherited_to_child_conversation_lines()
    }
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool {
        self._glacier_base.inherited_to_child_conversation_lines_mut()
    }
    fn required(&self) -> &bool {
        self._glacier_base.required()
    }
    fn required_mut(&mut self) -> &mut bool {
        self._glacier_base.required_mut()
    }
    fn handle_deleted_entity(&self) -> &bool {
        self._glacier_base.handle_deleted_entity()
    }
    fn handle_deleted_entity_mut(&mut self) -> &mut bool {
        self._glacier_base.handle_deleted_entity_mut()
    }
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy()
    }
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy {
        self._glacier_base.entity_sharing_policy_mut()
    }
}

impl TimelineTrackDataTrait for DummyEntityTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for DummyEntityTrackData {
}

impl super::core::DataBusPeerTrait for DummyEntityTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DummyEntityTrackData {
}

impl super::core::DataContainerTrait for DummyEntityTrackData {
}

pub static DUMMYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DummyEntityTrackData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DUMMYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DummyEntityTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        DUMMYENTITYTRACKDATA_TYPE_INFO
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


pub static DUMMYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("DummyEntityTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct EntityTrackBaseData {
    pub _glacier_base: TimelineTrackData,
    pub children: Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>,
    pub inherited_to_child_conversation_lines: bool,
    pub required: bool,
    pub handle_deleted_entity: bool,
    pub entity_sharing_policy: EntityTrackSharingPolicy,
}

pub trait EntityTrackBaseDataTrait: TimelineTrackDataTrait {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
    fn inherited_to_child_conversation_lines(&self) -> &bool;
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool;
    fn required(&self) -> &bool;
    fn required_mut(&mut self) -> &mut bool;
    fn handle_deleted_entity(&self) -> &bool;
    fn handle_deleted_entity_mut(&mut self) -> &mut bool;
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy;
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy;
}

impl EntityTrackBaseDataTrait for EntityTrackBaseData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &mut self.children
    }
    fn inherited_to_child_conversation_lines(&self) -> &bool {
        &self.inherited_to_child_conversation_lines
    }
    fn inherited_to_child_conversation_lines_mut(&mut self) -> &mut bool {
        &mut self.inherited_to_child_conversation_lines
    }
    fn required(&self) -> &bool {
        &self.required
    }
    fn required_mut(&mut self) -> &mut bool {
        &mut self.required
    }
    fn handle_deleted_entity(&self) -> &bool {
        &self.handle_deleted_entity
    }
    fn handle_deleted_entity_mut(&mut self) -> &mut bool {
        &mut self.handle_deleted_entity
    }
    fn entity_sharing_policy(&self) -> &EntityTrackSharingPolicy {
        &self.entity_sharing_policy
    }
    fn entity_sharing_policy_mut(&mut self) -> &mut EntityTrackSharingPolicy {
        &mut self.entity_sharing_policy
    }
}

impl TimelineTrackDataTrait for EntityTrackBaseData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for EntityTrackBaseData {
}

impl super::core::DataBusPeerTrait for EntityTrackBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for EntityTrackBaseData {
}

impl super::core::DataContainerTrait for EntityTrackBaseData {
}

pub static ENTITYTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<EntityTrackBaseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "TimelineTrackData-Array",
                rust_offset: offset_of!(EntityTrackBaseData, children),
            },
            FieldInfoData {
                name: "InheritedToChildConversationLines",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntityTrackBaseData, inherited_to_child_conversation_lines),
            },
            FieldInfoData {
                name: "Required",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntityTrackBaseData, required),
            },
            FieldInfoData {
                name: "HandleDeletedEntity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(EntityTrackBaseData, handle_deleted_entity),
            },
            FieldInfoData {
                name: "EntitySharingPolicy",
                flags: MemberInfoFlags::new(0),
                field_type: "EntityTrackSharingPolicy",
                rust_offset: offset_of!(EntityTrackBaseData, entity_sharing_policy),
            },
        ],
    }),
    array_type: Some(ENTITYTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityTrackBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYTRACKBASEDATA_TYPE_INFO
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


pub static ENTITYTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum EntityTrackSharingPolicy {
    #[default]
    EntityTrackSharingPolicy_FirstWins = 0,
    EntityTrackSharingPolicy_Combine = 1,
}

pub static ENTITYTRACKSHARINGPOLICY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackSharingPolicy",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYTRACKSHARINGPOLICY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityTrackSharingPolicy {
    fn type_info(&self) -> &'static TypeInfo {
        ENTITYTRACKSHARINGPOLICY_TYPE_INFO
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


pub static ENTITYTRACKSHARINGPOLICY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackSharingPolicy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackSharingPolicy"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct CurveData {
    pub _glacier_base: super::core::DataContainer,
    pub pre_infinity: InfinityType,
    pub post_infinity: InfinityType,
    pub is_weighted: bool,
    pub curve_type: CurveType,
    pub time: Vec<f32>,
    pub value: Vec<f32>,
    pub in_tan_x: Vec<f32>,
    pub in_tan_y: Vec<f32>,
    pub out_tan_x: Vec<f32>,
    pub out_tan_y: Vec<f32>,
}

pub trait CurveDataTrait: super::core::DataContainerTrait {
    fn pre_infinity(&self) -> &InfinityType;
    fn pre_infinity_mut(&mut self) -> &mut InfinityType;
    fn post_infinity(&self) -> &InfinityType;
    fn post_infinity_mut(&mut self) -> &mut InfinityType;
    fn is_weighted(&self) -> &bool;
    fn is_weighted_mut(&mut self) -> &mut bool;
    fn curve_type(&self) -> &CurveType;
    fn curve_type_mut(&mut self) -> &mut CurveType;
    fn time(&self) -> &Vec<f32>;
    fn time_mut(&mut self) -> &mut Vec<f32>;
    fn value(&self) -> &Vec<f32>;
    fn value_mut(&mut self) -> &mut Vec<f32>;
    fn in_tan_x(&self) -> &Vec<f32>;
    fn in_tan_x_mut(&mut self) -> &mut Vec<f32>;
    fn in_tan_y(&self) -> &Vec<f32>;
    fn in_tan_y_mut(&mut self) -> &mut Vec<f32>;
    fn out_tan_x(&self) -> &Vec<f32>;
    fn out_tan_x_mut(&mut self) -> &mut Vec<f32>;
    fn out_tan_y(&self) -> &Vec<f32>;
    fn out_tan_y_mut(&mut self) -> &mut Vec<f32>;
}

impl CurveDataTrait for CurveData {
    fn pre_infinity(&self) -> &InfinityType {
        &self.pre_infinity
    }
    fn pre_infinity_mut(&mut self) -> &mut InfinityType {
        &mut self.pre_infinity
    }
    fn post_infinity(&self) -> &InfinityType {
        &self.post_infinity
    }
    fn post_infinity_mut(&mut self) -> &mut InfinityType {
        &mut self.post_infinity
    }
    fn is_weighted(&self) -> &bool {
        &self.is_weighted
    }
    fn is_weighted_mut(&mut self) -> &mut bool {
        &mut self.is_weighted
    }
    fn curve_type(&self) -> &CurveType {
        &self.curve_type
    }
    fn curve_type_mut(&mut self) -> &mut CurveType {
        &mut self.curve_type
    }
    fn time(&self) -> &Vec<f32> {
        &self.time
    }
    fn time_mut(&mut self) -> &mut Vec<f32> {
        &mut self.time
    }
    fn value(&self) -> &Vec<f32> {
        &self.value
    }
    fn value_mut(&mut self) -> &mut Vec<f32> {
        &mut self.value
    }
    fn in_tan_x(&self) -> &Vec<f32> {
        &self.in_tan_x
    }
    fn in_tan_x_mut(&mut self) -> &mut Vec<f32> {
        &mut self.in_tan_x
    }
    fn in_tan_y(&self) -> &Vec<f32> {
        &self.in_tan_y
    }
    fn in_tan_y_mut(&mut self) -> &mut Vec<f32> {
        &mut self.in_tan_y
    }
    fn out_tan_x(&self) -> &Vec<f32> {
        &self.out_tan_x
    }
    fn out_tan_x_mut(&mut self) -> &mut Vec<f32> {
        &mut self.out_tan_x
    }
    fn out_tan_y(&self) -> &Vec<f32> {
        &self.out_tan_y
    }
    fn out_tan_y_mut(&mut self) -> &mut Vec<f32> {
        &mut self.out_tan_y
    }
}

impl super::core::DataContainerTrait for CurveData {
}

pub static CURVEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CurveData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PreInfinity",
                flags: MemberInfoFlags::new(0),
                field_type: "InfinityType",
                rust_offset: offset_of!(CurveData, pre_infinity),
            },
            FieldInfoData {
                name: "PostInfinity",
                flags: MemberInfoFlags::new(0),
                field_type: "InfinityType",
                rust_offset: offset_of!(CurveData, post_infinity),
            },
            FieldInfoData {
                name: "IsWeighted",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(CurveData, is_weighted),
            },
            FieldInfoData {
                name: "CurveType",
                flags: MemberInfoFlags::new(0),
                field_type: "CurveType",
                rust_offset: offset_of!(CurveData, curve_type),
            },
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(CurveData, time),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(CurveData, value),
            },
            FieldInfoData {
                name: "InTanX",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(CurveData, in_tan_x),
            },
            FieldInfoData {
                name: "InTanY",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(CurveData, in_tan_y),
            },
            FieldInfoData {
                name: "OutTanX",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(CurveData, out_tan_x),
            },
            FieldInfoData {
                name: "OutTanY",
                flags: MemberInfoFlags::new(144),
                field_type: "Float32-Array",
                rust_offset: offset_of!(CurveData, out_tan_y),
            },
        ],
    }),
    array_type: Some(CURVEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CurveData {
    fn type_info(&self) -> &'static TypeInfo {
        CURVEDATA_TYPE_INFO
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


pub static CURVEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("CurveData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum CurveType {
    #[default]
    CurveType_Zero = 0,
    CurveType_One = 1,
    CurveType_Constant = 2,
    CurveType_Basic_Linear = 3,
    CurveType_Basic_Step = 4,
    CurveType_Basic_StepNext = 5,
    CurveType_Complex = 6,
}

pub static CURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(CURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CurveType {
    fn type_info(&self) -> &'static TypeInfo {
        CURVETYPE_TYPE_INFO
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


pub static CURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("CurveType"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum InfinityType {
    #[default]
    InfinityType_Constant = 0,
    InfinityType_Linear = 1,
    InfinityType_Cycle = 2,
    InfinityType_CycleWithOffset = 3,
    InfinityType_Oscillate = 4,
    InfinityType_None = 5,
}

pub static INFINITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InfinityType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(INFINITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InfinityType {
    fn type_info(&self) -> &'static TypeInfo {
        INFINITYTYPE_TYPE_INFO
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


pub static INFINITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InfinityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("InfinityType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RgbColorTrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub keyframes: Vec<RgbColorKeyframe>,
}

pub trait RgbColorTrackDataTrait: PropertyTrackBaseDataTrait {
    fn keyframes(&self) -> &Vec<RgbColorKeyframe>;
    fn keyframes_mut(&mut self) -> &mut Vec<RgbColorKeyframe>;
}

impl RgbColorTrackDataTrait for RgbColorTrackData {
    fn keyframes(&self) -> &Vec<RgbColorKeyframe> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<RgbColorKeyframe> {
        &mut self.keyframes
    }
}

impl PropertyTrackBaseDataTrait for RgbColorTrackData {
}

impl SchematicPinTrackDataTrait for RgbColorTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for RgbColorTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for RgbColorTrackData {
}

impl super::core::DataBusPeerTrait for RgbColorTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for RgbColorTrackData {
}

impl super::core::DataContainerTrait for RgbColorTrackData {
}

pub static RGBCOLORTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RgbColorTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "RgbColorKeyframe-Array",
                rust_offset: offset_of!(RgbColorTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(RGBCOLORTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RgbColorTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        RGBCOLORTRACKDATA_TYPE_INFO
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


pub static RGBCOLORTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RgbColorTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct RgbColorKeyframe {
    pub time: f32,
    pub r_g_b_color: super::core::Vec3,
}

pub trait RgbColorKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn r_g_b_color(&self) -> &super::core::Vec3;
    fn r_g_b_color_mut(&mut self) -> &mut super::core::Vec3;
}

impl RgbColorKeyframeTrait for RgbColorKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn r_g_b_color(&self) -> &super::core::Vec3 {
        &self.r_g_b_color
    }
    fn r_g_b_color_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.r_g_b_color
    }
}

pub static RGBCOLORKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<RgbColorKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(RgbColorKeyframe, time),
            },
            FieldInfoData {
                name: "RGBColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(RgbColorKeyframe, r_g_b_color),
            },
        ],
    }),
    array_type: Some(RGBCOLORKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RgbColorKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        RGBCOLORKEYFRAME_TYPE_INFO
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


pub static RGBCOLORKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RgbColorKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorTrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub keyframes: Vec<ColorKeyframe>,
}

pub trait ColorTrackDataTrait: PropertyTrackBaseDataTrait {
    fn keyframes(&self) -> &Vec<ColorKeyframe>;
    fn keyframes_mut(&mut self) -> &mut Vec<ColorKeyframe>;
}

impl ColorTrackDataTrait for ColorTrackData {
    fn keyframes(&self) -> &Vec<ColorKeyframe> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<ColorKeyframe> {
        &mut self.keyframes
    }
}

impl PropertyTrackBaseDataTrait for ColorTrackData {
}

impl SchematicPinTrackDataTrait for ColorTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for ColorTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ColorTrackData {
}

impl super::core::DataBusPeerTrait for ColorTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ColorTrackData {
}

impl super::core::DataContainerTrait for ColorTrackData {
}

pub static COLORTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "ColorKeyframe-Array",
                rust_offset: offset_of!(ColorTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(COLORTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ColorTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        COLORTRACKDATA_TYPE_INFO
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


pub static COLORTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ColorTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorKeyframe {
    pub time: f32,
    pub r_g_b_color: super::core::Vec4,
}

pub trait ColorKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn r_g_b_color(&self) -> &super::core::Vec4;
    fn r_g_b_color_mut(&mut self) -> &mut super::core::Vec4;
}

impl ColorKeyframeTrait for ColorKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn r_g_b_color(&self) -> &super::core::Vec4 {
        &self.r_g_b_color
    }
    fn r_g_b_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.r_g_b_color
    }
}

pub static COLORKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ColorKeyframe, time),
            },
            FieldInfoData {
                name: "RGBColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ColorKeyframe, r_g_b_color),
            },
        ],
    }),
    array_type: Some(COLORKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        COLORKEYFRAME_TYPE_INFO
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


pub static COLORKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ColorKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoolTrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub keyframes: Vec<BoolKeyframe>,
}

pub trait BoolTrackDataTrait: PropertyTrackBaseDataTrait {
    fn keyframes(&self) -> &Vec<BoolKeyframe>;
    fn keyframes_mut(&mut self) -> &mut Vec<BoolKeyframe>;
}

impl BoolTrackDataTrait for BoolTrackData {
    fn keyframes(&self) -> &Vec<BoolKeyframe> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<BoolKeyframe> {
        &mut self.keyframes
    }
}

impl PropertyTrackBaseDataTrait for BoolTrackData {
}

impl SchematicPinTrackDataTrait for BoolTrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for BoolTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for BoolTrackData {
}

impl super::core::DataBusPeerTrait for BoolTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BoolTrackData {
}

impl super::core::DataContainerTrait for BoolTrackData {
}

pub static BOOLTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoolTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "BoolKeyframe-Array",
                rust_offset: offset_of!(BoolTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(BOOLTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        BOOLTRACKDATA_TYPE_INFO
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


pub static BOOLTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BoolTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BoolKeyframe {
    pub time: f32,
    pub value: bool,
}

pub trait BoolKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn value(&self) -> &bool;
    fn value_mut(&mut self) -> &mut bool;
}

impl BoolKeyframeTrait for BoolKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn value(&self) -> &bool {
        &self.value
    }
    fn value_mut(&mut self) -> &mut bool {
        &mut self.value
    }
}

pub static BOOLKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BoolKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BoolKeyframe, time),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BoolKeyframe, value),
            },
        ],
    }),
    array_type: Some(BOOLKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoolKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        BOOLKEYFRAME_TYPE_INFO
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


pub static BOOLKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BoolKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BookmarkTrackData {
    pub _glacier_base: TimelineTrackData,
    pub keyframes: Vec<BookmarkKeyframe>,
    pub jump_offset_type: JumpOffsetType,
    pub jump_time: f32,
}

pub trait BookmarkTrackDataTrait: TimelineTrackDataTrait {
    fn keyframes(&self) -> &Vec<BookmarkKeyframe>;
    fn keyframes_mut(&mut self) -> &mut Vec<BookmarkKeyframe>;
    fn jump_offset_type(&self) -> &JumpOffsetType;
    fn jump_offset_type_mut(&mut self) -> &mut JumpOffsetType;
    fn jump_time(&self) -> &f32;
    fn jump_time_mut(&mut self) -> &mut f32;
}

impl BookmarkTrackDataTrait for BookmarkTrackData {
    fn keyframes(&self) -> &Vec<BookmarkKeyframe> {
        &self.keyframes
    }
    fn keyframes_mut(&mut self) -> &mut Vec<BookmarkKeyframe> {
        &mut self.keyframes
    }
    fn jump_offset_type(&self) -> &JumpOffsetType {
        &self.jump_offset_type
    }
    fn jump_offset_type_mut(&mut self) -> &mut JumpOffsetType {
        &mut self.jump_offset_type
    }
    fn jump_time(&self) -> &f32 {
        &self.jump_time
    }
    fn jump_time_mut(&mut self) -> &mut f32 {
        &mut self.jump_time
    }
}

impl TimelineTrackDataTrait for BookmarkTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for BookmarkTrackData {
}

impl super::core::DataBusPeerTrait for BookmarkTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BookmarkTrackData {
}

impl super::core::DataContainerTrait for BookmarkTrackData {
}

pub static BOOKMARKTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BookmarkTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "BookmarkKeyframe-Array",
                rust_offset: offset_of!(BookmarkTrackData, keyframes),
            },
            FieldInfoData {
                name: "JumpOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: "JumpOffsetType",
                rust_offset: offset_of!(BookmarkTrackData, jump_offset_type),
            },
            FieldInfoData {
                name: "JumpTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BookmarkTrackData, jump_time),
            },
        ],
    }),
    array_type: Some(BOOKMARKTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BookmarkTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        BOOKMARKTRACKDATA_TYPE_INFO
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


pub static BOOKMARKTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BookmarkTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BookmarkKeyframe {
    pub time: f32,
    pub name_i_d: i32,
}

pub trait BookmarkKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn name_i_d(&self) -> &i32;
    fn name_i_d_mut(&mut self) -> &mut i32;
}

impl BookmarkKeyframeTrait for BookmarkKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn name_i_d(&self) -> &i32 {
        &self.name_i_d
    }
    fn name_i_d_mut(&mut self) -> &mut i32 {
        &mut self.name_i_d
    }
}

pub static BOOKMARKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkKeyframe",
    flags: MemberInfoFlags::new(32841),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BookmarkKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(BookmarkKeyframe, time),
            },
            FieldInfoData {
                name: "NameID",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(BookmarkKeyframe, name_i_d),
            },
        ],
    }),
    array_type: Some(BOOKMARKKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BookmarkKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        BOOKMARKKEYFRAME_TYPE_INFO
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


pub static BOOKMARKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BookmarkKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum JumpOffsetType {
    #[default]
    JumpOffsetType_AbsoluteTime = 0,
    JumpOffsetType_Relative = 1,
    JumpOffsetType_AsPercent = 2,
}

pub static JUMPOFFSETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpOffsetType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(JUMPOFFSETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for JumpOffsetType {
    fn type_info(&self) -> &'static TypeInfo {
        JUMPOFFSETTYPE_TYPE_INFO
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


pub static JUMPOFFSETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpOffsetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("JumpOffsetType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GuideTrackData {
    pub _glacier_base: TimelineTrackData,
    pub guide_track_priority: i32,
}

pub trait GuideTrackDataTrait: TimelineTrackDataTrait {
    fn guide_track_priority(&self) -> &i32;
    fn guide_track_priority_mut(&mut self) -> &mut i32;
}

impl GuideTrackDataTrait for GuideTrackData {
    fn guide_track_priority(&self) -> &i32 {
        &self.guide_track_priority
    }
    fn guide_track_priority_mut(&mut self) -> &mut i32 {
        &mut self.guide_track_priority
    }
}

impl TimelineTrackDataTrait for GuideTrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for GuideTrackData {
}

impl super::core::DataBusPeerTrait for GuideTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GuideTrackData {
}

impl super::core::DataContainerTrait for GuideTrackData {
}

pub static GUIDETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GuideTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GuideTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "GuideTrackPriority",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(GuideTrackData, guide_track_priority),
            },
        ],
    }),
    array_type: Some(GUIDETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GuideTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        GUIDETRACKDATA_TYPE_INFO
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


pub static GUIDETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GuideTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GuideTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineTrackData {
    pub _glacier_base: super::entity::GameObjectData,
    pub expose_pins: bool,
    pub is_disabled: bool,
    pub conditions: Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>>,
    pub update_pass_flags: u16,
    pub bucket_pre_children_order: u16,
    pub bucket_order: u16,
}

pub trait TimelineTrackDataTrait: super::entity::GameObjectDataTrait {
    fn expose_pins(&self) -> &bool;
    fn expose_pins_mut(&mut self) -> &mut bool;
    fn is_disabled(&self) -> &bool;
    fn is_disabled_mut(&mut self) -> &mut bool;
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>>;
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>>;
    fn update_pass_flags(&self) -> &u16;
    fn update_pass_flags_mut(&mut self) -> &mut u16;
    fn bucket_pre_children_order(&self) -> &u16;
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16;
    fn bucket_order(&self) -> &u16;
    fn bucket_order_mut(&mut self) -> &mut u16;
}

impl TimelineTrackDataTrait for TimelineTrackData {
    fn expose_pins(&self) -> &bool {
        &self.expose_pins
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        &mut self.expose_pins
    }
    fn is_disabled(&self) -> &bool {
        &self.is_disabled
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        &mut self.is_disabled
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        &self.conditions
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        &mut self.conditions
    }
    fn update_pass_flags(&self) -> &u16 {
        &self.update_pass_flags
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        &mut self.update_pass_flags
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        &self.bucket_pre_children_order
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        &mut self.bucket_pre_children_order
    }
    fn bucket_order(&self) -> &u16 {
        &self.bucket_order
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        &mut self.bucket_order
    }
}

impl super::entity::GameObjectDataTrait for TimelineTrackData {
}

impl super::core::DataBusPeerTrait for TimelineTrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TimelineTrackData {
}

impl super::core::DataContainerTrait for TimelineTrackData {
}

pub static TIMELINETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::GAMEOBJECTDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineTrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ExposePins",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineTrackData, expose_pins),
            },
            FieldInfoData {
                name: "IsDisabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineTrackData, is_disabled),
            },
            FieldInfoData {
                name: "Conditions",
                flags: MemberInfoFlags::new(144),
                field_type: "TimelineTrackDataConditionsBase-Array",
                rust_offset: offset_of!(TimelineTrackData, conditions),
            },
            FieldInfoData {
                name: "UpdatePassFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TimelineTrackData, update_pass_flags),
            },
            FieldInfoData {
                name: "BucketPreChildrenOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TimelineTrackData, bucket_pre_children_order),
            },
            FieldInfoData {
                name: "BucketOrder",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TimelineTrackData, bucket_order),
            },
        ],
    }),
    array_type: Some(TIMELINETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineTrackData {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINETRACKDATA_TYPE_INFO
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


pub static TIMELINETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineExtraData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TimelineExtraDataTrait: super::core::DataContainerTrait {
}

impl TimelineExtraDataTrait for TimelineExtraData {
}

impl super::core::DataContainerTrait for TimelineExtraData {
}

pub static TIMELINEEXTRADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineExtraData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineExtraData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEEXTRADATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineExtraData {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEEXTRADATA_TYPE_INFO
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


pub static TIMELINEEXTRADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineExtraData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineExtraData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineTrackDataConditionsBase {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TimelineTrackDataConditionsBaseTrait: super::core::DataContainerTrait {
}

impl TimelineTrackDataConditionsBaseTrait for TimelineTrackDataConditionsBase {
}

impl super::core::DataContainerTrait for TimelineTrackDataConditionsBase {
}

pub static TIMELINETRACKDATACONDITIONSBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackDataConditionsBase",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineTrackDataConditionsBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINETRACKDATACONDITIONSBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineTrackDataConditionsBase {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINETRACKDATACONDITIONSBASE_TYPE_INFO
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


pub static TIMELINETRACKDATACONDITIONSBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackDataConditionsBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTrackDataConditionsBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineKeyframeData {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TimelineKeyframeDataTrait: super::core::DataContainerTrait {
}

impl TimelineKeyframeDataTrait for TimelineKeyframeData {
}

impl super::core::DataContainerTrait for TimelineKeyframeData {
}

pub static TIMELINEKEYFRAMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineKeyframeData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineKeyframeData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineKeyframeData {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEKEYFRAMEDATA_TYPE_INFO
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


pub static TIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineKeyframeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineKeyframeData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineTeleportHelper {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TimelineTeleportHelperTrait: super::core::DataContainerTrait {
}

impl TimelineTeleportHelperTrait for TimelineTeleportHelper {
}

impl super::core::DataContainerTrait for TimelineTeleportHelper {
}

pub static TIMELINETELEPORTHELPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTeleportHelper",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineTeleportHelper as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINETELEPORTHELPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineTeleportHelper {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINETELEPORTHELPER_TYPE_INFO
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


pub static TIMELINETELEPORTHELPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTeleportHelper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTeleportHelper"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub timeline_data: Option<Arc<Mutex<dyn TimelineDataTrait>>>,
    pub auto_play: bool,
    pub delta_time_clock: TimelineClock,
    pub time_dilation_type: super::entity::TimeDeltaType,
    pub auto_init_connected_properties: bool,
    pub reset_time_on_started: bool,
    pub allow_animation_carry_forward: bool,
    pub always_end_on_pre_frame: bool,
    pub sync_timelines: bool,
    pub runtime_framerate: u8,
    pub blend_in_time: f32,
    pub blend_out_time: f32,
    pub looping: bool,
    pub infinite: bool,
    pub init_time: f32,
    pub start_time: f32,
    pub end_time: f32,
    pub playback_rate: f32,
    pub print_current_time: bool,
    pub update_pass_flags: u16,
}

pub trait TimelineEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn timeline_data(&self) -> &Option<Arc<Mutex<dyn TimelineDataTrait>>>;
    fn timeline_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TimelineDataTrait>>>;
    fn auto_play(&self) -> &bool;
    fn auto_play_mut(&mut self) -> &mut bool;
    fn delta_time_clock(&self) -> &TimelineClock;
    fn delta_time_clock_mut(&mut self) -> &mut TimelineClock;
    fn time_dilation_type(&self) -> &super::entity::TimeDeltaType;
    fn time_dilation_type_mut(&mut self) -> &mut super::entity::TimeDeltaType;
    fn auto_init_connected_properties(&self) -> &bool;
    fn auto_init_connected_properties_mut(&mut self) -> &mut bool;
    fn reset_time_on_started(&self) -> &bool;
    fn reset_time_on_started_mut(&mut self) -> &mut bool;
    fn allow_animation_carry_forward(&self) -> &bool;
    fn allow_animation_carry_forward_mut(&mut self) -> &mut bool;
    fn always_end_on_pre_frame(&self) -> &bool;
    fn always_end_on_pre_frame_mut(&mut self) -> &mut bool;
    fn sync_timelines(&self) -> &bool;
    fn sync_timelines_mut(&mut self) -> &mut bool;
    fn runtime_framerate(&self) -> &u8;
    fn runtime_framerate_mut(&mut self) -> &mut u8;
    fn blend_in_time(&self) -> &f32;
    fn blend_in_time_mut(&mut self) -> &mut f32;
    fn blend_out_time(&self) -> &f32;
    fn blend_out_time_mut(&mut self) -> &mut f32;
    fn looping(&self) -> &bool;
    fn looping_mut(&mut self) -> &mut bool;
    fn infinite(&self) -> &bool;
    fn infinite_mut(&mut self) -> &mut bool;
    fn init_time(&self) -> &f32;
    fn init_time_mut(&mut self) -> &mut f32;
    fn start_time(&self) -> &f32;
    fn start_time_mut(&mut self) -> &mut f32;
    fn end_time(&self) -> &f32;
    fn end_time_mut(&mut self) -> &mut f32;
    fn playback_rate(&self) -> &f32;
    fn playback_rate_mut(&mut self) -> &mut f32;
    fn print_current_time(&self) -> &bool;
    fn print_current_time_mut(&mut self) -> &mut bool;
    fn update_pass_flags(&self) -> &u16;
    fn update_pass_flags_mut(&mut self) -> &mut u16;
}

impl TimelineEntityDataTrait for TimelineEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn timeline_data(&self) -> &Option<Arc<Mutex<dyn TimelineDataTrait>>> {
        &self.timeline_data
    }
    fn timeline_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn TimelineDataTrait>>> {
        &mut self.timeline_data
    }
    fn auto_play(&self) -> &bool {
        &self.auto_play
    }
    fn auto_play_mut(&mut self) -> &mut bool {
        &mut self.auto_play
    }
    fn delta_time_clock(&self) -> &TimelineClock {
        &self.delta_time_clock
    }
    fn delta_time_clock_mut(&mut self) -> &mut TimelineClock {
        &mut self.delta_time_clock
    }
    fn time_dilation_type(&self) -> &super::entity::TimeDeltaType {
        &self.time_dilation_type
    }
    fn time_dilation_type_mut(&mut self) -> &mut super::entity::TimeDeltaType {
        &mut self.time_dilation_type
    }
    fn auto_init_connected_properties(&self) -> &bool {
        &self.auto_init_connected_properties
    }
    fn auto_init_connected_properties_mut(&mut self) -> &mut bool {
        &mut self.auto_init_connected_properties
    }
    fn reset_time_on_started(&self) -> &bool {
        &self.reset_time_on_started
    }
    fn reset_time_on_started_mut(&mut self) -> &mut bool {
        &mut self.reset_time_on_started
    }
    fn allow_animation_carry_forward(&self) -> &bool {
        &self.allow_animation_carry_forward
    }
    fn allow_animation_carry_forward_mut(&mut self) -> &mut bool {
        &mut self.allow_animation_carry_forward
    }
    fn always_end_on_pre_frame(&self) -> &bool {
        &self.always_end_on_pre_frame
    }
    fn always_end_on_pre_frame_mut(&mut self) -> &mut bool {
        &mut self.always_end_on_pre_frame
    }
    fn sync_timelines(&self) -> &bool {
        &self.sync_timelines
    }
    fn sync_timelines_mut(&mut self) -> &mut bool {
        &mut self.sync_timelines
    }
    fn runtime_framerate(&self) -> &u8 {
        &self.runtime_framerate
    }
    fn runtime_framerate_mut(&mut self) -> &mut u8 {
        &mut self.runtime_framerate
    }
    fn blend_in_time(&self) -> &f32 {
        &self.blend_in_time
    }
    fn blend_in_time_mut(&mut self) -> &mut f32 {
        &mut self.blend_in_time
    }
    fn blend_out_time(&self) -> &f32 {
        &self.blend_out_time
    }
    fn blend_out_time_mut(&mut self) -> &mut f32 {
        &mut self.blend_out_time
    }
    fn looping(&self) -> &bool {
        &self.looping
    }
    fn looping_mut(&mut self) -> &mut bool {
        &mut self.looping
    }
    fn infinite(&self) -> &bool {
        &self.infinite
    }
    fn infinite_mut(&mut self) -> &mut bool {
        &mut self.infinite
    }
    fn init_time(&self) -> &f32 {
        &self.init_time
    }
    fn init_time_mut(&mut self) -> &mut f32 {
        &mut self.init_time
    }
    fn start_time(&self) -> &f32 {
        &self.start_time
    }
    fn start_time_mut(&mut self) -> &mut f32 {
        &mut self.start_time
    }
    fn end_time(&self) -> &f32 {
        &self.end_time
    }
    fn end_time_mut(&mut self) -> &mut f32 {
        &mut self.end_time
    }
    fn playback_rate(&self) -> &f32 {
        &self.playback_rate
    }
    fn playback_rate_mut(&mut self) -> &mut f32 {
        &mut self.playback_rate
    }
    fn print_current_time(&self) -> &bool {
        &self.print_current_time
    }
    fn print_current_time_mut(&mut self) -> &mut bool {
        &mut self.print_current_time
    }
    fn update_pass_flags(&self) -> &u16 {
        &self.update_pass_flags
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        &mut self.update_pass_flags
    }
}

impl super::entity::EntityDataTrait for TimelineEntityData {
}

impl super::entity::GameObjectDataTrait for TimelineEntityData {
}

impl super::core::DataBusPeerTrait for TimelineEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TimelineEntityData {
}

impl super::core::DataContainerTrait for TimelineEntityData {
}

pub static TIMELINEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(TimelineEntityData, realm),
            },
            FieldInfoData {
                name: "TimelineData",
                flags: MemberInfoFlags::new(0),
                field_type: "TimelineData",
                rust_offset: offset_of!(TimelineEntityData, timeline_data),
            },
            FieldInfoData {
                name: "AutoPlay",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, auto_play),
            },
            FieldInfoData {
                name: "DeltaTimeClock",
                flags: MemberInfoFlags::new(0),
                field_type: "TimelineClock",
                rust_offset: offset_of!(TimelineEntityData, delta_time_clock),
            },
            FieldInfoData {
                name: "TimeDilationType",
                flags: MemberInfoFlags::new(0),
                field_type: "TimeDeltaType",
                rust_offset: offset_of!(TimelineEntityData, time_dilation_type),
            },
            FieldInfoData {
                name: "AutoInitConnectedProperties",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, auto_init_connected_properties),
            },
            FieldInfoData {
                name: "ResetTimeOnStarted",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, reset_time_on_started),
            },
            FieldInfoData {
                name: "AllowAnimationCarryForward",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, allow_animation_carry_forward),
            },
            FieldInfoData {
                name: "AlwaysEndOnPreFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, always_end_on_pre_frame),
            },
            FieldInfoData {
                name: "SyncTimelines",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, sync_timelines),
            },
            FieldInfoData {
                name: "RuntimeFramerate",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(TimelineEntityData, runtime_framerate),
            },
            FieldInfoData {
                name: "BlendInTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimelineEntityData, blend_in_time),
            },
            FieldInfoData {
                name: "BlendOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimelineEntityData, blend_out_time),
            },
            FieldInfoData {
                name: "Looping",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, looping),
            },
            FieldInfoData {
                name: "Infinite",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, infinite),
            },
            FieldInfoData {
                name: "InitTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimelineEntityData, init_time),
            },
            FieldInfoData {
                name: "StartTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimelineEntityData, start_time),
            },
            FieldInfoData {
                name: "EndTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimelineEntityData, end_time),
            },
            FieldInfoData {
                name: "PlaybackRate",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(TimelineEntityData, playback_rate),
            },
            FieldInfoData {
                name: "PrintCurrentTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(TimelineEntityData, print_current_time),
            },
            FieldInfoData {
                name: "UpdatePassFlags",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(TimelineEntityData, update_pass_flags),
            },
        ],
    }),
    array_type: Some(TIMELINEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEENTITYDATA_TYPE_INFO
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


pub static TIMELINEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineData {
    pub _glacier_base: TimelineTrackData,
    pub children: Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>,
}

pub trait TimelineDataTrait: TimelineTrackDataTrait {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>>;
}

impl TimelineDataTrait for TimelineData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataTrait>>>> {
        &mut self.children
    }
}

impl TimelineTrackDataTrait for TimelineData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for TimelineData {
}

impl super::core::DataBusPeerTrait for TimelineData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TimelineData {
}

impl super::core::DataContainerTrait for TimelineData {
}

pub static TIMELINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "TimelineTrackData-Array",
                rust_offset: offset_of!(TimelineData, children),
            },
        ],
    }),
    array_type: Some(TIMELINEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineData {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEDATA_TYPE_INFO
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


pub static TIMELINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TimelineEditorReinitializeState {
    #[default]
    TimelineEditorReinitializeState_ReadyToReinitialize = 0,
    TimelineEditorReinitializeState_BeginReceivingMetaProperties = 1,
    TimelineEditorReinitializeState_FinishedReceivingMetaProperties = 2,
}

pub static TIMELINEEDITORREINITIALIZESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEditorReinitializeState",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(TIMELINEEDITORREINITIALIZESTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimelineEditorReinitializeState {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEEDITORREINITIALIZESTATE_TYPE_INFO
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


pub static TIMELINEEDITORREINITIALIZESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEditorReinitializeState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineEditorReinitializeState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TimelineClock {
    #[default]
    Clock_SimTimeWithTimeDilation = 0,
    Clock_RawSimTime = 1,
    Clock_RealTime = 2,
}

pub static TIMELINECLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineClock",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(TIMELINECLOCK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimelineClock {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINECLOCK_TYPE_INFO
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


pub static TIMELINECLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineClock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineClock"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum TimelineFramerate {
    #[default]
    FPS_24 = 24,
    FPS_25 = 25,
    FPS_30 = 30,
    FPS_60 = 60,
    FPS_100 = 100,
    FPS_120 = 120,
}

pub static TIMELINEFRAMERATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineFramerate",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(TIMELINEFRAMERATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimelineFramerate {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEFRAMERATE_TYPE_INFO
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


pub static TIMELINEFRAMERATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineFramerate-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineFramerate"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FbxImportData {
    pub _glacier_base: TimelineExtraData,
}

pub trait FbxImportDataTrait: TimelineExtraDataTrait {
}

impl FbxImportDataTrait for FbxImportData {
}

impl TimelineExtraDataTrait for FbxImportData {
}

impl super::core::DataContainerTrait for FbxImportData {
}

pub static FBXIMPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbxImportData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEEXTRADATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FbxImportData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBXIMPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FbxImportData {
    fn type_info(&self) -> &'static TypeInfo {
        FBXIMPORTDATA_TYPE_INFO
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


pub static FBXIMPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbxImportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("FbxImportData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineBufferingHelper {
    pub _glacier_base: super::core::DataContainer,
}

pub trait TimelineBufferingHelperTrait: super::core::DataContainerTrait {
}

impl TimelineBufferingHelperTrait for TimelineBufferingHelper {
}

impl super::core::DataContainerTrait for TimelineBufferingHelper {
}

pub static TIMELINEBUFFERINGHELPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineBufferingHelper",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineBufferingHelper as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEBUFFERINGHELPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineBufferingHelper {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEBUFFERINGHELPER_TYPE_INFO
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


pub static TIMELINEBUFFERINGHELPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineBufferingHelper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineBufferingHelper"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec4TrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub x: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub y: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub z: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub w: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
}

pub trait Vec4TrackDataTrait: PropertyTrackBaseDataTrait {
    fn x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn w(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn w_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
}

impl Vec4TrackDataTrait for Vec4TrackData {
    fn x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.x
    }
    fn x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.x
    }
    fn y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.y
    }
    fn y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.y
    }
    fn z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.z
    }
    fn z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.z
    }
    fn w(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.w
    }
    fn w_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.w
    }
}

impl PropertyTrackBaseDataTrait for Vec4TrackData {
}

impl SchematicPinTrackDataTrait for Vec4TrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for Vec4TrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for Vec4TrackData {
}

impl super::core::DataBusPeerTrait for Vec4TrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for Vec4TrackData {
}

impl super::core::DataContainerTrait for Vec4TrackData {
}

pub static VEC4TRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4TrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec4TrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec4TrackData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec4TrackData, y),
            },
            FieldInfoData {
                name: "Z",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec4TrackData, z),
            },
            FieldInfoData {
                name: "W",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec4TrackData, w),
            },
        ],
    }),
    array_type: Some(VEC4TRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec4TrackData {
    fn type_info(&self) -> &'static TypeInfo {
        VEC4TRACKDATA_TYPE_INFO
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


pub static VEC4TRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4TrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec4TrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3TrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub x: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub y: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub z: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
}

pub trait Vec3TrackDataTrait: PropertyTrackBaseDataTrait {
    fn x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
}

impl Vec3TrackDataTrait for Vec3TrackData {
    fn x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.x
    }
    fn x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.x
    }
    fn y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.y
    }
    fn y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.y
    }
    fn z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.z
    }
    fn z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.z
    }
}

impl PropertyTrackBaseDataTrait for Vec3TrackData {
}

impl SchematicPinTrackDataTrait for Vec3TrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for Vec3TrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for Vec3TrackData {
}

impl super::core::DataBusPeerTrait for Vec3TrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for Vec3TrackData {
}

impl super::core::DataContainerTrait for Vec3TrackData {
}

pub static VEC3TRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3TrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3TrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec3TrackData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec3TrackData, y),
            },
            FieldInfoData {
                name: "Z",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec3TrackData, z),
            },
        ],
    }),
    array_type: Some(VEC3TRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3TrackData {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3TRACKDATA_TYPE_INFO
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


pub static VEC3TRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3TrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec3TrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec2TrackData {
    pub _glacier_base: PropertyTrackBaseData,
    pub x: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub y: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
}

pub trait Vec2TrackDataTrait: PropertyTrackBaseDataTrait {
    fn x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
}

impl Vec2TrackDataTrait for Vec2TrackData {
    fn x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.x
    }
    fn x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.x
    }
    fn y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.y
    }
    fn y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.y
    }
}

impl PropertyTrackBaseDataTrait for Vec2TrackData {
}

impl SchematicPinTrackDataTrait for Vec2TrackData {
    fn source_pin_id(&self) -> &i32 {
        self._glacier_base.source_pin_id()
    }
    fn source_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.source_pin_id_mut()
    }
    fn target_pin_id(&self) -> &i32 {
        self._glacier_base.target_pin_id()
    }
    fn target_pin_id_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_id_mut()
    }
    fn target_pin_name_hash(&self) -> &i32 {
        self._glacier_base.target_pin_name_hash()
    }
    fn target_pin_name_hash_mut(&mut self) -> &mut i32 {
        self._glacier_base.target_pin_name_hash_mut()
    }
}

impl TimelineTrackDataTrait for Vec2TrackData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for Vec2TrackData {
}

impl super::core::DataBusPeerTrait for Vec2TrackData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for Vec2TrackData {
}

impl super::core::DataContainerTrait for Vec2TrackData {
}

pub static VEC2TRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2TrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec2TrackData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec2TrackData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(Vec2TrackData, y),
            },
        ],
    }),
    array_type: Some(VEC2TRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2TrackData {
    fn type_info(&self) -> &'static TypeInfo {
        VEC2TRACKDATA_TYPE_INFO
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


pub static VEC2TRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2TrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec2TrackData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransformLayerData {
    pub _glacier_base: TimelineTrackData,
    pub weight: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub blendtype: LayeredTransformBlendType,
}

pub trait TransformLayerDataTrait: TimelineTrackDataTrait {
    fn weight(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn weight_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn blendtype(&self) -> &LayeredTransformBlendType;
    fn blendtype_mut(&mut self) -> &mut LayeredTransformBlendType;
}

impl TransformLayerDataTrait for TransformLayerData {
    fn weight(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.weight
    }
    fn weight_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.weight
    }
    fn blendtype(&self) -> &LayeredTransformBlendType {
        &self.blendtype
    }
    fn blendtype_mut(&mut self) -> &mut LayeredTransformBlendType {
        &mut self.blendtype
    }
}

impl TimelineTrackDataTrait for TransformLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for TransformLayerData {
}

impl super::core::DataBusPeerTrait for TransformLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TransformLayerData {
}

impl super::core::DataContainerTrait for TransformLayerData {
}

pub static TRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Weight",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(TransformLayerData, weight),
            },
            FieldInfoData {
                name: "Blendtype",
                flags: MemberInfoFlags::new(0),
                field_type: "LayeredTransform_BlendType",
                rust_offset: offset_of!(TransformLayerData, blendtype),
            },
        ],
    }),
    array_type: Some(TRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMLAYERDATA_TYPE_INFO
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


pub static TRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TransformLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LayeredTransformBlendType {
    #[default]
    LayeredTransform_BlendType_WorldOverride = 0,
    LayeredTransform_BlendType_WorldAdditive = 1,
    LayeredTransform_BlendType_WorldTranslationLocalRotationAdditive = 2,
    LayeredTransform_BlendType_LocalAdditive = 3,
    LayeredTransform_BlendType_Special = 4,
}

pub static LAYEREDTRANSFORM_BLENDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransform_BlendType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(LAYEREDTRANSFORM_BLENDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LayeredTransformBlendType {
    fn type_info(&self) -> &'static TypeInfo {
        LAYEREDTRANSFORM_BLENDTYPE_TYPE_INFO
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


pub static LAYEREDTRANSFORM_BLENDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransform_BlendType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LayeredTransform_BlendType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StartingLocationTransformLayerData {
    pub _glacier_base: TransformLayerData,
}

pub trait StartingLocationTransformLayerDataTrait: TransformLayerDataTrait {
}

impl StartingLocationTransformLayerDataTrait for StartingLocationTransformLayerData {
}

impl TransformLayerDataTrait for StartingLocationTransformLayerData {
    fn weight(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight()
    }
    fn weight_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight_mut()
    }
    fn blendtype(&self) -> &LayeredTransformBlendType {
        self._glacier_base.blendtype()
    }
    fn blendtype_mut(&mut self) -> &mut LayeredTransformBlendType {
        self._glacier_base.blendtype_mut()
    }
}

impl TimelineTrackDataTrait for StartingLocationTransformLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for StartingLocationTransformLayerData {
}

impl super::core::DataBusPeerTrait for StartingLocationTransformLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StartingLocationTransformLayerData {
}

impl super::core::DataContainerTrait for StartingLocationTransformLayerData {
}

pub static STARTINGLOCATIONTRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartingLocationTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StartingLocationTransformLayerData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STARTINGLOCATIONTRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StartingLocationTransformLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        STARTINGLOCATIONTRANSFORMLAYERDATA_TYPE_INFO
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


pub static STARTINGLOCATIONTRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartingLocationTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("StartingLocationTransformLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ScaleTransformLayerData {
    pub _glacier_base: TransformLayerData,
    pub is_uniform: bool,
    pub scale_x: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub scale_y: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub scale_z: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
}

pub trait ScaleTransformLayerDataTrait: TransformLayerDataTrait {
    fn is_uniform(&self) -> &bool;
    fn is_uniform_mut(&mut self) -> &mut bool;
    fn scale_x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn scale_x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn scale_y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn scale_y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn scale_z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn scale_z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
}

impl ScaleTransformLayerDataTrait for ScaleTransformLayerData {
    fn is_uniform(&self) -> &bool {
        &self.is_uniform
    }
    fn is_uniform_mut(&mut self) -> &mut bool {
        &mut self.is_uniform
    }
    fn scale_x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.scale_x
    }
    fn scale_x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.scale_x
    }
    fn scale_y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.scale_y
    }
    fn scale_y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.scale_y
    }
    fn scale_z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.scale_z
    }
    fn scale_z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.scale_z
    }
}

impl TransformLayerDataTrait for ScaleTransformLayerData {
    fn weight(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight()
    }
    fn weight_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight_mut()
    }
    fn blendtype(&self) -> &LayeredTransformBlendType {
        self._glacier_base.blendtype()
    }
    fn blendtype_mut(&mut self) -> &mut LayeredTransformBlendType {
        self._glacier_base.blendtype_mut()
    }
}

impl TimelineTrackDataTrait for ScaleTransformLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for ScaleTransformLayerData {
}

impl super::core::DataBusPeerTrait for ScaleTransformLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ScaleTransformLayerData {
}

impl super::core::DataContainerTrait for ScaleTransformLayerData {
}

pub static SCALETRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ScaleTransformLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IsUniform",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ScaleTransformLayerData, is_uniform),
            },
            FieldInfoData {
                name: "ScaleX",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(ScaleTransformLayerData, scale_x),
            },
            FieldInfoData {
                name: "ScaleY",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(ScaleTransformLayerData, scale_y),
            },
            FieldInfoData {
                name: "ScaleZ",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(ScaleTransformLayerData, scale_z),
            },
        ],
    }),
    array_type: Some(SCALETRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScaleTransformLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        SCALETRANSFORMLAYERDATA_TYPE_INFO
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


pub static SCALETRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ScaleTransformLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct KeyedTransformLayerData {
    pub _glacier_base: TransformLayerData,
    pub force_minimum_rotation_path_between_keys: bool,
    pub pre_infinity_for_min_rotation: InfinityType,
    pub post_infinity_for_min_rotation: InfinityType,
    pub translation_x: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub translation_y: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub translation_z: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub rotation_x: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub rotation_y: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub rotation_z: Option<Arc<Mutex<dyn FloatTrackDataTrait>>>,
    pub quat_keyframes: Vec<QuatKeyframe>,
}

pub trait KeyedTransformLayerDataTrait: TransformLayerDataTrait {
    fn force_minimum_rotation_path_between_keys(&self) -> &bool;
    fn force_minimum_rotation_path_between_keys_mut(&mut self) -> &mut bool;
    fn pre_infinity_for_min_rotation(&self) -> &InfinityType;
    fn pre_infinity_for_min_rotation_mut(&mut self) -> &mut InfinityType;
    fn post_infinity_for_min_rotation(&self) -> &InfinityType;
    fn post_infinity_for_min_rotation_mut(&mut self) -> &mut InfinityType;
    fn translation_x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn translation_x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn translation_y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn translation_y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn translation_z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn translation_z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn rotation_x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn rotation_x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn rotation_y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn rotation_y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn rotation_z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn rotation_z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>>;
    fn quat_keyframes(&self) -> &Vec<QuatKeyframe>;
    fn quat_keyframes_mut(&mut self) -> &mut Vec<QuatKeyframe>;
}

impl KeyedTransformLayerDataTrait for KeyedTransformLayerData {
    fn force_minimum_rotation_path_between_keys(&self) -> &bool {
        &self.force_minimum_rotation_path_between_keys
    }
    fn force_minimum_rotation_path_between_keys_mut(&mut self) -> &mut bool {
        &mut self.force_minimum_rotation_path_between_keys
    }
    fn pre_infinity_for_min_rotation(&self) -> &InfinityType {
        &self.pre_infinity_for_min_rotation
    }
    fn pre_infinity_for_min_rotation_mut(&mut self) -> &mut InfinityType {
        &mut self.pre_infinity_for_min_rotation
    }
    fn post_infinity_for_min_rotation(&self) -> &InfinityType {
        &self.post_infinity_for_min_rotation
    }
    fn post_infinity_for_min_rotation_mut(&mut self) -> &mut InfinityType {
        &mut self.post_infinity_for_min_rotation
    }
    fn translation_x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.translation_x
    }
    fn translation_x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.translation_x
    }
    fn translation_y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.translation_y
    }
    fn translation_y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.translation_y
    }
    fn translation_z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.translation_z
    }
    fn translation_z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.translation_z
    }
    fn rotation_x(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.rotation_x
    }
    fn rotation_x_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.rotation_x
    }
    fn rotation_y(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.rotation_y
    }
    fn rotation_y_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.rotation_y
    }
    fn rotation_z(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &self.rotation_z
    }
    fn rotation_z_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        &mut self.rotation_z
    }
    fn quat_keyframes(&self) -> &Vec<QuatKeyframe> {
        &self.quat_keyframes
    }
    fn quat_keyframes_mut(&mut self) -> &mut Vec<QuatKeyframe> {
        &mut self.quat_keyframes
    }
}

impl TransformLayerDataTrait for KeyedTransformLayerData {
    fn weight(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight()
    }
    fn weight_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight_mut()
    }
    fn blendtype(&self) -> &LayeredTransformBlendType {
        self._glacier_base.blendtype()
    }
    fn blendtype_mut(&mut self) -> &mut LayeredTransformBlendType {
        self._glacier_base.blendtype_mut()
    }
}

impl TimelineTrackDataTrait for KeyedTransformLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for KeyedTransformLayerData {
}

impl super::core::DataBusPeerTrait for KeyedTransformLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for KeyedTransformLayerData {
}

impl super::core::DataContainerTrait for KeyedTransformLayerData {
}

pub static KEYEDTRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "KeyedTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<KeyedTransformLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ForceMinimumRotationPathBetweenKeys",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(KeyedTransformLayerData, force_minimum_rotation_path_between_keys),
            },
            FieldInfoData {
                name: "PreInfinityForMinRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "InfinityType",
                rust_offset: offset_of!(KeyedTransformLayerData, pre_infinity_for_min_rotation),
            },
            FieldInfoData {
                name: "PostInfinityForMinRotation",
                flags: MemberInfoFlags::new(0),
                field_type: "InfinityType",
                rust_offset: offset_of!(KeyedTransformLayerData, post_infinity_for_min_rotation),
            },
            FieldInfoData {
                name: "TranslationX",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(KeyedTransformLayerData, translation_x),
            },
            FieldInfoData {
                name: "TranslationY",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(KeyedTransformLayerData, translation_y),
            },
            FieldInfoData {
                name: "TranslationZ",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(KeyedTransformLayerData, translation_z),
            },
            FieldInfoData {
                name: "RotationX",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(KeyedTransformLayerData, rotation_x),
            },
            FieldInfoData {
                name: "RotationY",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(KeyedTransformLayerData, rotation_y),
            },
            FieldInfoData {
                name: "RotationZ",
                flags: MemberInfoFlags::new(0),
                field_type: "FloatTrackData",
                rust_offset: offset_of!(KeyedTransformLayerData, rotation_z),
            },
            FieldInfoData {
                name: "QuatKeyframes",
                flags: MemberInfoFlags::new(144),
                field_type: "QuatKeyframe-Array",
                rust_offset: offset_of!(KeyedTransformLayerData, quat_keyframes),
            },
        ],
    }),
    array_type: Some(KEYEDTRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for KeyedTransformLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        KEYEDTRANSFORMLAYERDATA_TYPE_INFO
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


pub static KEYEDTRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "KeyedTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("KeyedTransformLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuatKeyframe {
    pub time: f32,
    pub quat_value: super::core::Vec4,
}

pub trait QuatKeyframeTrait: TypeObject {
    fn time(&self) -> &f32;
    fn time_mut(&mut self) -> &mut f32;
    fn quat_value(&self) -> &super::core::Vec4;
    fn quat_value_mut(&mut self) -> &mut super::core::Vec4;
}

impl QuatKeyframeTrait for QuatKeyframe {
    fn time(&self) -> &f32 {
        &self.time
    }
    fn time_mut(&mut self) -> &mut f32 {
        &mut self.time
    }
    fn quat_value(&self) -> &super::core::Vec4 {
        &self.quat_value
    }
    fn quat_value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.quat_value
    }
}

pub static QUATKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuatKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuatKeyframe as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(QuatKeyframe, time),
            },
            FieldInfoData {
                name: "QuatValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(QuatKeyframe, quat_value),
            },
        ],
    }),
    array_type: Some(QUATKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for QuatKeyframe {
    fn type_info(&self) -> &'static TypeInfo {
        QUATKEYFRAME_TYPE_INFO
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


pub static QUATKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuatKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("QuatKeyframe"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct GroupTransformLayerData {
    pub _glacier_base: TransformLayerData,
    pub children: Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>>,
}

pub trait GroupTransformLayerDataTrait: TransformLayerDataTrait {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>>;
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>>;
}

impl GroupTransformLayerDataTrait for GroupTransformLayerData {
    fn children(&self) -> &Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TransformLayerDataTrait>>>> {
        &mut self.children
    }
}

impl TransformLayerDataTrait for GroupTransformLayerData {
    fn weight(&self) -> &Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight()
    }
    fn weight_mut(&mut self) -> &mut Option<Arc<Mutex<dyn FloatTrackDataTrait>>> {
        self._glacier_base.weight_mut()
    }
    fn blendtype(&self) -> &LayeredTransformBlendType {
        self._glacier_base.blendtype()
    }
    fn blendtype_mut(&mut self) -> &mut LayeredTransformBlendType {
        self._glacier_base.blendtype_mut()
    }
}

impl TimelineTrackDataTrait for GroupTransformLayerData {
    fn expose_pins(&self) -> &bool {
        self._glacier_base.expose_pins()
    }
    fn expose_pins_mut(&mut self) -> &mut bool {
        self._glacier_base.expose_pins_mut()
    }
    fn is_disabled(&self) -> &bool {
        self._glacier_base.is_disabled()
    }
    fn is_disabled_mut(&mut self) -> &mut bool {
        self._glacier_base.is_disabled_mut()
    }
    fn conditions(&self) -> &Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions()
    }
    fn conditions_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn TimelineTrackDataConditionsBaseTrait>>>> {
        self._glacier_base.conditions_mut()
    }
    fn update_pass_flags(&self) -> &u16 {
        self._glacier_base.update_pass_flags()
    }
    fn update_pass_flags_mut(&mut self) -> &mut u16 {
        self._glacier_base.update_pass_flags_mut()
    }
    fn bucket_pre_children_order(&self) -> &u16 {
        self._glacier_base.bucket_pre_children_order()
    }
    fn bucket_pre_children_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_pre_children_order_mut()
    }
    fn bucket_order(&self) -> &u16 {
        self._glacier_base.bucket_order()
    }
    fn bucket_order_mut(&mut self) -> &mut u16 {
        self._glacier_base.bucket_order_mut()
    }
}

impl super::entity::GameObjectDataTrait for GroupTransformLayerData {
}

impl super::core::DataBusPeerTrait for GroupTransformLayerData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for GroupTransformLayerData {
}

impl super::core::DataContainerTrait for GroupTransformLayerData {
}

pub static GROUPTRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<GroupTransformLayerData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: "TransformLayerData-Array",
                rust_offset: offset_of!(GroupTransformLayerData, children),
            },
        ],
    }),
    array_type: Some(GROUPTRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupTransformLayerData {
    fn type_info(&self) -> &'static TypeInfo {
        GROUPTRANSFORMLAYERDATA_TYPE_INFO
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


pub static GROUPTRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTransformLayerData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineTrack {
}

pub trait TimelineTrackTrait: TypeObject {
}

impl TimelineTrackTrait for TimelineTrack {
}

pub static TIMELINETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TimelineTrack {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINETRACK_TYPE_INFO
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


pub static TIMELINETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineRootTrack {
    pub _glacier_base: TimelineTrack,
}

pub trait TimelineRootTrackTrait: TimelineTrackTrait {
}

impl TimelineRootTrackTrait for TimelineRootTrack {
}

impl TimelineTrackTrait for TimelineRootTrack {
}

pub static TIMELINEROOTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineRootTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineRootTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEROOTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TimelineRootTrack {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEROOTTRACK_TYPE_INFO
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


pub static TIMELINEROOTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineRootTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineRootTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TimelineEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TimelineEntityTrait: super::entity::EntityTrait {
}

impl TimelineEntityTrait for TimelineEntity {
}

impl super::entity::EntityTrait for TimelineEntity {
}

impl super::entity::EntityBusPeerTrait for TimelineEntity {
}

pub static TIMELINEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntity",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TimelineEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TimelineEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TIMELINEENTITY_TYPE_INFO
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


pub static TIMELINEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineEntity"),
    array_type: None,
    alignment: 8,
};


