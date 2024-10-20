use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_game_client_u_i_types(registry: &mut TypeRegistry) {
    registry.register_type(UISCREENRENDERTARGETENTITY_TYPE_INFO);
    registry.register_type(UISCREENRENDERTARGETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTUISCREENRENDERENTITY_TYPE_INFO);
    registry.register_type(CLIENTUISCREENRENDERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UIWIDGETENTITY_TYPE_INFO);
    registry.register_type(UIWIDGETENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTLAYERENTITY_TYPE_INFO);
    registry.register_type(UIELEMENTLAYERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTWIDGETREFERENCEENTITY_TYPE_INFO);
    registry.register_type(UIELEMENTWIDGETREFERENCEENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UIMASKINGCONTAINERENTITY_TYPE_INFO);
    registry.register_type(UIMASKINGCONTAINERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UICONTAINERENTITY_TYPE_INFO);
    registry.register_type(UICONTAINERENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UIELEMENTENTITY_TYPE_INFO);
    registry.register_type(UIELEMENTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UICPPSCREEN_TYPE_INFO);
    registry.register_type(UICPPSCREEN_ARRAY_TYPE_INFO);
    registry.register_type(UISYSTEMPOSTINITCOMPLETEMESSAGE_TYPE_INFO);
    registry.register_type(PLAYVIDEOENTITYDATA_TYPE_INFO);
    registry.register_type(PLAYVIDEOENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MOVIEDEBUGENTITYDATA_TYPE_INFO);
    registry.register_type(MOVIEDEBUGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIHITZONECOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIHITZONECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIHITZONECOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIHITZONECOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENPROJECTIONCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UISCREENPROJECTIONCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UISCREENPROJECTIONCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UISCREENPROJECTIONCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIINPUTCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIINPUTCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIMOUSEINPUTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIMOUSEINPUTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIMOUSEINPUTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIMOUSEINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWCLEARCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWCLEARCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWVIEWPORTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWVIEWPORTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWVIEWPORTCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWVIEWPORTCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWBLENDCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWBLENDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWBLENDCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWBLENDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWBLENDCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWBLENDCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWDEPTHCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWDEPTHCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWDEPTHCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWDEPTHCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWDEPTHCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWDEPTHCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILMASKCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILMASKCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILMASKCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILMASKCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWSTENCILCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWTRANSFORMCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWTRANSFORMCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWTRANSFORMCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWTRANSFORMCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWCOLORCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWCOLORCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWCOLORCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWCOLORCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWCOLORCOMMANDTYPE_TYPE_INFO);
    registry.register_type(UIDRAWCOLORCOMMANDTYPE_ARRAY_TYPE_INFO);
    registry.register_type(UIMOVIECOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIMOVIECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIMOVIECOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIMOVIECOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIMEASURETEXTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIMEASURETEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIMEASURETEXTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIMEASURETEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWTEXTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWTEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWTEXTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWTEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWAACIRCLECOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWAACIRCLECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWLINELISTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWLINELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWLINELISTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWLINELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIGRADIENTRECTPARAMS_TYPE_INFO);
    registry.register_type(UIGRADIENTRECTPARAMS_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWRECTCOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTUIINPUTENTITY_TYPE_INFO);
    registry.register_type(CLIENTUIINPUTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTMOVIETRACK_TYPE_INFO);
    registry.register_type(CLIENTMOVIETRACK_ARRAY_TYPE_INFO);
    registry.register_type(UISYSTEM_TYPE_INFO);
    registry.register_type(UISYSTEM_ARRAY_TYPE_INFO);
    registry.register_type(CLIENTPLAYVIDEOENTITY_TYPE_INFO);
    registry.register_type(CLIENTPLAYVIDEOENTITY_ARRAY_TYPE_INFO);
    registry.register_type(UITTFFONTFILE_TYPE_INFO);
    registry.register_type(UITTFFONTFILE_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIScreenRenderTargetEntity {
}

pub const UISCREENRENDERTARGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTUISCREENRENDERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UISCREENRENDERTARGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIScreenRenderTargetEntity {
    fn type_info() -> &'static TypeInfo {
        UISCREENRENDERTARGETENTITY_TYPE_INFO
    }
}


pub const UISCREENRENDERTARGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIScreenRenderTargetEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientUIScreenRenderEntity {
}

pub const CLIENTUISCREENRENDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIScreenRenderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUISCREENRENDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUIScreenRenderEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTUISCREENRENDERENTITY_TYPE_INFO
    }
}


pub const CLIENTUISCREENRENDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIScreenRenderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientUIScreenRenderEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWidgetEntity {
}

pub const UIWIDGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIWIDGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIWidgetEntity {
    fn type_info() -> &'static TypeInfo {
        UIWIDGETENTITY_TYPE_INFO
    }
}


pub const UIWIDGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIWidgetEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementLayerEntity {
}

pub const UIELEMENTLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementLayerEntity {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTLAYERENTITY_TYPE_INFO
    }
}


pub const UIELEMENTLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIElementLayerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementWidgetReferenceEntity {
}

pub const UIELEMENTWIDGETREFERENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTWIDGETREFERENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementWidgetReferenceEntity {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTWIDGETREFERENCEENTITY_TYPE_INFO
    }
}


pub const UIELEMENTWIDGETREFERENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIElementWidgetReferenceEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIMaskingContainerEntity {
}

pub const UIMASKINGCONTAINERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UICONTAINERENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIMASKINGCONTAINERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIMaskingContainerEntity {
    fn type_info() -> &'static TypeInfo {
        UIMASKINGCONTAINERENTITY_TYPE_INFO
    }
}


pub const UIMASKINGCONTAINERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMaskingContainerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIContainerEntity {
}

pub const UICONTAINERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UICONTAINERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIContainerEntity {
    fn type_info() -> &'static TypeInfo {
        UICONTAINERENTITY_TYPE_INFO
    }
}


pub const UICONTAINERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIContainerEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIElementEntity {
}

pub const UIELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementEntity {
    fn type_info() -> &'static TypeInfo {
        UIELEMENTENTITY_TYPE_INFO
    }
}


pub const UIELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIElementEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UICppScreen {
}

pub const UICPPSCREEN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreen",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(UICPPSCREEN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UICppScreen {
    fn type_info() -> &'static TypeInfo {
        UICPPSCREEN_TYPE_INFO
    }
}


pub const UICPPSCREEN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreen-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UICppScreen-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UISystemPostInitCompleteMessage {
}

pub const UISYSTEMPOSTINITCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystemPostInitCompleteMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UISystemPostInitCompleteMessage {
    fn type_info() -> &'static TypeInfo {
        UISYSTEMPOSTINITCOMPLETEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlayVideoEntityData {
    pub movie: super::movie_base::MovieTextureBaseAsset,
    pub network_streaming_url: String,
    pub r#loop: bool,
    pub keep_black_screen: bool,
    pub allow_skip: bool,
    pub draw_in_widget: bool,
    pub render_world: bool,
    pub use_sim_time: bool,
    pub video_identifier: String,
}

pub const PLAYVIDEOENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayVideoEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: MOVIETEXTUREBASEASSET_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, movie),
            },
            FieldInfoData {
                name: "NetworkStreamingUrl",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, network_streaming_url),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, r#loop),
            },
            FieldInfoData {
                name: "KeepBlackScreen",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, keep_black_screen),
            },
            FieldInfoData {
                name: "AllowSkip",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, allow_skip),
            },
            FieldInfoData {
                name: "DrawInWidget",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, draw_in_widget),
            },
            FieldInfoData {
                name: "RenderWorld",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, render_world),
            },
            FieldInfoData {
                name: "UseSimTime",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, use_sim_time),
            },
            FieldInfoData {
                name: "VideoIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(PlayVideoEntityData, video_identifier),
            },
        ],
    }),
    array_type: Some(PLAYVIDEOENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayVideoEntityData {
    fn type_info() -> &'static TypeInfo {
        PLAYVIDEOENTITYDATA_TYPE_INFO
    }
}


pub const PLAYVIDEOENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayVideoEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("PlayVideoEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MovieDebugEntityData {
    pub language_override: super::core::LanguageFormat,
    pub stop_frame: i32,
}

pub const MOVIEDEBUGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDebugEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LanguageOverride",
                flags: MemberInfoFlags::new(0),
                field_type: LANGUAGEFORMAT_TYPE_INFO,
                rust_offset: offset_of!(MovieDebugEntityData, language_override),
            },
            FieldInfoData {
                name: "StopFrame",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(MovieDebugEntityData, stop_frame),
            },
        ],
    }),
    array_type: Some(MOVIEDEBUGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieDebugEntityData {
    fn type_info() -> &'static TypeInfo {
        MOVIEDEBUGENTITYDATA_TYPE_INFO
    }
}


pub const MOVIEDEBUGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDebugEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("MovieDebugEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIHitZoneCommandDynamicState {
    pub rect: super::core::Vec4,
    pub transform: super::core::Mat4,
    pub view_projection: super::core::Mat4,
    pub field_flag_changed0: u8,
}

pub const UIHITZONECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: MAT4_TYPE_INFO,
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, transform),
            },
            FieldInfoData {
                name: "ViewProjection",
                flags: MemberInfoFlags::new(0),
                field_type: MAT4_TYPE_INFO,
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, view_projection),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIHITZONECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIHitZoneCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIHITZONECOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIHITZONECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIHitZoneCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIHitZoneCommandStaticState {
    pub measure_handle: super::game_base::UIImReverseHandle,
    pub field_flag_changed0: u8,
}

pub const UIHITZONECOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "MeasureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMREVERSEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIHitZoneCommandStaticState, measure_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIHitZoneCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIHITZONECOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIHitZoneCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIHITZONECOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIHITZONECOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIHitZoneCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIScreenProjectionCommandDynamicState {
    pub input: super::core::Vec2,
    pub plane: super::core::Vec4,
    pub is_input_normalized: bool,
    pub field_flag_changed0: u8,
}

pub const UISCREENPROJECTIONCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Input",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, input),
            },
            FieldInfoData {
                name: "Plane",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, plane),
            },
            FieldInfoData {
                name: "IsInputNormalized",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, is_input_normalized),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UISCREENPROJECTIONCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIScreenProjectionCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UISCREENPROJECTIONCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UISCREENPROJECTIONCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIScreenProjectionCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIScreenProjectionCommandStaticState {
    pub reverse_handle: super::game_base::UIImReverseHandle,
    pub field_flag_changed0: u8,
}

pub const UISCREENPROJECTIONCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ReverseHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMREVERSEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIScreenProjectionCommandStaticState, reverse_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIScreenProjectionCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UISCREENPROJECTIONCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIScreenProjectionCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UISCREENPROJECTIONCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UISCREENPROJECTIONCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIScreenProjectionCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIInputCommandStaticState {
    pub command_type: UIInputCommandType,
    pub reverse_handle: super::game_base::UIImReverseHandle,
    pub z_depth: f32,
    pub input_action: i32,
    pub field_flag_changed0: u8,
}

pub const UIINPUTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIINPUTCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIInputCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "ReverseHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMREVERSEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIInputCommandStaticState, reverse_handle),
            },
            FieldInfoData {
                name: "ZDepth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputCommandStaticState, z_depth),
            },
            FieldInfoData {
                name: "InputAction",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIInputCommandStaticState, input_action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIInputCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIInputCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIINPUTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIInputCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIInputCommandType {
    #[default]
    UIInputCommandType_EnterLayer = 0,
    UIInputCommandType_LeaveLayer = 1,
    UIInputCommandType_RequestAllInput = 2,
    UIInputCommandType_RequestMouseEveryWhere = 3,
    UIInputCommandType_RequestInputAction = 4,
    UIInputCommandType_RequestText = 5,
    UIInputCommandType_MadeTextInputRequest = 6,
}

pub const UIINPUTCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIINPUTCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIInputCommandType {
    fn type_info() -> &'static TypeInfo {
        UIINPUTCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIINPUTCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIInputCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIMouseInputCommandDynamicState {
    pub tag: String,
    pub listen_rect: super::core::Vec4,
    pub plane: super::core::Vec4,
    pub screen_local_transform: super::core::LinearTransform,
    pub mouse_input_type_mask: i32,
    pub field_flag_changed0: u8,
}

pub const UIMOUSEINPUTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Tag",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, tag),
            },
            FieldInfoData {
                name: "ListenRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, listen_rect),
            },
            FieldInfoData {
                name: "Plane",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, plane),
            },
            FieldInfoData {
                name: "ScreenLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, screen_local_transform),
            },
            FieldInfoData {
                name: "MouseInputTypeMask",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, mouse_input_type_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOUSEINPUTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIMouseInputCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIMOUSEINPUTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIMOUSEINPUTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMouseInputCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIMouseInputCommandStaticState {
    pub reverse_handle: super::game_base::UIImReverseHandle,
    pub field_flag_changed0: u8,
}

pub const UIMOUSEINPUTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ReverseHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMREVERSEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandStaticState, reverse_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIMouseInputCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOUSEINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIMouseInputCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIMOUSEINPUTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIMOUSEINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMouseInputCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawSectionNameCommandDynamicState {
    pub name: String,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSectionNameCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSectionNameCommandDynamicState, name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSectionNameCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawSectionNameCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSectionNameCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawSectionNameCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawClearCommandDynamicState {
    pub color: super::core::Vec4,
    pub stencil: u8,
    pub clear_mask: u32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWCLEARCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawClearCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, color),
            },
            FieldInfoData {
                name: "Stencil",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, stencil),
            },
            FieldInfoData {
                name: "ClearMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, clear_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWCLEARCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawClearCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWCLEARCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWCLEARCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawClearCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawClearCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawViewportCommandDynamicState {
    pub viewport: super::render_base::ViewportRect,
    pub field_flag_changed0: u8,
}

pub const UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Viewport",
                flags: MemberInfoFlags::new(0),
                field_type: VIEWPORTRECT_TYPE_INFO,
                rust_offset: offset_of!(UIDrawViewportCommandDynamicState, viewport),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawViewportCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawViewportCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawViewportCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawViewportCommandStaticState {
    pub command_type: UIDrawViewportCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWVIEWPORTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWVIEWPORTCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawViewportCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawViewportCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWVIEWPORTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawViewportCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWVIEWPORTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWVIEWPORTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawViewportCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawViewportCommandType {
    #[default]
    UIDrawViewportCommandType_Push = 0,
    UIDrawViewportCommandType_Pop = 1,
}

pub const UIDRAWVIEWPORTCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWVIEWPORTCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawViewportCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWVIEWPORTCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWVIEWPORTCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawViewportCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawBlendCommandDynamicState {
    pub enabled: bool,
    pub separate_alpha_blend: bool,
    pub color_op: super::render::RenderBlendOp,
    pub alpha_op: super::render::RenderBlendOp,
    pub source_color: super::render::RenderBlendMode,
    pub dest_color: super::render::RenderBlendMode,
    pub source_alpha: super::render::RenderBlendMode,
    pub dest_alpha: super::render::RenderBlendMode,
    pub field_flag_changed0: u8,
}

pub const UIDRAWBLENDCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, enabled),
            },
            FieldInfoData {
                name: "SeparateAlphaBlend",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, separate_alpha_blend),
            },
            FieldInfoData {
                name: "ColorOp",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBLENDOP_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, color_op),
            },
            FieldInfoData {
                name: "AlphaOp",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBLENDOP_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, alpha_op),
            },
            FieldInfoData {
                name: "SourceColor",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, source_color),
            },
            FieldInfoData {
                name: "DestColor",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, dest_color),
            },
            FieldInfoData {
                name: "SourceAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, source_alpha),
            },
            FieldInfoData {
                name: "DestAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERBLENDMODE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, dest_alpha),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWBLENDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawBlendCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWBLENDCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWBLENDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawBlendCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawBlendCommandStaticState {
    pub command_type: UIDrawBlendCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWBLENDCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWBLENDCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawBlendCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWBLENDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawBlendCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWBLENDCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWBLENDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawBlendCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawBlendCommandType {
    #[default]
    UIDrawBlendCommandType_Push = 0,
    UIDrawBlendCommandType_Pop = 1,
}

pub const UIDRAWBLENDCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWBLENDCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawBlendCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWBLENDCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWBLENDCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawBlendCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawDepthCommandDynamicState {
    pub mode: super::render::RenderDepthMode,
    pub field_flag_changed0: u8,
}

pub const UIDRAWDEPTHCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: RENDERDEPTHMODE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDepthCommandDynamicState, mode),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDepthCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDEPTHCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawDepthCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWDEPTHCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWDEPTHCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDepthCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawDepthCommandStaticState {
    pub command_type: UIDrawDepthCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWDEPTHCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWDEPTHCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDepthCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDepthCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDEPTHCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawDepthCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWDEPTHCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWDEPTHCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDepthCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawDepthCommandType {
    #[default]
    UIDrawDepthCommandType_Push = 0,
    UIDrawDepthCommandType_Pop = 1,
}

pub const UIDRAWDEPTHCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWDEPTHCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawDepthCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWDEPTHCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWDEPTHCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDepthCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawStencilMaskCommandDynamicState {
    pub invert_test: bool,
    pub rect: super::core::Vec4,
    pub texture: super::render_base::TextureResourceHandle,
    pub uv_rect: super::core::Vec4,
    pub alpha_threshold: f32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "InvertTest",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, invert_test),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, uv_rect),
            },
            FieldInfoData {
                name: "AlphaThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, alpha_threshold),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawStencilMaskCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilMaskCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawStencilMaskCommandStaticState {
    pub command_type: UIDrawStencilMaskCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSTENCILMASKCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWSTENCILMASKCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilMaskCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILMASKCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawStencilMaskCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSTENCILMASKCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSTENCILMASKCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilMaskCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawStencilMaskCommandType {
    #[default]
    UIDrawStencilMaskCommandType_Push = 0,
    UIDrawStencilMaskCommandType_Pop = 1,
}

pub const UIDRAWSTENCILMASKCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWSTENCILMASKCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawStencilMaskCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSTENCILMASKCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWSTENCILMASKCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilMaskCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawStencilCommandDynamicState {
    pub enabled: bool,
    pub write_color: bool,
    pub r#ref: u8,
    pub stencil_mask: u8,
    pub func: super::render::DepthStencilCompareFunc,
    pub fail_op: super::render::StencilOperation,
    pub depth_fail_op: super::render::StencilOperation,
    pub pass_op: super::render::StencilOperation,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSTENCILCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, enabled),
            },
            FieldInfoData {
                name: "WriteColor",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, write_color),
            },
            FieldInfoData {
                name: "Ref",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, r#ref),
            },
            FieldInfoData {
                name: "StencilMask",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, stencil_mask),
            },
            FieldInfoData {
                name: "Func",
                flags: MemberInfoFlags::new(0),
                field_type: DEPTHSTENCILCOMPAREFUNC_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, func),
            },
            FieldInfoData {
                name: "FailOp",
                flags: MemberInfoFlags::new(0),
                field_type: STENCILOPERATION_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, fail_op),
            },
            FieldInfoData {
                name: "DepthFailOp",
                flags: MemberInfoFlags::new(0),
                field_type: STENCILOPERATION_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, depth_fail_op),
            },
            FieldInfoData {
                name: "PassOp",
                flags: MemberInfoFlags::new(0),
                field_type: STENCILOPERATION_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, pass_op),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawStencilCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSTENCILCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSTENCILCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawStencilCommandStaticState {
    pub command_type: UIDrawStencilCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSTENCILCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWSTENCILCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawStencilCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawStencilCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSTENCILCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSTENCILCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawStencilCommandType {
    #[default]
    UIDrawStencilCommandType_Push = 0,
    UIDrawStencilCommandType_Pop = 1,
}

pub const UIDRAWSTENCILCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWSTENCILCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawStencilCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSTENCILCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWSTENCILCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIClipThresholdCommandDynamicState {
    pub threshold: f32,
    pub field_flag_changed0: u8,
}

pub const UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIClipThresholdCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Threshold",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIClipThresholdCommandDynamicState, threshold),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIClipThresholdCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIClipThresholdCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIClipThresholdCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIClipThresholdCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawTransformCommandDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub field_flag_changed0: u8,
}

pub const UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: LINEARTRANSFORM_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTransformCommandDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTransformCommandDynamicState, enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTransformCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawTransformCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTransformCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawTransformCommandStaticState {
    pub command_type: UIDrawTransformCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWTRANSFORMCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWTRANSFORMCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTransformCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTransformCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTRANSFORMCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawTransformCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWTRANSFORMCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWTRANSFORMCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTransformCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawTransformCommandType {
    #[default]
    UIDrawTransformCommandType_Push = 0,
    UIDrawTransformCommandType_Replace = 1,
    UIDrawTransformCommandType_Pop = 2,
}

pub const UIDRAWTRANSFORMCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWTRANSFORMCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawTransformCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWTRANSFORMCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWTRANSFORMCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTransformCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawColorCommandDynamicState {
    pub color: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub const UIDRAWCOLORCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawColorCommandDynamicState, color),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawColorCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWCOLORCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawColorCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWCOLORCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWCOLORCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawColorCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawColorCommandStaticState {
    pub command_type: UIDrawColorCommandType,
    pub field_flag_changed0: u8,
}

pub const UIDRAWCOLORCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: UIDRAWCOLORCOMMANDTYPE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawColorCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawColorCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWCOLORCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawColorCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWCOLORCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWCOLORCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawColorCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum UIDrawColorCommandType {
    #[default]
    UIDrawColorCommandType_Push = 0,
    UIDrawColorCommandType_Pop = 1,
}

pub const UIDRAWCOLORCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWCOLORCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawColorCommandType {
    fn type_info() -> &'static TypeInfo {
        UIDRAWCOLORCOMMANDTYPE_TYPE_INFO
    }
}


pub const UIDRAWCOLORCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawColorCommandType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIMovieCommandDynamicState {
    pub rect: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub const UIMOVIECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIMovieCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIMovieCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOVIECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIMovieCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIMOVIECOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIMOVIECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMovieCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIMovieCommandStaticState {
    pub movie: super::movie_base::MovieHandle,
    pub field_flag_changed0: u8,
}

pub const UIMOVIECOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: MOVIEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIMovieCommandStaticState, movie),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIMovieCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOVIECOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIMovieCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIMOVIECOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIMOVIECOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMovieCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIMeasureTextCommandDynamicState {
    pub text: String,
    pub measure_width: f32,
    pub custom_text_layout_callback: u64,
    pub field_flag_changed0: u8,
}

pub const UIMEASURETEXTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, text),
            },
            FieldInfoData {
                name: "MeasureWidth",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, measure_width),
            },
            FieldInfoData {
                name: "CustomTextLayoutCallback",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, custom_text_layout_callback),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMEASURETEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIMeasureTextCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIMEASURETEXTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIMEASURETEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMeasureTextCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIMeasureTextCommandStaticState {
    pub config: super::game_base::UIImTextCommandConfig,
    pub measure_handle: super::game_base::UIImReverseHandle,
    pub measure_only_visible_glyphs: bool,
    pub field_flag_changed0: u8,
}

pub const UIMEASURETEXTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Config",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMTEXTCOMMANDCONFIG_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, config),
            },
            FieldInfoData {
                name: "MeasureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMREVERSEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, measure_handle),
            },
            FieldInfoData {
                name: "MeasureOnlyVisibleGlyphs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, measure_only_visible_glyphs),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMEASURETEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIMeasureTextCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIMEASURETEXTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIMEASURETEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMeasureTextCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawTextCommandDynamicState {
    pub text: String,
    pub rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub custom_text_layout_callback: u64,
    pub field_flag_changed0: u8,
}

pub const UIDRAWTEXTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, text),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "CustomTextLayoutCallback",
                flags: MemberInfoFlags::new(0),
                field_type: UINT64_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, custom_text_layout_callback),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawTextCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWTEXTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWTEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTextCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawTextCommandStaticState {
    pub config: super::game_base::UIImTextCommandConfig,
    pub shader_program: i32,
    pub measure_handle: super::game_base::UIImReverseHandle,
    pub measure_only_visible_glyphs: bool,
    pub field_flag_changed0: u8,
}

pub const UIDRAWTEXTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Config",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMTEXTCOMMANDCONFIG_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandStaticState, config),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "MeasureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: UIIMREVERSEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandStaticState, measure_handle),
            },
            FieldInfoData {
                name: "MeasureOnlyVisibleGlyphs",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandStaticState, measure_only_visible_glyphs),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTextCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawTextCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWTEXTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWTEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTextCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawAACircleCommandDynamicState {
    pub center: super::core::Vec2,
    pub radius: f32,
    pub thickness: f32,
    pub non_premultiplied_color: super::core::Vec4,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWAACIRCLECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAACircleCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Center",
                flags: MemberInfoFlags::new(0),
                field_type: VEC2_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, center),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, radius),
            },
            FieldInfoData {
                name: "Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, thickness),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWAACIRCLECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawAACircleCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWAACIRCLECOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWAACIRCLECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAACircleCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAACircleCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawAALineStripCommandDynamicState {
    pub vertices: Vec<super::core::Vec2>,
    pub non_premultiplied_colors: Vec<super::core::Vec4>,
    pub width: f32,
    pub closed: bool,
    pub fill_left: bool,
    pub fill_right: bool,
    pub field_flag_changed0: u8,
}

pub const UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAALineStripCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "NonPremultipliedColors",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, non_premultiplied_colors),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: FLOAT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, width),
            },
            FieldInfoData {
                name: "Closed",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, closed),
            },
            FieldInfoData {
                name: "FillLeft",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, fill_left),
            },
            FieldInfoData {
                name: "FillRight",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, fill_right),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawAALineStripCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAALineStripCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAALineStripCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawLineListCommandDynamicState {
    pub vertices: Vec<super::core::Vec3>,
    pub non_premultiplied_colors: Vec<super::core::Vec4>,
    pub indices: Vec<u16>,
    pub field_flag_changed0: u8,
}

pub const UIDRAWLINELISTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "NonPremultipliedColors",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, non_premultiplied_colors),
            },
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, indices),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWLINELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawLineListCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWLINELISTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWLINELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawLineListCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawLineListCommandStaticState {
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWLINELISTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawLineListCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawLineListCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWLINELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawLineListCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWLINELISTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWLINELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawLineListCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawSolidTriangleListCommandDynamicState {
    pub vertices: Vec<super::core::Vec3>,
    pub non_premultiplied_colors: Vec<super::core::Vec4>,
    pub indices: Vec<u16>,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "NonPremultipliedColors",
                flags: MemberInfoFlags::new(144),
                field_type: VEC4_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, non_premultiplied_colors),
            },
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, indices),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawSolidTriangleListCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawSolidTriangleListCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawSolidTriangleListCommandStaticState {
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawSolidTriangleListCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawSolidTriangleListCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawTriangleListCommandDynamicState {
    pub texture: super::render_base::TextureResourceHandle,
    pub vertices: Vec<super::core::Vec3>,
    pub uvs: Vec<super::core::Vec2>,
    pub indices: Vec<u16>,
    pub field_flag_changed0: u8,
}

pub const UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTriangleListCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "Uvs",
                flags: MemberInfoFlags::new(144),
                field_type: VEC2_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, uvs),
            },
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: UINT16_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, indices),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawTriangleListCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTriangleListCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTriangleListCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawDistanceFieldCommandDynamicState {
    pub texture: super::render_base::TextureResourceHandle,
    pub corners: Vec<super::core::Vec3>,
    pub uv_rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub const UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "Corners",
                flags: MemberInfoFlags::new(144),
                field_type: VEC3_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, corners),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, uv_rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawDistanceFieldCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDistanceFieldCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawDistanceFieldCommandStaticState {
    pub params: super::game_shared_u_i::UIElementBitmapDistanceFieldParams,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: UIELEMENTBITMAPDISTANCEFIELDPARAMS_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandStaticState, params),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawDistanceFieldCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawDistanceFieldCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDistanceFieldCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawAdvancedRectCommandDynamicState {
    pub texture1: super::render_base::TextureResourceHandle,
    pub uv_rect1: super::core::Vec4,
    pub texture2: super::render_base::TextureResourceHandle,
    pub uv_rect2: super::core::Vec4,
    pub rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub gradient_params: UIGradientRectParams,
    pub field_flag_changed0: u8,
}

pub const UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Texture1",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, texture1),
            },
            FieldInfoData {
                name: "UvRect1",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, uv_rect1),
            },
            FieldInfoData {
                name: "Texture2",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, texture2),
            },
            FieldInfoData {
                name: "UvRect2",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, uv_rect2),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "GradientParams",
                flags: MemberInfoFlags::new(0),
                field_type: UIGRADIENTRECTPARAMS_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, gradient_params),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawAdvancedRectCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAdvancedRectCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIDrawAdvancedRectCommandStaticState {
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub filled: bool,
    pub outlined: bool,
    pub gradient: bool,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTUREADDRESS_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, address_v),
            },
            FieldInfoData {
                name: "Filled",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, filled),
            },
            FieldInfoData {
                name: "Outlined",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, outlined),
            },
            FieldInfoData {
                name: "Gradient",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, gradient),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawAdvancedRectCommandStaticState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_TYPE_INFO
    }
}


pub const UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAdvancedRectCommandStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIGradientRectParams {
    pub top_left_color: super::core::Vec4,
    pub top_right_color: super::core::Vec4,
    pub bottom_left_color: super::core::Vec4,
    pub bottom_right_color: super::core::Vec4,
}

pub const UIGRADIENTRECTPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGradientRectParams",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TopLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIGradientRectParams, top_left_color),
            },
            FieldInfoData {
                name: "TopRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIGradientRectParams, top_right_color),
            },
            FieldInfoData {
                name: "BottomLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIGradientRectParams, bottom_left_color),
            },
            FieldInfoData {
                name: "BottomRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIGradientRectParams, bottom_right_color),
            },
        ],
    }),
    array_type: Some(UIGRADIENTRECTPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIGradientRectParams {
    fn type_info() -> &'static TypeInfo {
        UIGRADIENTRECTPARAMS_TYPE_INFO
    }
}


pub const UIGRADIENTRECTPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGradientRectParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIGradientRectParams-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Debug)]
pub struct UIDrawRectCommandDynamicState {
    pub texture: super::render_base::TextureResourceHandle,
    pub rect: super::core::Vec4,
    pub uv_rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub const UIDRAWRECTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawRectCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: TEXTURERESOURCEHANDLE_TYPE_INFO,
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, uv_rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: VEC4_TYPE_INFO,
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawRectCommandDynamicState {
    fn type_info() -> &'static TypeInfo {
        UIDRAWRECTCOMMANDDYNAMICSTATE_TYPE_INFO
    }
}


pub const UIDRAWRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawRectCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawRectCommandDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientUIInputEntity {
}

pub const CLIENTUIINPUTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIInputEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUIINPUTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUIInputEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTUIINPUTENTITY_TYPE_INFO
    }
}


pub const CLIENTUIINPUTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIInputEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientUIInputEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientMovieTrack {
}

pub const CLIENTMOVIETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(TIMELINETRACK_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMOVIETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMovieTrack {
    fn type_info() -> &'static TypeInfo {
        CLIENTMOVIETRACK_TYPE_INFO
    }
}


pub const CLIENTMOVIETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientMovieTrack-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UISystem {
}

pub const UISYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystem",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IUISYSTEM_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UISYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UISystem {
    fn type_info() -> &'static TypeInfo {
        UISYSTEM_TYPE_INFO
    }
}


pub const UISYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UISystem-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ClientPlayVideoEntity {
}

pub const CLIENTPLAYVIDEOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVideoEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYVIDEOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayVideoEntity {
    fn type_info() -> &'static TypeInfo {
        CLIENTPLAYVIDEOENTITY_TYPE_INFO
    }
}


pub const CLIENTPLAYVIDEOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVideoEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientPlayVideoEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UITtfFontFile {
}

pub const UITTFFONTFILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfFontFile",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(UITTFFONTFILE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UITtfFontFile {
    fn type_info() -> &'static TypeInfo {
        UITTFFONTFILE_TYPE_INFO
    }
}


pub const UITTFFONTFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfFontFile-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UITtfFontFile-Array"),
    array_type: None,
    alignment: 8,
};


