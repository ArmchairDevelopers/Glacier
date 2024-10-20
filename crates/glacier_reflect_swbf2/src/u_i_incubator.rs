use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct ClientQuitGameEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientQuitGameEntityTrait: super::entity::EntityTrait {
}

impl ClientQuitGameEntityTrait for ClientQuitGameEntity {
}

impl super::entity::EntityTrait for ClientQuitGameEntity {
}

impl super::entity::EntityBusPeerTrait for ClientQuitGameEntity {
}

pub static CLIENTQUITGAMEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuitGameEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientQuitGameEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTQUITGAMEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientQuitGameEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTQUITGAMEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTQUITGAMEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientQuitGameEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ClientQuitGameEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransformInterpolatorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TransformInterpolatorEntityTrait: super::entity::EntityTrait {
}

impl TransformInterpolatorEntityTrait for TransformInterpolatorEntity {
}

impl super::entity::EntityTrait for TransformInterpolatorEntity {
}

impl super::entity::EntityBusPeerTrait for TransformInterpolatorEntity {
}

pub static TRANSFORMINTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformInterpolatorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TRANSFORMINTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TransformInterpolatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMINTERPOLATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TRANSFORMINTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("TransformInterpolatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec4InterpolatorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait Vec4InterpolatorEntityTrait: super::entity::EntityTrait {
}

impl Vec4InterpolatorEntityTrait for Vec4InterpolatorEntity {
}

impl super::entity::EntityTrait for Vec4InterpolatorEntity {
}

impl super::entity::EntityBusPeerTrait for Vec4InterpolatorEntity {
}

pub static VEC4INTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec4InterpolatorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEC4INTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec4InterpolatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VEC4INTERPOLATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC4INTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("Vec4InterpolatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3InterpolatorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait Vec3InterpolatorEntityTrait: super::entity::EntityTrait {
}

impl Vec3InterpolatorEntityTrait for Vec3InterpolatorEntity {
}

impl super::entity::EntityTrait for Vec3InterpolatorEntity {
}

impl super::entity::EntityBusPeerTrait for Vec3InterpolatorEntity {
}

pub static VEC3INTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3InterpolatorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEC3INTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec3InterpolatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3INTERPOLATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC3INTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("Vec3InterpolatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec2InterpolatorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait Vec2InterpolatorEntityTrait: super::entity::EntityTrait {
}

impl Vec2InterpolatorEntityTrait for Vec2InterpolatorEntity {
}

impl super::entity::EntityTrait for Vec2InterpolatorEntity {
}

impl super::entity::EntityBusPeerTrait for Vec2InterpolatorEntity {
}

pub static VEC2INTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec2InterpolatorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(VEC2INTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Vec2InterpolatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        VEC2INTERPOLATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static VEC2INTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("Vec2InterpolatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatInterpolatorEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait FloatInterpolatorEntityTrait: super::entity::EntityTrait {
}

impl FloatInterpolatorEntityTrait for FloatInterpolatorEntity {
}

impl super::entity::EntityTrait for FloatInterpolatorEntity {
}

impl super::entity::EntityBusPeerTrait for FloatInterpolatorEntity {
}

pub static FLOATINTERPOLATORENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatInterpolatorEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FLOATINTERPOLATORENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FloatInterpolatorEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATINTERPOLATORENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FLOATINTERPOLATORENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FloatInterpolatorEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectHubEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ObjectHubEntityTrait: super::entity::EntityTrait {
}

impl ObjectHubEntityTrait for ObjectHubEntity {
}

impl super::entity::EntityTrait for ObjectHubEntity {
}

impl super::entity::EntityBusPeerTrait for ObjectHubEntity {
}

pub static OBJECTHUBENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectHubEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(OBJECTHUBENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ObjectHubEntity {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHUBENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static OBJECTHUBENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ObjectHubEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MathIntOpEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait MathIntOpEntityTrait: super::entity::EntityTrait {
}

impl MathIntOpEntityTrait for MathIntOpEntity {
}

impl super::entity::EntityTrait for MathIntOpEntity {
}

impl super::entity::EntityBusPeerTrait for MathIntOpEntity {
}

pub static MATHINTOPENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MathIntOpEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(MATHINTOPENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for MathIntOpEntity {
    fn type_info(&self) -> &'static TypeInfo {
        MATHINTOPENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static MATHINTOPENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("MathIntOpEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedStringEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait LocalizedStringEntityBaseTrait: super::entity::EntityTrait {
}

impl LocalizedStringEntityBaseTrait for LocalizedStringEntityBase {
}

impl super::entity::EntityTrait for LocalizedStringEntityBase {
}

impl super::entity::EntityBusPeerTrait for LocalizedStringEntityBase {
}

pub static LOCALIZEDSTRINGENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALIZEDSTRINGENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("LocalizedStringEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedStringEntity {
    pub _glacier_base: LocalizedStringEntityBase,
}

pub trait LocalizedStringEntityTrait: LocalizedStringEntityBaseTrait {
}

impl LocalizedStringEntityTrait for LocalizedStringEntity {
}

impl LocalizedStringEntityBaseTrait for LocalizedStringEntity {
}

impl super::entity::EntityTrait for LocalizedStringEntity {
}

impl super::entity::EntityBusPeerTrait for LocalizedStringEntity {
}

pub static LOCALIZEDSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZEDSTRINGENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static LOCALIZEDSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("LocalizedStringEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureSwitchEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait TextureSwitchEntityTrait: super::entity::EntityTrait {
}

impl TextureSwitchEntityTrait for TextureSwitchEntity {
}

impl super::entity::EntityTrait for TextureSwitchEntity {
}

impl super::entity::EntityBusPeerTrait for TextureSwitchEntity {
}

pub static TEXTURESWITCHENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureSwitchEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(TEXTURESWITCHENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for TextureSwitchEntity {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURESWITCHENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static TEXTURESWITCHENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("TextureSwitchEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIStaticTextureElementEntity {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait FBUIStaticTextureElementEntityTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl FBUIStaticTextureElementEntityTrait for FBUIStaticTextureElementEntity {
}

impl super::game_client_u_i::UIElementEntityTrait for FBUIStaticTextureElementEntity {
}

impl super::entity::EntityTrait for FBUIStaticTextureElementEntity {
}

impl super::entity::EntityBusPeerTrait for FBUIStaticTextureElementEntity {
}

pub static FBUISTATICTEXTUREELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIStaticTextureElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUISTATICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIStaticTextureElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUISTATICTEXTUREELEMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUISTATICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIStaticTextureElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIDynamicTextureElementEntity {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait FBUIDynamicTextureElementEntityTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl FBUIDynamicTextureElementEntityTrait for FBUIDynamicTextureElementEntity {
}

impl super::game_client_u_i::UIElementEntityTrait for FBUIDynamicTextureElementEntity {
}

impl super::entity::EntityTrait for FBUIDynamicTextureElementEntity {
}

impl super::entity::EntityBusPeerTrait for FBUIDynamicTextureElementEntity {
}

pub static FBUIDYNAMICTEXTUREELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIDynamicTextureElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUIDYNAMICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIDynamicTextureElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUIDYNAMICTEXTUREELEMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUIDYNAMICTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIDynamicTextureElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUISlicedTextureElementEntity {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait FBUISlicedTextureElementEntityTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl FBUISlicedTextureElementEntityTrait for FBUISlicedTextureElementEntity {
}

impl super::game_client_u_i::UIElementEntityTrait for FBUISlicedTextureElementEntity {
}

impl super::entity::EntityTrait for FBUISlicedTextureElementEntity {
}

impl super::entity::EntityBusPeerTrait for FBUISlicedTextureElementEntity {
}

pub static FBUISLICEDTEXTUREELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUISlicedTextureElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUISLICEDTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUISlicedTextureElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUISLICEDTEXTUREELEMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUISLICEDTEXTUREELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUISlicedTextureElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIMovieElementEntity {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait FBUIMovieElementEntityTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl FBUIMovieElementEntityTrait for FBUIMovieElementEntity {
}

impl super::game_client_u_i::UIElementEntityTrait for FBUIMovieElementEntity {
}

impl super::entity::EntityTrait for FBUIMovieElementEntity {
}

impl super::entity::EntityBusPeerTrait for FBUIMovieElementEntity {
}

pub static FBUIMOVIEELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIMovieElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUIMOVIEELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIMovieElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUIMOVIEELEMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUIMOVIEELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIMovieElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIListItemWidgetEntity {
    pub _glacier_base: super::game_client_u_i::UIWidgetEntity,
}

pub trait FBUIListItemWidgetEntityTrait: super::game_client_u_i::UIWidgetEntityTrait {
}

impl FBUIListItemWidgetEntityTrait for FBUIListItemWidgetEntity {
}

impl super::game_client_u_i::UIWidgetEntityTrait for FBUIListItemWidgetEntity {
}

impl super::entity::EntityTrait for FBUIListItemWidgetEntity {
}

impl super::entity::EntityBusPeerTrait for FBUIListItemWidgetEntity {
}

pub static FBUILISTITEMWIDGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIWIDGETENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIListItemWidgetEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUILISTITEMWIDGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIListItemWidgetEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUILISTITEMWIDGETENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUILISTITEMWIDGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIListItemWidgetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIListElementEntity {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait FBUIListElementEntityTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl FBUIListElementEntityTrait for FBUIListElementEntity {
}

impl super::game_client_u_i::UIElementEntityTrait for FBUIListElementEntity {
}

impl super::entity::EntityTrait for FBUIListElementEntity {
}

impl super::entity::EntityBusPeerTrait for FBUIListElementEntity {
}

pub static FBUILISTELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIListElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUILISTELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUIListElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUILISTELEMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUILISTELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUIListElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUILabelElementEntity {
    pub _glacier_base: super::game_client_u_i::UIElementEntity,
}

pub trait FBUILabelElementEntityTrait: super::game_client_u_i::UIElementEntityTrait {
}

impl FBUILabelElementEntityTrait for FBUILabelElementEntity {
}

impl super::game_client_u_i::UIElementEntityTrait for FBUILabelElementEntity {
}

impl super::entity::EntityTrait for FBUILabelElementEntity {
}

impl super::entity::EntityBusPeerTrait for FBUILabelElementEntity {
}

pub static FBUILABELELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUILabelElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FBUILABELELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for FBUILabelElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        FBUILABELELEMENTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FBUILABELELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("FBUILabelElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicCastEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait DynamicCastEntityTrait: super::entity::EntityTrait {
}

impl DynamicCastEntityTrait for DynamicCastEntity {
}

impl super::entity::EntityTrait for DynamicCastEntity {
}

impl super::entity::EntityBusPeerTrait for DynamicCastEntity {
}

pub static DYNAMICCASTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicCastEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(DYNAMICCASTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for DynamicCastEntity {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICCASTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static DYNAMICCASTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("DynamicCastEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConfigEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConfigEntityTrait: super::entity::EntityTrait {
}

impl ConfigEntityTrait for ConfigEntity {
}

impl super::entity::EntityTrait for ConfigEntity {
}

impl super::entity::EntityBusPeerTrait for ConfigEntity {
}

pub static CONFIGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConfigEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONFIGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConfigEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONFIGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONFIGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConfigEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalStringEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalStringEntityTrait: super::entity::EntityTrait {
}

impl ConditionalStringEntityTrait for ConditionalStringEntity {
}

impl super::entity::EntityTrait for ConditionalStringEntity {
}

impl super::entity::EntityBusPeerTrait for ConditionalStringEntity {
}

pub static CONDITIONALSTRINGENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalStringEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALSTRINGENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalStringEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALSTRINGENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALSTRINGENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalStringEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalTransformEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalTransformEntityTrait: super::entity::EntityTrait {
}

impl ConditionalTransformEntityTrait for ConditionalTransformEntity {
}

impl super::entity::EntityTrait for ConditionalTransformEntity {
}

impl super::entity::EntityBusPeerTrait for ConditionalTransformEntity {
}

pub static CONDITIONALTRANSFORMENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalTransformEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALTRANSFORMENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalTransformEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALTRANSFORMENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALTRANSFORMENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalTransformEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalVec4Entity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalVec4EntityTrait: super::entity::EntityTrait {
}

impl ConditionalVec4EntityTrait for ConditionalVec4Entity {
}

impl super::entity::EntityTrait for ConditionalVec4Entity {
}

impl super::entity::EntityBusPeerTrait for ConditionalVec4Entity {
}

pub static CONDITIONALVEC4ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4Entity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalVec4Entity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALVEC4ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalVec4Entity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALVEC4ENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALVEC4ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalVec4Entity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalVec3Entity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalVec3EntityTrait: super::entity::EntityTrait {
}

impl ConditionalVec3EntityTrait for ConditionalVec3Entity {
}

impl super::entity::EntityTrait for ConditionalVec3Entity {
}

impl super::entity::EntityBusPeerTrait for ConditionalVec3Entity {
}

pub static CONDITIONALVEC3ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3Entity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalVec3Entity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALVEC3ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalVec3Entity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALVEC3ENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALVEC3ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalVec3Entity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalVec2Entity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalVec2EntityTrait: super::entity::EntityTrait {
}

impl ConditionalVec2EntityTrait for ConditionalVec2Entity {
}

impl super::entity::EntityTrait for ConditionalVec2Entity {
}

impl super::entity::EntityBusPeerTrait for ConditionalVec2Entity {
}

pub static CONDITIONALVEC2ENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2Entity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalVec2Entity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALVEC2ENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalVec2Entity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALVEC2ENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALVEC2ENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2Entity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalVec2Entity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalFloatEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalFloatEntityTrait: super::entity::EntityTrait {
}

impl ConditionalFloatEntityTrait for ConditionalFloatEntity {
}

impl super::entity::EntityTrait for ConditionalFloatEntity {
}

impl super::entity::EntityBusPeerTrait for ConditionalFloatEntity {
}

pub static CONDITIONALFLOATENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalFloatEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALFLOATENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalFloatEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALFLOATENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALFLOATENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalFloatEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalIntEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ConditionalIntEntityTrait: super::entity::EntityTrait {
}

impl ConditionalIntEntityTrait for ConditionalIntEntity {
}

impl super::entity::EntityTrait for ConditionalIntEntity {
}

impl super::entity::EntityBusPeerTrait for ConditionalIntEntity {
}

pub static CONDITIONALINTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalIntEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONDITIONALINTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalIntEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALINTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CONDITIONALINTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ConditionalIntEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientUIMaskingWidgetEntity {
    pub _glacier_base: super::game_client_u_i::UIWidgetEntity,
}

pub trait ClientUIMaskingWidgetEntityTrait: super::game_client_u_i::UIWidgetEntityTrait {
}

impl ClientUIMaskingWidgetEntityTrait for ClientUIMaskingWidgetEntity {
}

impl super::game_client_u_i::UIWidgetEntityTrait for ClientUIMaskingWidgetEntity {
}

impl super::entity::EntityTrait for ClientUIMaskingWidgetEntity {
}

impl super::entity::EntityBusPeerTrait for ClientUIMaskingWidgetEntity {
}

pub static CLIENTUIMASKINGWIDGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIMaskingWidgetEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIWIDGETENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUIMaskingWidgetEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUIMASKINGWIDGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUIMaskingWidgetEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTUIMASKINGWIDGETENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static CLIENTUIMASKINGWIDGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIMaskingWidgetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("ClientUIMaskingWidgetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StringSwitchCaseEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait StringSwitchCaseEntityTrait: super::entity::EntityTrait {
}

impl StringSwitchCaseEntityTrait for StringSwitchCaseEntity {
}

impl super::entity::EntityTrait for StringSwitchCaseEntity {
}

impl super::entity::EntityBusPeerTrait for StringSwitchCaseEntity {
}

pub static STRINGSWITCHCASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StringSwitchCaseEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STRINGSWITCHCASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StringSwitchCaseEntity {
    fn type_info(&self) -> &'static TypeInfo {
        STRINGSWITCHCASEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STRINGSWITCHCASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("StringSwitchCaseEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntegerSwitchCaseEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait IntegerSwitchCaseEntityTrait: super::entity::EntityTrait {
}

impl IntegerSwitchCaseEntityTrait for IntegerSwitchCaseEntity {
}

impl super::entity::EntityTrait for IntegerSwitchCaseEntity {
}

impl super::entity::EntityBusPeerTrait for IntegerSwitchCaseEntity {
}

pub static INTEGERSWITCHCASEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntegerSwitchCaseEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INTEGERSWITCHCASEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IntegerSwitchCaseEntity {
    fn type_info(&self) -> &'static TypeInfo {
        INTEGERSWITCHCASEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INTEGERSWITCHCASEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("IntegerSwitchCaseEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StateNodeEntityBase {
    pub _glacier_base: super::entity::Entity,
}

pub trait StateNodeEntityBaseTrait: super::entity::EntityTrait {
}

impl StateNodeEntityBaseTrait for StateNodeEntityBase {
}

impl super::entity::EntityTrait for StateNodeEntityBase {
}

impl super::entity::EntityBusPeerTrait for StateNodeEntityBase {
}

pub static STATENODEENTITYBASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBase",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StateNodeEntityBase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATENODEENTITYBASE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StateNodeEntityBase {
    fn type_info(&self) -> &'static TypeInfo {
        STATENODEENTITYBASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATENODEENTITYBASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBase-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("StateNodeEntityBase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StateNodeEntity {
    pub _glacier_base: StateNodeEntityBase,
}

pub trait StateNodeEntityTrait: StateNodeEntityBaseTrait {
}

impl StateNodeEntityTrait for StateNodeEntity {
}

impl StateNodeEntityBaseTrait for StateNodeEntity {
}

impl super::entity::EntityTrait for StateNodeEntity {
}

impl super::entity::EntityBusPeerTrait for StateNodeEntity {
}

pub static STATENODEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATENODEENTITYBASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StateNodeEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(STATENODEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StateNodeEntity {
    fn type_info(&self) -> &'static TypeInfo {
        STATENODEENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static STATENODEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("StateNodeEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SelectObjectEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait SelectObjectEntityTrait: super::entity::EntityTrait {
}

impl SelectObjectEntityTrait for SelectObjectEntity {
}

impl super::entity::EntityTrait for SelectObjectEntity {
}

impl super::entity::EntityBusPeerTrait for SelectObjectEntity {
}

pub static SELECTOBJECTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntity",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubator",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SelectObjectEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(SELECTOBJECTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for SelectObjectEntity {
    fn type_info(&self) -> &'static TypeInfo {
        SELECTOBJECTENTITY_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static SELECTOBJECTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubator",
    data: TypeInfoData::Array("SelectObjectEntity"),
    array_type: None,
    alignment: 8,
};


