use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4Track {
}

pub const VEC4TRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4Track",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC4TRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4Track {
    fn type_info() -> &'static TypeInfo {
        VEC4TRACK_TYPE_INFO
    }
}


pub const VEC4TRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4Track-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec4Track-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3Track {
}

pub const VEC3TRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Track",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC3TRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3Track {
    fn type_info() -> &'static TypeInfo {
        VEC3TRACK_TYPE_INFO
    }
}


pub const VEC3TRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Track-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec3Track-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicEventTrack {
}

pub const SCHEMATICEVENTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicEventTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SCHEMATICEVENTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicEventTrack {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICEVENTTRACK_TYPE_INFO
    }
}


pub const SCHEMATICEVENTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicEventTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SchematicEventTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RecordTrackBase {
}

pub const RECORDTRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RECORDTRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RecordTrackBase {
    fn type_info() -> &'static TypeInfo {
        RECORDTRACKBASE_TYPE_INFO
    }
}


pub const RECORDTRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RecordTrackBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BoolTrack {
}

pub const BOOLTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BOOLTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BoolTrack {
    fn type_info() -> &'static TypeInfo {
        BOOLTRACK_TYPE_INFO
    }
}


pub const BOOLTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BoolTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BookmarkTrack {
}

pub const BOOKMARKTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(BOOKMARKTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BookmarkTrack {
    fn type_info() -> &'static TypeInfo {
        BOOKMARKTRACK_TYPE_INFO
    }
}


pub const BOOKMARKTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BookmarkTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformLayer {
}

pub const TRANSFORMLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayer",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMLAYER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformLayer {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMLAYER_TYPE_INFO
    }
}


pub const TRANSFORMLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TransformLayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SimpleTransformLayer {
}

pub const SIMPLETRANSFORMLAYER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleTransformLayer",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SIMPLETRANSFORMLAYER_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SimpleTransformLayer {
    fn type_info() -> &'static TypeInfo {
        SIMPLETRANSFORMLAYER_TYPE_INFO
    }
}


pub const SIMPLETRANSFORMLAYER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SimpleTransformLayer-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SimpleTransformLayer-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MasterTimelineTrack {
}

pub const MASTERTIMELINETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MASTERTIMELINETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MasterTimelineTrack {
    fn type_info() -> &'static TypeInfo {
        MASTERTIMELINETRACK_TYPE_INFO
    }
}


pub const MASTERTIMELINETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("MasterTimelineTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkTrack {
}

pub const LINKTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINKTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinkTrack {
    fn type_info() -> &'static TypeInfo {
        LINKTRACK_TYPE_INFO
    }
}


pub const LINKTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkedMasterTimelineTrack {
}

pub const LINKEDMASTERTIMELINETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINKEDMASTERTIMELINETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinkedMasterTimelineTrack {
    fn type_info() -> &'static TypeInfo {
        LINKEDMASTERTIMELINETRACK_TYPE_INFO
    }
}


pub const LINKEDMASTERTIMELINETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedMasterTimelineTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LayeredTransformTrack {
}

pub const LAYEREDTRANSFORMTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LAYEREDTRANSFORMTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LayeredTransformTrack {
    fn type_info() -> &'static TypeInfo {
        LAYEREDTRANSFORMTRACK_TYPE_INFO
    }
}


pub const LAYEREDTRANSFORMTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LayeredTransformTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IPropertyTrack {
}

pub const IPROPERTYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPropertyTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IPROPERTYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IPropertyTrack {
    fn type_info() -> &'static TypeInfo {
        IPROPERTYTRACK_TYPE_INFO
    }
}


pub const IPROPERTYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IPropertyTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IPropertyTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntTrack {
}

pub const INTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntTrack {
    fn type_info() -> &'static TypeInfo {
        INTTRACK_TYPE_INFO
    }
}


pub const INTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IntTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupTrack {
}

pub const GROUPTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(GROUPTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GroupTrack {
    fn type_info() -> &'static TypeInfo {
        GROUPTRACK_TYPE_INFO
    }
}


pub const GROUPTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatTrack {
}

pub const FLOATTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IPROPERTYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatTrack {
    fn type_info() -> &'static TypeInfo {
        FLOATTRACK_TYPE_INFO
    }
}


pub const FLOATTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("FloatTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalTimeTrack {
}

pub const EXTERNALTIMETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(EXTERNALTIMETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ExternalTimeTrack {
    fn type_info() -> &'static TypeInfo {
        EXTERNALTIMETRACK_TYPE_INFO
    }
}


pub const EXTERNALTIMETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ExternalTimeTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTrack {
}

pub const ENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityTrack {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRACK_TYPE_INFO
    }
}


pub const ENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkedProxyEntityTrack {
}

pub const LINKEDPROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEMPLATEDPROXYENTITYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINKEDPROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LinkedProxyEntityTrack {
    fn type_info() -> &'static TypeInfo {
        LINKEDPROXYENTITYTRACK_TYPE_INFO
    }
}


pub const LINKEDPROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedProxyEntityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplatedProxyEntityTrack {
}

pub const TEMPLATEDPROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROXYENTITYTRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEMPLATEDPROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TemplatedProxyEntityTrack {
    fn type_info() -> &'static TypeInfo {
        TEMPLATEDPROXYENTITYTRACK_TYPE_INFO
    }
}


pub const TEMPLATEDPROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TemplatedProxyEntityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyEntityTrack {
}

pub const PROXYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROXYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ProxyEntityTrack {
    fn type_info() -> &'static TypeInfo {
        PROXYENTITYTRACK_TYPE_INFO
    }
}


pub const PROXYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ProxyEntityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DummyEntityTrack {
}

pub const DUMMYENTITYTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DUMMYENTITYTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DummyEntityTrack {
    fn type_info() -> &'static TypeInfo {
        DUMMYENTITYTRACK_TYPE_INFO
    }
}


pub const DUMMYENTITYTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("DummyEntityTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTrackBase {
}

pub const ENTITYTRACKBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBase",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ENTITYTRACKBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for EntityTrackBase {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRACKBASE_TYPE_INFO
    }
}


pub const ENTITYTRACKBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchematicPinTrackData {
    pub source_pin_id: i32,
    pub target_pin_id: i32,
    pub target_pin_name_hash: i32,
}

pub const SCHEMATICPINTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicPinTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourcePinId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SchematicPinTrackData, source_pin_id),
            },
            FieldInfoData {
                name: "TargetPinId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SchematicPinTrackData, target_pin_id),
            },
            FieldInfoData {
                name: "TargetPinNameHash",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(SchematicPinTrackData, target_pin_name_hash),
            },
        ],
    }),
    array_type: Some(SCHEMATICPINTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SchematicPinTrackData {
    fn type_info() -> &'static TypeInfo {
        SCHEMATICPINTRACKDATA_TYPE_INFO
    }
}


pub const SCHEMATICPINTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SchematicPinTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SchematicPinTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RecordTrackChildrenData {
}

pub const RECORDTRACKCHILDRENDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackChildrenData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(RECORDTRACKBASEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(RECORDTRACKCHILDRENDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RecordTrackChildrenData {
    fn type_info() -> &'static TypeInfo {
        RECORDTRACKCHILDRENDATA_TYPE_INFO
    }
}


pub const RECORDTRACKCHILDRENDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackChildrenData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RecordTrackChildrenData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct RecordTrackBaseData {
    pub frames_to_skip_per_key: i32,
}

pub const RECORDTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "FramesToSkipPerKey",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(RecordTrackBaseData, frames_to_skip_per_key),
            },
        ],
    }),
    array_type: Some(RECORDTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for RecordTrackBaseData {
    fn type_info() -> &'static TypeInfo {
        RECORDTRACKBASEDATA_TYPE_INFO
    }
}


pub const RECORDTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RecordTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RecordTrackBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyTrackBaseData {
}

pub const PROPERTYTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROPERTYTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PropertyTrackBaseData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYTRACKBASEDATA_TYPE_INFO
    }
}


pub const PROPERTYTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("PropertyTrackBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PropertyReaderTrackBaseData {
    pub realm: super::core::Realm,
}

pub const PROPERTYREADERTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyReaderTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(PropertyReaderTrackBaseData, realm),
            },
        ],
    }),
    array_type: Some(PROPERTYREADERTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyReaderTrackBaseData {
    fn type_info() -> &'static TypeInfo {
        PROPERTYREADERTRACKBASEDATA_TYPE_INFO
    }
}


pub const PROPERTYREADERTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyReaderTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("PropertyReaderTrackBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct MasterTimelineTrackData {
    pub keyframes: Vec<SlaveTimelineKeyframeData>,
    pub children: Vec<TimelineTrackData>,
}

pub const MASTERTIMELINETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: SLAVETIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MasterTimelineTrackData, keyframes),
            },
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: TIMELINETRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(MasterTimelineTrackData, children),
            },
        ],
    }),
    array_type: Some(MASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MasterTimelineTrackData {
    fn type_info() -> &'static TypeInfo {
        MASTERTIMELINETRACKDATA_TYPE_INFO
    }
}


pub const MASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MasterTimelineTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("MasterTimelineTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct SlaveTimelineKeyframeData {
    pub time: f32,
    pub length: f32,
    pub guide_time: f32,
    pub timeline_data: TimelineEntityData,
    pub slave_guid_chain: Vec<super::core::Guid>,
}

pub const SLAVETIMELINEKEYFRAMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlaveTimelineKeyframeData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEKEYFRAMEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlaveTimelineKeyframeData, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlaveTimelineKeyframeData, length),
            },
            FieldInfoData {
                name: "GuideTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(SlaveTimelineKeyframeData, guide_time),
            },
            FieldInfoData {
                name: "TimelineData",
                flags: MemberInfoFlags::new(0),
                field_type: TIMELINEENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(SlaveTimelineKeyframeData, timeline_data),
            },
            FieldInfoData {
                name: "SlaveGuidChain",
                flags: MemberInfoFlags::new(144),
                field_type: GUID_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(SlaveTimelineKeyframeData, slave_guid_chain),
            },
        ],
    }),
    array_type: Some(SLAVETIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SlaveTimelineKeyframeData {
    fn type_info() -> &'static TypeInfo {
        SLAVETIMELINEKEYFRAMEDATA_TYPE_INFO
    }
}


pub const SLAVETIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SlaveTimelineKeyframeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("SlaveTimelineKeyframeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkTrackData {
}

pub const LINKTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LINKTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkTrackData {
    fn type_info() -> &'static TypeInfo {
        LINKTRACKDATA_TYPE_INFO
    }
}


pub const LINKTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinkedMasterTimelineTrackData {
    pub linked_timeline_data: TimelineEntityData,
    pub linked_timeline_time_offset: f32,
    pub keyframes: Vec<LinkedMasterTimelineKeyframe>,
}

pub const LINKEDMASTERTIMELINETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LINKTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LinkedTimelineData",
                flags: MemberInfoFlags::new(0),
                field_type: TIMELINEENTITYDATA_TYPE_INFO,
                rust_offset: offset_of!(LinkedMasterTimelineTrackData, linked_timeline_data),
            },
            FieldInfoData {
                name: "LinkedTimelineTimeOffset",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkedMasterTimelineTrackData, linked_timeline_time_offset),
            },
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: LINKEDMASTERTIMELINEKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LinkedMasterTimelineTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(LINKEDMASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkedMasterTimelineTrackData {
    fn type_info() -> &'static TypeInfo {
        LINKEDMASTERTIMELINETRACKDATA_TYPE_INFO
    }
}


pub const LINKEDMASTERTIMELINETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedMasterTimelineTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LinkedMasterTimelineKeyframe {
    pub time: f32,
    pub length: f32,
}

pub const LINKEDMASTERTIMELINEKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineKeyframe",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkedMasterTimelineKeyframe, time),
            },
            FieldInfoData {
                name: "Length",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(LinkedMasterTimelineKeyframe, length),
            },
        ],
    }),
    array_type: Some(LINKEDMASTERTIMELINEKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkedMasterTimelineKeyframe {
    fn type_info() -> &'static TypeInfo {
        LINKEDMASTERTIMELINEKEYFRAME_TYPE_INFO
    }
}


pub const LINKEDMASTERTIMELINEKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedMasterTimelineKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedMasterTimelineKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct LayeredTransformTrackData {
    pub layer_tracks: Vec<TransformLayerData>,
    pub use_timeline_space: bool,
    pub transform_space_enabled: bool,
}

pub const LAYEREDTRANSFORMTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LayerTracks",
                flags: MemberInfoFlags::new(144),
                field_type: TRANSFORMLAYERDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LayeredTransformTrackData, layer_tracks),
            },
            FieldInfoData {
                name: "UseTimelineSpace",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LayeredTransformTrackData, use_timeline_space),
            },
            FieldInfoData {
                name: "TransformSpaceEnabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LayeredTransformTrackData, transform_space_enabled),
            },
        ],
    }),
    array_type: Some(LAYEREDTRANSFORMTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LayeredTransformTrackData {
    fn type_info() -> &'static TypeInfo {
        LAYEREDTRANSFORMTRACKDATA_TYPE_INFO
    }
}


pub const LAYEREDTRANSFORMTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransformTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LayeredTransformTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IntTrackData {
    pub keyframes: Vec<IntKeyframe>,
}

pub const INTTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: INTKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(IntTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(INTTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntTrackData {
    fn type_info() -> &'static TypeInfo {
        INTTRACKDATA_TYPE_INFO
    }
}


pub const INTTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IntTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct IntKeyframe {
    pub time: f32,
    pub value: i32,
}

pub const INTKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(IntKeyframe, time),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(IntKeyframe, value),
            },
        ],
    }),
    array_type: Some(INTKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for IntKeyframe {
    fn type_info() -> &'static TypeInfo {
        INTKEYFRAME_TYPE_INFO
    }
}


pub const INTKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("IntKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupTrackRootData {
    pub children: Vec<TimelineTrackData>,
}

pub const GROUPTRACKROOTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackRootData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: TIMELINETRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GroupTrackRootData, children),
            },
        ],
    }),
    array_type: Some(GROUPTRACKROOTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupTrackRootData {
    fn type_info() -> &'static TypeInfo {
        GROUPTRACKROOTDATA_TYPE_INFO
    }
}


pub const GROUPTRACKROOTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackRootData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTrackRootData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupTrackData {
    pub root_data: GroupTrackRootData,
}

pub const GROUPTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "RootData",
                flags: MemberInfoFlags::new(0),
                field_type: GROUPTRACKROOTDATA_TYPE_INFO,
                rust_offset: offset_of!(GroupTrackData, root_data),
            },
        ],
    }),
    array_type: Some(GROUPTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupTrackData {
    fn type_info() -> &'static TypeInfo {
        GROUPTRACKDATA_TYPE_INFO
    }
}


pub const GROUPTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct FloatTrackData {
    pub curve_data: CurveData,
}

pub const FLOATTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CurveData",
                flags: MemberInfoFlags::new(0),
                field_type: CURVEDATA_TYPE_INFO,
                rust_offset: offset_of!(FloatTrackData, curve_data),
            },
        ],
    }),
    array_type: Some(FLOATTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatTrackData {
    fn type_info() -> &'static TypeInfo {
        FLOATTRACKDATA_TYPE_INFO
    }
}


pub const FLOATTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("FloatTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExternalTimeTrackData {
    pub external_time_priority: i32,
    pub external_time_offset_type: OffsetType,
    pub init_time_offset_type: OffsetType,
    pub current_time_offset_type: OffsetType,
}

pub const EXTERNALTIMETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExternalTimePriority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(ExternalTimeTrackData, external_time_priority),
            },
            FieldInfoData {
                name: "ExternalTimeOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: OFFSETTYPE_TYPE_INFO,
                rust_offset: offset_of!(ExternalTimeTrackData, external_time_offset_type),
            },
            FieldInfoData {
                name: "InitTimeOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: OFFSETTYPE_TYPE_INFO,
                rust_offset: offset_of!(ExternalTimeTrackData, init_time_offset_type),
            },
            FieldInfoData {
                name: "CurrentTimeOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: OFFSETTYPE_TYPE_INFO,
                rust_offset: offset_of!(ExternalTimeTrackData, current_time_offset_type),
            },
        ],
    }),
    array_type: Some(EXTERNALTIMETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ExternalTimeTrackData {
    fn type_info() -> &'static TypeInfo {
        EXTERNALTIMETRACKDATA_TYPE_INFO
    }
}


pub const EXTERNALTIMETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExternalTimeTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ExternalTimeTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum OffsetType {
    #[default]
    OffsetType_AbsoluteTime = 0,
    OffsetType_Relative = 1,
    OffsetType_AsPercent = 2,
}

pub const OFFSETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(OFFSETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for OffsetType {
    fn type_info() -> &'static TypeInfo {
        OFFSETTYPE_TYPE_INFO
    }
}


pub const OFFSETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "OffsetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("OffsetType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EventTrackData {
    pub keyframes: Vec<EventKeyframe>,
    pub fire_events_upon_skip: bool,
    pub update_properties_at_events: bool,
    pub anti_event_id: i32,
}

pub const EVENTTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SCHEMATICPINTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: EVENTKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EventTrackData, keyframes),
            },
            FieldInfoData {
                name: "FireEventsUponSkip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventTrackData, fire_events_upon_skip),
            },
            FieldInfoData {
                name: "UpdatePropertiesAtEvents",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EventTrackData, update_properties_at_events),
            },
            FieldInfoData {
                name: "AntiEventId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(EventTrackData, anti_event_id),
            },
        ],
    }),
    array_type: Some(EVENTTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EventTrackData {
    fn type_info() -> &'static TypeInfo {
        EVENTTRACKDATA_TYPE_INFO
    }
}


pub const EVENTTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EventTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct EventKeyframe {
    pub time: f32,
}

pub const EVENTKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(EventKeyframe, time),
            },
        ],
    }),
    array_type: Some(EVENTKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for EventKeyframe {
    fn type_info() -> &'static TypeInfo {
        EVENTKEYFRAME_TYPE_INFO
    }
}


pub const EVENTKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EventKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EventKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTrackData {
    pub guid_chain: Vec<super::core::Guid>,
}

pub const ENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GuidChain",
                flags: MemberInfoFlags::new(144),
                field_type: GUID_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntityTrackData, guid_chain),
            },
        ],
    }),
    array_type: Some(ENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityTrackData {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRACKDATA_TYPE_INFO
    }
}


pub const ENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LinkedProxyEntityTrackData {
    pub source_pin_id: i32,
}

pub const LINKEDPROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "SourcePinId",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LinkedProxyEntityTrackData, source_pin_id),
            },
        ],
    }),
    array_type: Some(LINKEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LinkedProxyEntityTrackData {
    fn type_info() -> &'static TypeInfo {
        LINKEDPROXYENTITYTRACKDATA_TYPE_INFO
    }
}


pub const LINKEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LinkedProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LinkedProxyEntityTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TemplatedProxyEntityTrackData {
}

pub const TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROXYENTITYTRACKDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEMPLATEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TemplatedProxyEntityTrackData {
    fn type_info() -> &'static TypeInfo {
        TEMPLATEDPROXYENTITYTRACKDATA_TYPE_INFO
    }
}


pub const TEMPLATEDPROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TemplatedProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TemplatedProxyEntityTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProxyEntityTrackData {
}

pub const PROXYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(PROXYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ProxyEntityTrackData {
    fn type_info() -> &'static TypeInfo {
        PROXYENTITYTRACKDATA_TYPE_INFO
    }
}


pub const PROXYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ProxyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ProxyEntityTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DummyEntityTrackData {
}

pub const DUMMYENTITYTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYTRACKBASEDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DUMMYENTITYTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DummyEntityTrackData {
    fn type_info() -> &'static TypeInfo {
        DUMMYENTITYTRACKDATA_TYPE_INFO
    }
}


pub const DUMMYENTITYTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DummyEntityTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("DummyEntityTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct EntityTrackBaseData {
    pub children: Vec<TimelineTrackData>,
    pub inherited_to_child_conversation_lines: bool,
    pub required: bool,
    pub handle_deleted_entity: bool,
    pub entity_sharing_policy: EntityTrackSharingPolicy,
}

pub const ENTITYTRACKBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBaseData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: TIMELINETRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(EntityTrackBaseData, children),
            },
            FieldInfoData {
                name: "InheritedToChildConversationLines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntityTrackBaseData, inherited_to_child_conversation_lines),
            },
            FieldInfoData {
                name: "Required",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntityTrackBaseData, required),
            },
            FieldInfoData {
                name: "HandleDeletedEntity",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(EntityTrackBaseData, handle_deleted_entity),
            },
            FieldInfoData {
                name: "EntitySharingPolicy",
                flags: MemberInfoFlags::new(0),
                field_type: ENTITYTRACKSHARINGPOLICY_TYPE_INFO,
                rust_offset: offset_of!(EntityTrackBaseData, entity_sharing_policy),
            },
        ],
    }),
    array_type: Some(ENTITYTRACKBASEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for EntityTrackBaseData {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRACKBASEDATA_TYPE_INFO
    }
}


pub const ENTITYTRACKBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackBaseData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum EntityTrackSharingPolicy {
    #[default]
    EntityTrackSharingPolicy_FirstWins = 0,
    EntityTrackSharingPolicy_Combine = 1,
}

pub const ENTITYTRACKSHARINGPOLICY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackSharingPolicy",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(ENTITYTRACKSHARINGPOLICY_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for EntityTrackSharingPolicy {
    fn type_info() -> &'static TypeInfo {
        ENTITYTRACKSHARINGPOLICY_TYPE_INFO
    }
}


pub const ENTITYTRACKSHARINGPOLICY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "EntityTrackSharingPolicy-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("EntityTrackSharingPolicy-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct CurveData {
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

pub const CURVEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PreInfinity",
                flags: MemberInfoFlags::new(0),
                field_type: INFINITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(CurveData, pre_infinity),
            },
            FieldInfoData {
                name: "PostInfinity",
                flags: MemberInfoFlags::new(0),
                field_type: INFINITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(CurveData, post_infinity),
            },
            FieldInfoData {
                name: "IsWeighted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(CurveData, is_weighted),
            },
            FieldInfoData {
                name: "CurveType",
                flags: MemberInfoFlags::new(0),
                field_type: CURVETYPE_TYPE_INFO,
                rust_offset: offset_of!(CurveData, curve_type),
            },
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CurveData, time),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CurveData, value),
            },
            FieldInfoData {
                name: "InTanX",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CurveData, in_tan_x),
            },
            FieldInfoData {
                name: "InTanY",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CurveData, in_tan_y),
            },
            FieldInfoData {
                name: "OutTanX",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CurveData, out_tan_x),
            },
            FieldInfoData {
                name: "OutTanY",
                flags: MemberInfoFlags::new(144),
                field_type: FLOAT32_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(CurveData, out_tan_y),
            },
        ],
    }),
    array_type: Some(CURVEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CurveData {
    fn type_info() -> &'static TypeInfo {
        CURVEDATA_TYPE_INFO
    }
}


pub const CURVEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("CurveData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
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

pub const CURVETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(CURVETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for CurveType {
    fn type_info() -> &'static TypeInfo {
        CURVETYPE_TYPE_INFO
    }
}


pub const CURVETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CurveType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("CurveType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum InfinityType {
    #[default]
    InfinityType_Constant = 0,
    InfinityType_Linear = 1,
    InfinityType_Cycle = 2,
    InfinityType_CycleWithOffset = 3,
    InfinityType_Oscillate = 4,
    InfinityType_None = 5,
}

pub const INFINITYTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InfinityType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(INFINITYTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InfinityType {
    fn type_info() -> &'static TypeInfo {
        INFINITYTYPE_TYPE_INFO
    }
}


pub const INFINITYTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InfinityType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("InfinityType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RgbColorTrackData {
    pub keyframes: Vec<RgbColorKeyframe>,
}

pub const RGBCOLORTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: RGBCOLORKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(RgbColorTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(RGBCOLORTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for RgbColorTrackData {
    fn type_info() -> &'static TypeInfo {
        RGBCOLORTRACKDATA_TYPE_INFO
    }
}


pub const RGBCOLORTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RgbColorTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct RgbColorKeyframe {
    pub time: f32,
    pub r_g_b_color: super::core::Vec3,
}

pub const RGBCOLORKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(RgbColorKeyframe, time),
            },
            FieldInfoData {
                name: "RGBColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC3_TYPE_INFO,
                rust_offset: offset_of!(RgbColorKeyframe, r_g_b_color),
            },
        ],
    }),
    array_type: Some(RGBCOLORKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for RgbColorKeyframe {
    fn type_info() -> &'static TypeInfo {
        RGBCOLORKEYFRAME_TYPE_INFO
    }
}


pub const RGBCOLORKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "RgbColorKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("RgbColorKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorTrackData {
    pub keyframes: Vec<ColorKeyframe>,
}

pub const COLORTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: COLORKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(ColorTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(COLORTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ColorTrackData {
    fn type_info() -> &'static TypeInfo {
        COLORTRACKDATA_TYPE_INFO
    }
}


pub const COLORTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ColorTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ColorKeyframe {
    pub time: f32,
    pub r_g_b_color: super::core::Vec4,
}

pub const COLORKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(ColorKeyframe, time),
            },
            FieldInfoData {
                name: "RGBColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(ColorKeyframe, r_g_b_color),
            },
        ],
    }),
    array_type: Some(COLORKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorKeyframe {
    fn type_info() -> &'static TypeInfo {
        COLORKEYFRAME_TYPE_INFO
    }
}


pub const COLORKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ColorKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoolTrackData {
    pub keyframes: Vec<BoolKeyframe>,
}

pub const BOOLTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: BOOLKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BoolTrackData, keyframes),
            },
        ],
    }),
    array_type: Some(BOOLTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BoolTrackData {
    fn type_info() -> &'static TypeInfo {
        BOOLTRACKDATA_TYPE_INFO
    }
}


pub const BOOLTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BoolTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BoolKeyframe {
    pub time: f32,
    pub value: bool,
}

pub const BOOLKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BoolKeyframe, time),
            },
            FieldInfoData {
                name: "Value",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(BoolKeyframe, value),
            },
        ],
    }),
    array_type: Some(BOOLKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BoolKeyframe {
    fn type_info() -> &'static TypeInfo {
        BOOLKEYFRAME_TYPE_INFO
    }
}


pub const BOOLKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BoolKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BoolKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BookmarkTrackData {
    pub keyframes: Vec<BookmarkKeyframe>,
    pub jump_offset_type: JumpOffsetType,
    pub jump_time: f32,
}

pub const BOOKMARKTRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Keyframes",
                flags: MemberInfoFlags::new(144),
                field_type: BOOKMARKKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(BookmarkTrackData, keyframes),
            },
            FieldInfoData {
                name: "JumpOffsetType",
                flags: MemberInfoFlags::new(0),
                field_type: JUMPOFFSETTYPE_TYPE_INFO,
                rust_offset: offset_of!(BookmarkTrackData, jump_offset_type),
            },
            FieldInfoData {
                name: "JumpTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BookmarkTrackData, jump_time),
            },
        ],
    }),
    array_type: Some(BOOKMARKTRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for BookmarkTrackData {
    fn type_info() -> &'static TypeInfo {
        BOOKMARKTRACKDATA_TYPE_INFO
    }
}


pub const BOOKMARKTRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BookmarkTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct BookmarkKeyframe {
    pub time: f32,
    pub name_i_d: i32,
}

pub const BOOKMARKKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkKeyframe",
    flags: MemberInfoFlags::new(32841),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(BookmarkKeyframe, time),
            },
            FieldInfoData {
                name: "NameID",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(BookmarkKeyframe, name_i_d),
            },
        ],
    }),
    array_type: Some(BOOKMARKKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for BookmarkKeyframe {
    fn type_info() -> &'static TypeInfo {
        BOOKMARKKEYFRAME_TYPE_INFO
    }
}


pub const BOOKMARKKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BookmarkKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("BookmarkKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum JumpOffsetType {
    #[default]
    JumpOffsetType_AbsoluteTime = 0,
    JumpOffsetType_Relative = 1,
    JumpOffsetType_AsPercent = 2,
}

pub const JUMPOFFSETTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpOffsetType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(JUMPOFFSETTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for JumpOffsetType {
    fn type_info() -> &'static TypeInfo {
        JUMPOFFSETTYPE_TYPE_INFO
    }
}


pub const JUMPOFFSETTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "JumpOffsetType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("JumpOffsetType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GuideTrackData {
    pub guide_track_priority: i32,
}

pub const GUIDETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GuideTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "GuideTrackPriority",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(GuideTrackData, guide_track_priority),
            },
        ],
    }),
    array_type: Some(GUIDETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for GuideTrackData {
    fn type_info() -> &'static TypeInfo {
        GUIDETRACKDATA_TYPE_INFO
    }
}


pub const GUIDETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GuideTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GuideTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineTrackData {
    pub expose_pins: bool,
    pub is_disabled: bool,
    pub conditions: Vec<TimelineTrackDataConditionsBase>,
    pub update_pass_flags: u16,
    pub bucket_pre_children_order: u16,
    pub bucket_order: u16,
}

pub const TIMELINETRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(GAMEOBJECTDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ExposePins",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineTrackData, expose_pins),
            },
            FieldInfoData {
                name: "IsDisabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineTrackData, is_disabled),
            },
            FieldInfoData {
                name: "Conditions",
                flags: MemberInfoFlags::new(144),
                field_type: TIMELINETRACKDATACONDITIONSBASE_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TimelineTrackData, conditions),
            },
            FieldInfoData {
                name: "UpdatePassFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TimelineTrackData, update_pass_flags),
            },
            FieldInfoData {
                name: "BucketPreChildrenOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TimelineTrackData, bucket_pre_children_order),
            },
            FieldInfoData {
                name: "BucketOrder",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TimelineTrackData, bucket_order),
            },
        ],
    }),
    array_type: Some(TIMELINETRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineTrackData {
    fn type_info() -> &'static TypeInfo {
        TIMELINETRACKDATA_TYPE_INFO
    }
}


pub const TIMELINETRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineExtraData {
}

pub const TIMELINEEXTRADATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineExtraData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEEXTRADATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineExtraData {
    fn type_info() -> &'static TypeInfo {
        TIMELINEEXTRADATA_TYPE_INFO
    }
}


pub const TIMELINEEXTRADATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineExtraData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineExtraData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineTrackDataConditionsBase {
}

pub const TIMELINETRACKDATACONDITIONSBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackDataConditionsBase",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINETRACKDATACONDITIONSBASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineTrackDataConditionsBase {
    fn type_info() -> &'static TypeInfo {
        TIMELINETRACKDATACONDITIONSBASE_TYPE_INFO
    }
}


pub const TIMELINETRACKDATACONDITIONSBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrackDataConditionsBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTrackDataConditionsBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineKeyframeData {
}

pub const TIMELINEKEYFRAMEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineKeyframeData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineKeyframeData {
    fn type_info() -> &'static TypeInfo {
        TIMELINEKEYFRAMEDATA_TYPE_INFO
    }
}


pub const TIMELINEKEYFRAMEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineKeyframeData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineKeyframeData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineTeleportHelper {
}

pub const TIMELINETELEPORTHELPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTeleportHelper",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINETELEPORTHELPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineTeleportHelper {
    fn type_info() -> &'static TypeInfo {
        TIMELINETELEPORTHELPER_TYPE_INFO
    }
}


pub const TIMELINETELEPORTHELPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTeleportHelper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTeleportHelper-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TimelineEntityData {
    pub realm: super::core::Realm,
    pub timeline_data: TimelineData,
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

pub const TIMELINEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntityData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, realm),
            },
            FieldInfoData {
                name: "TimelineData",
                flags: MemberInfoFlags::new(0),
                field_type: TIMELINEDATA_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, timeline_data),
            },
            FieldInfoData {
                name: "AutoPlay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, auto_play),
            },
            FieldInfoData {
                name: "DeltaTimeClock",
                flags: MemberInfoFlags::new(0),
                field_type: TIMELINECLOCK_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, delta_time_clock),
            },
            FieldInfoData {
                name: "TimeDilationType",
                flags: MemberInfoFlags::new(0),
                field_type: TIMEDELTATYPE_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, time_dilation_type),
            },
            FieldInfoData {
                name: "AutoInitConnectedProperties",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, auto_init_connected_properties),
            },
            FieldInfoData {
                name: "ResetTimeOnStarted",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, reset_time_on_started),
            },
            FieldInfoData {
                name: "AllowAnimationCarryForward",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, allow_animation_carry_forward),
            },
            FieldInfoData {
                name: "AlwaysEndOnPreFrame",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, always_end_on_pre_frame),
            },
            FieldInfoData {
                name: "SyncTimelines",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, sync_timelines),
            },
            FieldInfoData {
                name: "RuntimeFramerate",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, runtime_framerate),
            },
            FieldInfoData {
                name: "BlendInTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, blend_in_time),
            },
            FieldInfoData {
                name: "BlendOutTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, blend_out_time),
            },
            FieldInfoData {
                name: "Looping",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, looping),
            },
            FieldInfoData {
                name: "Infinite",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, infinite),
            },
            FieldInfoData {
                name: "InitTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, init_time),
            },
            FieldInfoData {
                name: "StartTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, start_time),
            },
            FieldInfoData {
                name: "EndTime",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, end_time),
            },
            FieldInfoData {
                name: "PlaybackRate",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, playback_rate),
            },
            FieldInfoData {
                name: "PrintCurrentTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, print_current_time),
            },
            FieldInfoData {
                name: "UpdatePassFlags",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(TimelineEntityData, update_pass_flags),
            },
        ],
    }),
    array_type: Some(TIMELINEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineEntityData {
    fn type_info() -> &'static TypeInfo {
        TIMELINEENTITYDATA_TYPE_INFO
    }
}


pub const TIMELINEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineData {
    pub children: Vec<TimelineTrackData>,
}

pub const TIMELINEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: TIMELINETRACKDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(TimelineData, children),
            },
        ],
    }),
    array_type: Some(TIMELINEDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineData {
    fn type_info() -> &'static TypeInfo {
        TIMELINEDATA_TYPE_INFO
    }
}


pub const TIMELINEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TimelineEditorReinitializeState {
    #[default]
    TimelineEditorReinitializeState_ReadyToReinitialize = 0,
    TimelineEditorReinitializeState_BeginReceivingMetaProperties = 1,
    TimelineEditorReinitializeState_FinishedReceivingMetaProperties = 2,
}

pub const TIMELINEEDITORREINITIALIZESTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEditorReinitializeState",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(TIMELINEEDITORREINITIALIZESTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimelineEditorReinitializeState {
    fn type_info() -> &'static TypeInfo {
        TIMELINEEDITORREINITIALIZESTATE_TYPE_INFO
    }
}


pub const TIMELINEEDITORREINITIALIZESTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEditorReinitializeState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineEditorReinitializeState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TimelineClock {
    #[default]
    Clock_SimTimeWithTimeDilation = 0,
    Clock_RawSimTime = 1,
    Clock_RealTime = 2,
}

pub const TIMELINECLOCK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineClock",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(TIMELINECLOCK_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimelineClock {
    fn type_info() -> &'static TypeInfo {
        TIMELINECLOCK_TYPE_INFO
    }
}


pub const TIMELINECLOCK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineClock-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineClock-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum TimelineFramerate {
    #[default]
    FPS_24 = 24,
    FPS_25 = 25,
    FPS_30 = 30,
    FPS_60 = 60,
    FPS_100 = 100,
    FPS_120 = 120,
}

pub const TIMELINEFRAMERATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineFramerate",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(TIMELINEFRAMERATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for TimelineFramerate {
    fn type_info() -> &'static TypeInfo {
        TIMELINEFRAMERATE_TYPE_INFO
    }
}


pub const TIMELINEFRAMERATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineFramerate-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineFramerate-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FbxImportData {
}

pub const FBXIMPORTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbxImportData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINEEXTRADATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBXIMPORTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FbxImportData {
    fn type_info() -> &'static TypeInfo {
        FBXIMPORTDATA_TYPE_INFO
    }
}


pub const FBXIMPORTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FbxImportData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("FbxImportData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineBufferingHelper {
}

pub const TIMELINEBUFFERINGHELPER_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineBufferingHelper",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEBUFFERINGHELPER_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TimelineBufferingHelper {
    fn type_info() -> &'static TypeInfo {
        TIMELINEBUFFERINGHELPER_TYPE_INFO
    }
}


pub const TIMELINEBUFFERINGHELPER_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineBufferingHelper-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineBufferingHelper-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec4TrackData {
    pub x: FloatTrackData,
    pub y: FloatTrackData,
    pub z: FloatTrackData,
    pub w: FloatTrackData,
}

pub const VEC4TRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4TrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec4TrackData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec4TrackData, y),
            },
            FieldInfoData {
                name: "Z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec4TrackData, z),
            },
            FieldInfoData {
                name: "W",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec4TrackData, w),
            },
        ],
    }),
    array_type: Some(VEC4TRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec4TrackData {
    fn type_info() -> &'static TypeInfo {
        VEC4TRACKDATA_TYPE_INFO
    }
}


pub const VEC4TRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4TrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec4TrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec3TrackData {
    pub x: FloatTrackData,
    pub y: FloatTrackData,
    pub z: FloatTrackData,
}

pub const VEC3TRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3TrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec3TrackData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec3TrackData, y),
            },
            FieldInfoData {
                name: "Z",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec3TrackData, z),
            },
        ],
    }),
    array_type: Some(VEC3TRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3TrackData {
    fn type_info() -> &'static TypeInfo {
        VEC3TRACKDATA_TYPE_INFO
    }
}


pub const VEC3TRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3TrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec3TrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct Vec2TrackData {
    pub x: FloatTrackData,
    pub y: FloatTrackData,
}

pub const VEC2TRACKDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2TrackData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYTRACKBASEDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "X",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec2TrackData, x),
            },
            FieldInfoData {
                name: "Y",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(Vec2TrackData, y),
            },
        ],
    }),
    array_type: Some(VEC2TRACKDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2TrackData {
    fn type_info() -> &'static TypeInfo {
        VEC2TRACKDATA_TYPE_INFO
    }
}


pub const VEC2TRACKDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2TrackData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("Vec2TrackData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct TransformLayerData {
    pub weight: FloatTrackData,
    pub blendtype: LayeredTransform_BlendType,
}

pub const TRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACKDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Weight",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(TransformLayerData, weight),
            },
            FieldInfoData {
                name: "Blendtype",
                flags: MemberInfoFlags::new(0),
                field_type: LAYEREDTRANSFORM_BLENDTYPE_TYPE_INFO,
                rust_offset: offset_of!(TransformLayerData, blendtype),
            },
        ],
    }),
    array_type: Some(TRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformLayerData {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMLAYERDATA_TYPE_INFO
    }
}


pub const TRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TransformLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum LayeredTransform_BlendType {
    #[default]
    LayeredTransform_BlendType_WorldOverride = 0,
    LayeredTransform_BlendType_WorldAdditive = 1,
    LayeredTransform_BlendType_WorldTranslationLocalRotationAdditive = 2,
    LayeredTransform_BlendType_LocalAdditive = 3,
    LayeredTransform_BlendType_Special = 4,
}

pub const LAYEREDTRANSFORM_BLENDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransform_BlendType",
    flags: MemberInfoFlags::new(49429),
    module: "Timeline",
    data: TypeInfoData::Enum,
    array_type: Some(LAYEREDTRANSFORM_BLENDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LayeredTransform_BlendType {
    fn type_info() -> &'static TypeInfo {
        LAYEREDTRANSFORM_BLENDTYPE_TYPE_INFO
    }
}


pub const LAYEREDTRANSFORM_BLENDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LayeredTransform_BlendType-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("LayeredTransform_BlendType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct StartingLocationTransformLayerData {
}

pub const STARTINGLOCATIONTRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartingLocationTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STARTINGLOCATIONTRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StartingLocationTransformLayerData {
    fn type_info() -> &'static TypeInfo {
        STARTINGLOCATIONTRANSFORMLAYERDATA_TYPE_INFO
    }
}


pub const STARTINGLOCATIONTRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StartingLocationTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("StartingLocationTransformLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct ScaleTransformLayerData {
    pub is_uniform: bool,
    pub scale_x: FloatTrackData,
    pub scale_y: FloatTrackData,
    pub scale_z: FloatTrackData,
}

pub const SCALETRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "IsUniform",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(ScaleTransformLayerData, is_uniform),
            },
            FieldInfoData {
                name: "ScaleX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(ScaleTransformLayerData, scale_x),
            },
            FieldInfoData {
                name: "ScaleY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(ScaleTransformLayerData, scale_y),
            },
            FieldInfoData {
                name: "ScaleZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(ScaleTransformLayerData, scale_z),
            },
        ],
    }),
    array_type: Some(SCALETRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ScaleTransformLayerData {
    fn type_info() -> &'static TypeInfo {
        SCALETRANSFORMLAYERDATA_TYPE_INFO
    }
}


pub const SCALETRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ScaleTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("ScaleTransformLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct KeyedTransformLayerData {
    pub force_minimum_rotation_path_between_keys: bool,
    pub pre_infinity_for_min_rotation: InfinityType,
    pub post_infinity_for_min_rotation: InfinityType,
    pub translation_x: FloatTrackData,
    pub translation_y: FloatTrackData,
    pub translation_z: FloatTrackData,
    pub rotation_x: FloatTrackData,
    pub rotation_y: FloatTrackData,
    pub rotation_z: FloatTrackData,
    pub quat_keyframes: Vec<QuatKeyframe>,
}

pub const KEYEDTRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "KeyedTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ForceMinimumRotationPathBetweenKeys",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, force_minimum_rotation_path_between_keys),
            },
            FieldInfoData {
                name: "PreInfinityForMinRotation",
                flags: MemberInfoFlags::new(0),
                field_type: INFINITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, pre_infinity_for_min_rotation),
            },
            FieldInfoData {
                name: "PostInfinityForMinRotation",
                flags: MemberInfoFlags::new(0),
                field_type: INFINITYTYPE_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, post_infinity_for_min_rotation),
            },
            FieldInfoData {
                name: "TranslationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, translation_x),
            },
            FieldInfoData {
                name: "TranslationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, translation_y),
            },
            FieldInfoData {
                name: "TranslationZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, translation_z),
            },
            FieldInfoData {
                name: "RotationX",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, rotation_x),
            },
            FieldInfoData {
                name: "RotationY",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, rotation_y),
            },
            FieldInfoData {
                name: "RotationZ",
                flags: MemberInfoFlags::new(0),
                field_type: FLOATTRACKDATA_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, rotation_z),
            },
            FieldInfoData {
                name: "QuatKeyframes",
                flags: MemberInfoFlags::new(144),
                field_type: QUATKEYFRAME_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(KeyedTransformLayerData, quat_keyframes),
            },
        ],
    }),
    array_type: Some(KEYEDTRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for KeyedTransformLayerData {
    fn type_info() -> &'static TypeInfo {
        KEYEDTRANSFORMLAYERDATA_TYPE_INFO
    }
}


pub const KEYEDTRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "KeyedTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("KeyedTransformLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct QuatKeyframe {
    pub time: f32,
    pub quat_value: super::core::Vec4,
}

pub const QUATKEYFRAME_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuatKeyframe",
    flags: MemberInfoFlags::new(36937),
    module: "Timeline",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Time",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(QuatKeyframe, time),
            },
            FieldInfoData {
                name: "QuatValue",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(QuatKeyframe, quat_value),
            },
        ],
    }),
    array_type: Some(QUATKEYFRAME_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for QuatKeyframe {
    fn type_info() -> &'static TypeInfo {
        QUATKEYFRAME_TYPE_INFO
    }
}


pub const QUATKEYFRAME_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuatKeyframe-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("QuatKeyframe-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct GroupTransformLayerData {
    pub children: Vec<TransformLayerData>,
}

pub const GROUPTRANSFORMLAYERDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTransformLayerData",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TRANSFORMLAYERDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Children",
                flags: MemberInfoFlags::new(144),
                field_type: TRANSFORMLAYERDATA_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(GroupTransformLayerData, children),
            },
        ],
    }),
    array_type: Some(GROUPTRANSFORMLAYERDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for GroupTransformLayerData {
    fn type_info() -> &'static TypeInfo {
        GROUPTRANSFORMLAYERDATA_TYPE_INFO
    }
}


pub const GROUPTRANSFORMLAYERDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GroupTransformLayerData-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("GroupTransformLayerData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineTrack {
}

pub const TIMELINETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(TIMELINETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TimelineTrack {
    fn type_info() -> &'static TypeInfo {
        TIMELINETRACK_TYPE_INFO
    }
}


pub const TIMELINETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineRootTrack {
}

pub const TIMELINEROOTTRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineRootTrack",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEROOTTRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TimelineRootTrack {
    fn type_info() -> &'static TypeInfo {
        TIMELINEROOTTRACK_TYPE_INFO
    }
}


pub const TIMELINEROOTTRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineRootTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineRootTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TimelineEntity {
}

pub const TIMELINEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntity",
    flags: MemberInfoFlags::new(101),
    module: "Timeline",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TIMELINEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TimelineEntity {
    fn type_info() -> &'static TypeInfo {
        TIMELINEENTITY_TYPE_INFO
    }
}


pub const TIMELINEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TimelineEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "Timeline",
    data: TypeInfoData::Array("TimelineEntity-Array"),
    array_type: None,
    alignment: 8,
};


