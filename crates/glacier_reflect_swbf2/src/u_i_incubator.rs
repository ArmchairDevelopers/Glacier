use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_u_i_incubator_types(registry: &mut TypeRegistry) {
    registry.register_type(CLIENTQUITGAMEENTITY_TYPE_INFO);
    registry.register_type(CLIENTQUITGAMEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMINTERPOLATORENTITY_TYPE_INFO);
    registry.register_type(TRANSFORMINTERPOLATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC4INTERPOLATORENTITY_TYPE_INFO);
    registry.register_type(VEC4INTERPOLATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC3INTERPOLATORENTITY_TYPE_INFO);
    registry.register_type(VEC3INTERPOLATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(VEC2INTERPOLATORENTITY_TYPE_INFO);
    registry.register_type(VEC2INTERPOLATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FLOATINTERPOLATORENTITY_TYPE_INFO);
    registry.register_type(FLOATINTERPOLATORENTITY_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHUBENTITY_TYPE_INFO);
    registry.register_type(OBJECTHUBENTITY_ARRAY_TYPE_INFO);
    registry.register_type(MATHINTOPENTITY_TYPE_INFO);
    registry.register_type(MATHINTOPENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYBASE_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESWITCHENTITY_TYPE_INFO);
    registry.register_type(TEXTURESWITCHENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUISTATICTEXTUREELEMENTENTITY_TYPE_INFO);
    registry.register_type(FBUISTATICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUIDYNAMICTEXTUREELEMENTENTITY_TYPE_INFO);
    registry.register_type(FBUIDYNAMICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUISLICEDTEXTUREELEMENTENTITY_TYPE_INFO);
    registry.register_type(FBUISLICEDTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUIMOVIEELEMENTENTITY_TYPE_INFO);
    registry.register_type(FBUIMOVIEELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUILISTITEMWIDGETENTITY_TYPE_INFO);
    registry.register_type(FBUILISTITEMWIDGETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUILISTELEMENTENTITY_TYPE_INFO);
    registry.register_type(FBUILISTELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(FBUILABELELEMENTENTITY_TYPE_INFO);
    registry.register_type(FBUILABELELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICCASTENTITY_TYPE_INFO);
    registry.register_type(DYNAMICCASTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGENTITY_TYPE_INFO);
    registry.register_type(CONFIGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALSTRINGENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALSTRINGENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALTRANSFORMENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALTRANSFORMENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC4ENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC4ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC3ENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC3ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC2ENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC2ENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALFLOATENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALFLOATENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALINTENTITY_TYPE_INFO);
    registry.register_type(CONDITIONALINTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTUIMASKINGWIDGETENTITY_TYPE_INFO);
    registry.register_type(CLIENTUIMASKINGWIDGETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STRINGSWITCHCASEENTITY_TYPE_INFO);
    registry.register_type(STRINGSWITCHCASEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(INTEGERSWITCHCASEENTITY_TYPE_INFO);
    registry.register_type(INTEGERSWITCHCASEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(STATENODEENTITYBASE_TYPE_INFO);
    registry.register_type(STATENODEENTITYBASE_ARRAY_TYPE_INFO);
    registry.register_type(STATENODEENTITY_TYPE_INFO);
    registry.register_type(STATENODEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(SELECTOBJECTENTITY_TYPE_INFO);
    registry.register_type(SELECTOBJECTENTITY_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientQuitGameEntity {
}

pub const CLIENTQUITGAMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuitGameEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTQUITGAMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientQuitGameEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTQUITGAMEENTITY_TYPE_INFO
    }
}


pub const CLIENTQUITGAMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuitGameEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ClientQuitGameEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TransformInterpolatorEntity {
}

pub const TRANSFORMINTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMINTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformInterpolatorEntity {
    fn type_info() -> &'static TypeInfo {
        TRANSFORMINTERPOLATORENTITY_TYPE_INFO
    }
}


pub const TRANSFORMINTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("TransformInterpolatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec4InterpolatorEntity {
}

pub const VEC4INTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC4INTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4InterpolatorEntity {
    fn type_info() -> &'static TypeInfo {
        VEC4INTERPOLATORENTITY_TYPE_INFO
    }
}


pub const VEC4INTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("Vec4InterpolatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec3InterpolatorEntity {
}

pub const VEC3INTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC3INTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3InterpolatorEntity {
    fn type_info() -> &'static TypeInfo {
        VEC3INTERPOLATORENTITY_TYPE_INFO
    }
}


pub const VEC3INTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("Vec3InterpolatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2InterpolatorEntity {
}

pub const VEC2INTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(VEC2INTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec2InterpolatorEntity {
    fn type_info() -> &'static TypeInfo {
        VEC2INTERPOLATORENTITY_TYPE_INFO
    }
}


pub const VEC2INTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("Vec2InterpolatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FloatInterpolatorEntity {
}

pub const FLOATINTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FLOATINTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatInterpolatorEntity {
    fn type_info() -> &'static TypeInfo {
        FLOATINTERPOLATORENTITY_TYPE_INFO
    }
}


pub const FLOATINTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FloatInterpolatorEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ObjectHubEntity {
}

pub const OBJECTHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(OBJECTHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectHubEntity {
    fn type_info() -> &'static TypeInfo {
        OBJECTHUBENTITY_TYPE_INFO
    }
}


pub const OBJECTHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ObjectHubEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MathIntOpEntity {
}

pub const MATHINTOPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(MATHINTOPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MathIntOpEntity {
    fn type_info() -> &'static TypeInfo {
        MATHINTOPENTITY_TYPE_INFO
    }
}


pub const MATHINTOPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("MathIntOpEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringEntityBase {
}

pub const LOCALIZEDSTRINGENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringEntityBase {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITYBASE_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("LocalizedStringEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizedStringEntity {
}

pub const LOCALIZEDSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZEDSTRINGENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringEntity {
    fn type_info() -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITY_TYPE_INFO
    }
}


pub const LOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("LocalizedStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TextureSwitchEntity {
}

pub const TEXTURESWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(TEXTURESWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TextureSwitchEntity {
    fn type_info() -> &'static TypeInfo {
        TEXTURESWITCHENTITY_TYPE_INFO
    }
}


pub const TEXTURESWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("TextureSwitchEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUIStaticTextureElementEntity {
}

pub const FBUISTATICTEXTUREELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUISTATICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIStaticTextureElementEntity {
    fn type_info() -> &'static TypeInfo {
        FBUISTATICTEXTUREELEMENTENTITY_TYPE_INFO
    }
}


pub const FBUISTATICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIStaticTextureElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUIDynamicTextureElementEntity {
}

pub const FBUIDYNAMICTEXTUREELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUIDYNAMICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIDynamicTextureElementEntity {
    fn type_info() -> &'static TypeInfo {
        FBUIDYNAMICTEXTUREELEMENTENTITY_TYPE_INFO
    }
}


pub const FBUIDYNAMICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIDynamicTextureElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUISlicedTextureElementEntity {
}

pub const FBUISLICEDTEXTUREELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUISLICEDTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUISlicedTextureElementEntity {
    fn type_info() -> &'static TypeInfo {
        FBUISLICEDTEXTUREELEMENTENTITY_TYPE_INFO
    }
}


pub const FBUISLICEDTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUISlicedTextureElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUIMovieElementEntity {
}

pub const FBUIMOVIEELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUIMOVIEELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIMovieElementEntity {
    fn type_info() -> &'static TypeInfo {
        FBUIMOVIEELEMENTENTITY_TYPE_INFO
    }
}


pub const FBUIMOVIEELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIMovieElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUIListItemWidgetEntity {
}

pub const FBUILISTITEMWIDGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIWIDGETENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUILISTITEMWIDGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIListItemWidgetEntity {
    fn type_info() -> &'static TypeInfo {
        FBUILISTITEMWIDGETENTITY_TYPE_INFO
    }
}


pub const FBUILISTITEMWIDGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIListItemWidgetEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUIListElementEntity {
}

pub const FBUILISTELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUILISTELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIListElementEntity {
    fn type_info() -> &'static TypeInfo {
        FBUILISTELEMENTENTITY_TYPE_INFO
    }
}


pub const FBUILISTELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIListElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FBUILabelElementEntity {
}

pub const FBUILABELELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FBUILABELELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUILabelElementEntity {
    fn type_info() -> &'static TypeInfo {
        FBUILABELELEMENTENTITY_TYPE_INFO
    }
}


pub const FBUILABELELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUILabelElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DynamicCastEntity {
}

pub const DYNAMICCASTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICCASTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicCastEntity {
    fn type_info() -> &'static TypeInfo {
        DYNAMICCASTENTITY_TYPE_INFO
    }
}


pub const DYNAMICCASTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("DynamicCastEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConfigEntity {
}

pub const CONFIGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONFIGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConfigEntity {
    fn type_info() -> &'static TypeInfo {
        CONFIGENTITY_TYPE_INFO
    }
}


pub const CONFIGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConfigEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalStringEntity {
}

pub const CONDITIONALSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalStringEntity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALSTRINGENTITY_TYPE_INFO
    }
}


pub const CONDITIONALSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalStringEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalTransformEntity {
}

pub const CONDITIONALTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalTransformEntity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALTRANSFORMENTITY_TYPE_INFO
    }
}


pub const CONDITIONALTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalTransformEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalVec4Entity {
}

pub const CONDITIONALVEC4ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4Entity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALVEC4ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalVec4Entity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALVEC4ENTITY_TYPE_INFO
    }
}


pub const CONDITIONALVEC4ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalVec4Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalVec3Entity {
}

pub const CONDITIONALVEC3ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3Entity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALVEC3ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalVec3Entity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALVEC3ENTITY_TYPE_INFO
    }
}


pub const CONDITIONALVEC3ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalVec3Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalVec2Entity {
}

pub const CONDITIONALVEC2ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2Entity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALVEC2ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalVec2Entity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALVEC2ENTITY_TYPE_INFO
    }
}


pub const CONDITIONALVEC2ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalVec2Entity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalFloatEntity {
}

pub const CONDITIONALFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalFloatEntity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALFLOATENTITY_TYPE_INFO
    }
}


pub const CONDITIONALFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalFloatEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ConditionalIntEntity {
}

pub const CONDITIONALINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalIntEntity {
    fn type_info() -> &'static TypeInfo {
        CONDITIONALINTENTITY_TYPE_INFO
    }
}


pub const CONDITIONALINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalIntEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientUIMaskingWidgetEntity {
}

pub const CLIENTUIMASKINGWIDGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIMaskingWidgetEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIWIDGETENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUIMASKINGWIDGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUIMaskingWidgetEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTUIMASKINGWIDGETENTITY_TYPE_INFO
    }
}


pub const CLIENTUIMASKINGWIDGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIMaskingWidgetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ClientUIMaskingWidgetEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StringSwitchCaseEntity {
}

pub const STRINGSWITCHCASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STRINGSWITCHCASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StringSwitchCaseEntity {
    fn type_info() -> &'static TypeInfo {
        STRINGSWITCHCASEENTITY_TYPE_INFO
    }
}


pub const STRINGSWITCHCASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("StringSwitchCaseEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IntegerSwitchCaseEntity {
}

pub const INTEGERSWITCHCASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INTEGERSWITCHCASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntegerSwitchCaseEntity {
    fn type_info() -> &'static TypeInfo {
        INTEGERSWITCHCASEENTITY_TYPE_INFO
    }
}


pub const INTEGERSWITCHCASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("IntegerSwitchCaseEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StateNodeEntityBase {
}

pub const STATENODEENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATENODEENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StateNodeEntityBase {
    fn type_info() -> &'static TypeInfo {
        STATENODEENTITYBASE_TYPE_INFO
    }
}


pub const STATENODEENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("StateNodeEntityBase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct StateNodeEntity {
}

pub const STATENODEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATENODEENTITYBASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(STATENODEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StateNodeEntity {
    fn type_info() -> &'static TypeInfo {
        STATENODEENTITY_TYPE_INFO
    }
}


pub const STATENODEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("StateNodeEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SelectObjectEntity {
}

pub const SELECTOBJECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(SELECTOBJECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectObjectEntity {
    fn type_info() -> &'static TypeInfo {
        SELECTOBJECTENTITY_TYPE_INFO
    }
}


pub const SELECTOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("SelectObjectEntity-Array"),
    array_type: None,
    alignment: 8,
};


