use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct UIScreenRenderTargetEntity {
    pub _glacier_base: ClientUIScreenRenderEntity,
}

pub trait UIScreenRenderTargetEntityTrait: ClientUIScreenRenderEntityTrait {
}

impl UIScreenRenderTargetEntityTrait for UIScreenRenderTargetEntity {
}

impl ClientUIScreenRenderEntityTrait for UIScreenRenderTargetEntity {
}

impl super::entity::EntityTrait for UIScreenRenderTargetEntity {
}

impl super::entity::EntityBusPeerTrait for UIScreenRenderTargetEntity {
}

pub static UISCREENRENDERTARGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CLIENTUISCREENRENDERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenRenderTargetEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UISCREENRENDERTARGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIScreenRenderTargetEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENRENDERTARGETENTITY_TYPE_INFO
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


pub static UISCREENRENDERTARGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenRenderTargetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIScreenRenderTargetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientUIScreenRenderEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientUIScreenRenderEntityTrait: super::entity::EntityTrait {
}

impl ClientUIScreenRenderEntityTrait for ClientUIScreenRenderEntity {
}

impl super::entity::EntityTrait for ClientUIScreenRenderEntity {
}

impl super::entity::EntityBusPeerTrait for ClientUIScreenRenderEntity {
}

pub static CLIENTUISCREENRENDERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIScreenRenderEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUIScreenRenderEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUISCREENRENDERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUIScreenRenderEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTUISCREENRENDERENTITY_TYPE_INFO
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


pub static CLIENTUISCREENRENDERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIScreenRenderEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientUIScreenRenderEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIWidgetEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait UIWidgetEntityTrait: super::entity::EntityTrait {
}

impl UIWidgetEntityTrait for UIWidgetEntity {
}

impl super::entity::EntityTrait for UIWidgetEntity {
}

impl super::entity::EntityBusPeerTrait for UIWidgetEntity {
}

pub static UIWIDGETENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWidgetEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIWIDGETENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIWidgetEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UIWIDGETENTITY_TYPE_INFO
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


pub static UIWIDGETENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWidgetEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIWidgetEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementLayerEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait UIElementLayerEntityTrait: super::entity::EntityTrait {
}

impl UIElementLayerEntityTrait for UIElementLayerEntity {
}

impl super::entity::EntityTrait for UIElementLayerEntity {
}

impl super::entity::EntityBusPeerTrait for UIElementLayerEntity {
}

pub static UIELEMENTLAYERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementLayerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTLAYERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementLayerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTLAYERENTITY_TYPE_INFO
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


pub static UIELEMENTLAYERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementLayerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIElementLayerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementWidgetReferenceEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait UIElementWidgetReferenceEntityTrait: super::entity::EntityTrait {
}

impl UIElementWidgetReferenceEntityTrait for UIElementWidgetReferenceEntity {
}

impl super::entity::EntityTrait for UIElementWidgetReferenceEntity {
}

impl super::entity::EntityBusPeerTrait for UIElementWidgetReferenceEntity {
}

pub static UIELEMENTWIDGETREFERENCEENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementWidgetReferenceEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTWIDGETREFERENCEENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementWidgetReferenceEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTWIDGETREFERENCEENTITY_TYPE_INFO
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


pub static UIELEMENTWIDGETREFERENCEENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementWidgetReferenceEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIElementWidgetReferenceEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMaskingContainerEntity {
    pub _glacier_base: UIContainerEntity,
}

pub trait UIMaskingContainerEntityTrait: UIContainerEntityTrait {
}

impl UIMaskingContainerEntityTrait for UIMaskingContainerEntity {
}

impl UIContainerEntityTrait for UIMaskingContainerEntity {
}

impl UIElementEntityTrait for UIMaskingContainerEntity {
}

impl super::entity::EntityTrait for UIMaskingContainerEntity {
}

impl super::entity::EntityBusPeerTrait for UIMaskingContainerEntity {
}

pub static UIMASKINGCONTAINERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UICONTAINERENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMaskingContainerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIMASKINGCONTAINERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIMaskingContainerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UIMASKINGCONTAINERENTITY_TYPE_INFO
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


pub static UIMASKINGCONTAINERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingContainerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMaskingContainerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIContainerEntity {
    pub _glacier_base: UIElementEntity,
}

pub trait UIContainerEntityTrait: UIElementEntityTrait {
}

impl UIContainerEntityTrait for UIContainerEntity {
}

impl UIElementEntityTrait for UIContainerEntity {
}

impl super::entity::EntityTrait for UIContainerEntity {
}

impl super::entity::EntityBusPeerTrait for UIContainerEntity {
}

pub static UICONTAINERENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIELEMENTENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIContainerEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UICONTAINERENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIContainerEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UICONTAINERENTITY_TYPE_INFO
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


pub static UICONTAINERENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIContainerEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIContainerEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIElementEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait UIElementEntityTrait: super::entity::EntityTrait {
}

impl UIElementEntityTrait for UIElementEntity {
}

impl super::entity::EntityTrait for UIElementEntity {
}

impl super::entity::EntityBusPeerTrait for UIElementEntity {
}

pub static UIELEMENTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIElementEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIELEMENTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIElementEntity {
    fn type_info(&self) -> &'static TypeInfo {
        UIELEMENTENTITY_TYPE_INFO
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


pub static UIELEMENTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIElementEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIElementEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UICppScreen {
}

pub trait UICppScreenTrait: TypeObject {
}

impl UICppScreenTrait for UICppScreen {
}

pub static UICPPSCREEN_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreen",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UICppScreen as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UICPPSCREEN_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UICppScreen {
    fn type_info(&self) -> &'static TypeInfo {
        UICPPSCREEN_TYPE_INFO
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


pub static UICPPSCREEN_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UICppScreen-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UICppScreen"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UISystemPostInitCompleteMessage {
}

pub trait UISystemPostInitCompleteMessageTrait: TypeObject {
}

impl UISystemPostInitCompleteMessageTrait for UISystemPostInitCompleteMessage {
}

pub static UISYSTEMPOSTINITCOMPLETEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystemPostInitCompleteMessage",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISystemPostInitCompleteMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UISystemPostInitCompleteMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UISYSTEMPOSTINITCOMPLETEMESSAGE_TYPE_INFO
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

#[derive(Clone, Debug, Default)]
pub struct PlayVideoEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub movie: Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>,
    pub network_streaming_url: String,
    pub r#loop: bool,
    pub keep_black_screen: bool,
    pub allow_skip: bool,
    pub draw_in_widget: bool,
    pub render_world: bool,
    pub use_sim_time: bool,
    pub video_identifier: String,
}

pub trait PlayVideoEntityDataTrait: super::entity::EntityDataTrait {
    fn movie(&self) -> &Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>;
    fn movie_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>;
    fn network_streaming_url(&self) -> &String;
    fn network_streaming_url_mut(&mut self) -> &mut String;
    fn r#loop(&self) -> &bool;
    fn r#loop_mut(&mut self) -> &mut bool;
    fn keep_black_screen(&self) -> &bool;
    fn keep_black_screen_mut(&mut self) -> &mut bool;
    fn allow_skip(&self) -> &bool;
    fn allow_skip_mut(&mut self) -> &mut bool;
    fn draw_in_widget(&self) -> &bool;
    fn draw_in_widget_mut(&mut self) -> &mut bool;
    fn render_world(&self) -> &bool;
    fn render_world_mut(&mut self) -> &mut bool;
    fn use_sim_time(&self) -> &bool;
    fn use_sim_time_mut(&mut self) -> &mut bool;
    fn video_identifier(&self) -> &String;
    fn video_identifier_mut(&mut self) -> &mut String;
}

impl PlayVideoEntityDataTrait for PlayVideoEntityData {
    fn movie(&self) -> &Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>> {
        &self.movie
    }
    fn movie_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>> {
        &mut self.movie
    }
    fn network_streaming_url(&self) -> &String {
        &self.network_streaming_url
    }
    fn network_streaming_url_mut(&mut self) -> &mut String {
        &mut self.network_streaming_url
    }
    fn r#loop(&self) -> &bool {
        &self.r#loop
    }
    fn r#loop_mut(&mut self) -> &mut bool {
        &mut self.r#loop
    }
    fn keep_black_screen(&self) -> &bool {
        &self.keep_black_screen
    }
    fn keep_black_screen_mut(&mut self) -> &mut bool {
        &mut self.keep_black_screen
    }
    fn allow_skip(&self) -> &bool {
        &self.allow_skip
    }
    fn allow_skip_mut(&mut self) -> &mut bool {
        &mut self.allow_skip
    }
    fn draw_in_widget(&self) -> &bool {
        &self.draw_in_widget
    }
    fn draw_in_widget_mut(&mut self) -> &mut bool {
        &mut self.draw_in_widget
    }
    fn render_world(&self) -> &bool {
        &self.render_world
    }
    fn render_world_mut(&mut self) -> &mut bool {
        &mut self.render_world
    }
    fn use_sim_time(&self) -> &bool {
        &self.use_sim_time
    }
    fn use_sim_time_mut(&mut self) -> &mut bool {
        &mut self.use_sim_time
    }
    fn video_identifier(&self) -> &String {
        &self.video_identifier
    }
    fn video_identifier_mut(&mut self) -> &mut String {
        &mut self.video_identifier
    }
}

impl super::entity::EntityDataTrait for PlayVideoEntityData {
}

impl super::entity::GameObjectDataTrait for PlayVideoEntityData {
}

impl super::core::DataBusPeerTrait for PlayVideoEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PlayVideoEntityData {
}

impl super::core::DataContainerTrait for PlayVideoEntityData {
}

pub static PLAYVIDEOENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayVideoEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PlayVideoEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(PlayVideoEntityData, movie),
            },
            FieldInfoData {
                name: "NetworkStreamingUrl",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PlayVideoEntityData, network_streaming_url),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayVideoEntityData, r#loop),
            },
            FieldInfoData {
                name: "KeepBlackScreen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayVideoEntityData, keep_black_screen),
            },
            FieldInfoData {
                name: "AllowSkip",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayVideoEntityData, allow_skip),
            },
            FieldInfoData {
                name: "DrawInWidget",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayVideoEntityData, draw_in_widget),
            },
            FieldInfoData {
                name: "RenderWorld",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayVideoEntityData, render_world),
            },
            FieldInfoData {
                name: "UseSimTime",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PlayVideoEntityData, use_sim_time),
            },
            FieldInfoData {
                name: "VideoIdentifier",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(PlayVideoEntityData, video_identifier),
            },
        ],
    }),
    array_type: Some(PLAYVIDEOENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for PlayVideoEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PLAYVIDEOENTITYDATA_TYPE_INFO
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


pub static PLAYVIDEOENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PlayVideoEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("PlayVideoEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MovieDebugEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub language_override: super::core::LanguageFormat,
    pub stop_frame: i32,
}

pub trait MovieDebugEntityDataTrait: super::entity::EntityDataTrait {
    fn language_override(&self) -> &super::core::LanguageFormat;
    fn language_override_mut(&mut self) -> &mut super::core::LanguageFormat;
    fn stop_frame(&self) -> &i32;
    fn stop_frame_mut(&mut self) -> &mut i32;
}

impl MovieDebugEntityDataTrait for MovieDebugEntityData {
    fn language_override(&self) -> &super::core::LanguageFormat {
        &self.language_override
    }
    fn language_override_mut(&mut self) -> &mut super::core::LanguageFormat {
        &mut self.language_override
    }
    fn stop_frame(&self) -> &i32 {
        &self.stop_frame
    }
    fn stop_frame_mut(&mut self) -> &mut i32 {
        &mut self.stop_frame
    }
}

impl super::entity::EntityDataTrait for MovieDebugEntityData {
}

impl super::entity::GameObjectDataTrait for MovieDebugEntityData {
}

impl super::core::DataBusPeerTrait for MovieDebugEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MovieDebugEntityData {
}

impl super::core::DataContainerTrait for MovieDebugEntityData {
}

pub static MOVIEDEBUGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDebugEntityData",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MovieDebugEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LanguageOverride",
                flags: MemberInfoFlags::new(0),
                field_type: "LanguageFormat",
                rust_offset: offset_of!(MovieDebugEntityData, language_override),
            },
            FieldInfoData {
                name: "StopFrame",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(MovieDebugEntityData, stop_frame),
            },
        ],
    }),
    array_type: Some(MOVIEDEBUGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MovieDebugEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MOVIEDEBUGENTITYDATA_TYPE_INFO
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


pub static MOVIEDEBUGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MovieDebugEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("MovieDebugEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIHitZoneCommandDynamicState {
    pub rect: super::core::Vec4,
    pub transform: super::core::Mat4,
    pub view_projection: super::core::Mat4,
    pub field_flag_changed0: u8,
}

pub trait UIHitZoneCommandDynamicStateTrait: TypeObject {
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn transform(&self) -> &super::core::Mat4;
    fn transform_mut(&mut self) -> &mut super::core::Mat4;
    fn view_projection(&self) -> &super::core::Mat4;
    fn view_projection_mut(&mut self) -> &mut super::core::Mat4;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIHitZoneCommandDynamicStateTrait for UIHitZoneCommandDynamicState {
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn transform(&self) -> &super::core::Mat4 {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::Mat4 {
        &mut self.transform
    }
    fn view_projection(&self) -> &super::core::Mat4 {
        &self.view_projection
    }
    fn view_projection_mut(&mut self) -> &mut super::core::Mat4 {
        &mut self.view_projection
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIHITZONECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIHitZoneCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "Mat4",
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, transform),
            },
            FieldInfoData {
                name: "ViewProjection",
                flags: MemberInfoFlags::new(0),
                field_type: "Mat4",
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, view_projection),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIHitZoneCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIHITZONECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIHitZoneCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIHITZONECOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIHITZONECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIHitZoneCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIHitZoneCommandStaticState {
    pub measure_handle: super::game_base::UIImReverseHandle,
    pub field_flag_changed0: u8,
}

pub trait UIHitZoneCommandStaticStateTrait: TypeObject {
    fn measure_handle(&self) -> &super::game_base::UIImReverseHandle;
    fn measure_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIHitZoneCommandStaticStateTrait for UIHitZoneCommandStaticState {
    fn measure_handle(&self) -> &super::game_base::UIImReverseHandle {
        &self.measure_handle
    }
    fn measure_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle {
        &mut self.measure_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIHITZONECOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIHitZoneCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MeasureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImReverseHandle",
                rust_offset: offset_of!(UIHitZoneCommandStaticState, measure_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIHitZoneCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIHITZONECOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIHitZoneCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIHITZONECOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIHITZONECOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIHitZoneCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIHitZoneCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIScreenProjectionCommandDynamicState {
    pub input: super::core::Vec2,
    pub plane: super::core::Vec4,
    pub is_input_normalized: bool,
    pub field_flag_changed0: u8,
}

pub trait UIScreenProjectionCommandDynamicStateTrait: TypeObject {
    fn input(&self) -> &super::core::Vec2;
    fn input_mut(&mut self) -> &mut super::core::Vec2;
    fn plane(&self) -> &super::core::Vec4;
    fn plane_mut(&mut self) -> &mut super::core::Vec4;
    fn is_input_normalized(&self) -> &bool;
    fn is_input_normalized_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIScreenProjectionCommandDynamicStateTrait for UIScreenProjectionCommandDynamicState {
    fn input(&self) -> &super::core::Vec2 {
        &self.input
    }
    fn input_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.input
    }
    fn plane(&self) -> &super::core::Vec4 {
        &self.plane
    }
    fn plane_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.plane
    }
    fn is_input_normalized(&self) -> &bool {
        &self.is_input_normalized
    }
    fn is_input_normalized_mut(&mut self) -> &mut bool {
        &mut self.is_input_normalized
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UISCREENPROJECTIONCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenProjectionCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Input",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, input),
            },
            FieldInfoData {
                name: "Plane",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, plane),
            },
            FieldInfoData {
                name: "IsInputNormalized",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, is_input_normalized),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIScreenProjectionCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UISCREENPROJECTIONCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIScreenProjectionCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENPROJECTIONCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UISCREENPROJECTIONCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIScreenProjectionCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIScreenProjectionCommandStaticState {
    pub reverse_handle: super::game_base::UIImReverseHandle,
    pub field_flag_changed0: u8,
}

pub trait UIScreenProjectionCommandStaticStateTrait: TypeObject {
    fn reverse_handle(&self) -> &super::game_base::UIImReverseHandle;
    fn reverse_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIScreenProjectionCommandStaticStateTrait for UIScreenProjectionCommandStaticState {
    fn reverse_handle(&self) -> &super::game_base::UIImReverseHandle {
        &self.reverse_handle
    }
    fn reverse_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle {
        &mut self.reverse_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UISCREENPROJECTIONCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIScreenProjectionCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReverseHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImReverseHandle",
                rust_offset: offset_of!(UIScreenProjectionCommandStaticState, reverse_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIScreenProjectionCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UISCREENPROJECTIONCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIScreenProjectionCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UISCREENPROJECTIONCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UISCREENPROJECTIONCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIScreenProjectionCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIScreenProjectionCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIInputCommandStaticState {
    pub command_type: UIInputCommandType,
    pub reverse_handle: super::game_base::UIImReverseHandle,
    pub z_depth: f32,
    pub input_action: i32,
    pub field_flag_changed0: u8,
}

pub trait UIInputCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIInputCommandType;
    fn command_type_mut(&mut self) -> &mut UIInputCommandType;
    fn reverse_handle(&self) -> &super::game_base::UIImReverseHandle;
    fn reverse_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle;
    fn z_depth(&self) -> &f32;
    fn z_depth_mut(&mut self) -> &mut f32;
    fn input_action(&self) -> &i32;
    fn input_action_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIInputCommandStaticStateTrait for UIInputCommandStaticState {
    fn command_type(&self) -> &UIInputCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIInputCommandType {
        &mut self.command_type
    }
    fn reverse_handle(&self) -> &super::game_base::UIImReverseHandle {
        &self.reverse_handle
    }
    fn reverse_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle {
        &mut self.reverse_handle
    }
    fn z_depth(&self) -> &f32 {
        &self.z_depth
    }
    fn z_depth_mut(&mut self) -> &mut f32 {
        &mut self.z_depth
    }
    fn input_action(&self) -> &i32 {
        &self.input_action
    }
    fn input_action_mut(&mut self) -> &mut i32 {
        &mut self.input_action
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIINPUTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIInputCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIInputCommandType",
                rust_offset: offset_of!(UIInputCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "ReverseHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImReverseHandle",
                rust_offset: offset_of!(UIInputCommandStaticState, reverse_handle),
            },
            FieldInfoData {
                name: "ZDepth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIInputCommandStaticState, z_depth),
            },
            FieldInfoData {
                name: "InputAction",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIInputCommandStaticState, input_action),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIInputCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIInputCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIInputCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
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

pub static UIINPUTCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIINPUTCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIInputCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIINPUTCOMMANDTYPE_TYPE_INFO
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


pub static UIINPUTCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIInputCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIInputCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMouseInputCommandDynamicState {
    pub tag: String,
    pub listen_rect: super::core::Vec4,
    pub plane: super::core::Vec4,
    pub screen_local_transform: super::core::LinearTransform,
    pub mouse_input_type_mask: i32,
    pub field_flag_changed0: u8,
}

pub trait UIMouseInputCommandDynamicStateTrait: TypeObject {
    fn tag(&self) -> &String;
    fn tag_mut(&mut self) -> &mut String;
    fn listen_rect(&self) -> &super::core::Vec4;
    fn listen_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn plane(&self) -> &super::core::Vec4;
    fn plane_mut(&mut self) -> &mut super::core::Vec4;
    fn screen_local_transform(&self) -> &super::core::LinearTransform;
    fn screen_local_transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn mouse_input_type_mask(&self) -> &i32;
    fn mouse_input_type_mask_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIMouseInputCommandDynamicStateTrait for UIMouseInputCommandDynamicState {
    fn tag(&self) -> &String {
        &self.tag
    }
    fn tag_mut(&mut self) -> &mut String {
        &mut self.tag
    }
    fn listen_rect(&self) -> &super::core::Vec4 {
        &self.listen_rect
    }
    fn listen_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.listen_rect
    }
    fn plane(&self) -> &super::core::Vec4 {
        &self.plane
    }
    fn plane_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.plane
    }
    fn screen_local_transform(&self) -> &super::core::LinearTransform {
        &self.screen_local_transform
    }
    fn screen_local_transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.screen_local_transform
    }
    fn mouse_input_type_mask(&self) -> &i32 {
        &self.mouse_input_type_mask
    }
    fn mouse_input_type_mask_mut(&mut self) -> &mut i32 {
        &mut self.mouse_input_type_mask
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIMOUSEINPUTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMouseInputCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Tag",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, tag),
            },
            FieldInfoData {
                name: "ListenRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, listen_rect),
            },
            FieldInfoData {
                name: "Plane",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, plane),
            },
            FieldInfoData {
                name: "ScreenLocalTransform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, screen_local_transform),
            },
            FieldInfoData {
                name: "MouseInputTypeMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, mouse_input_type_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIMouseInputCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOUSEINPUTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIMouseInputCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIMOUSEINPUTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIMOUSEINPUTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMouseInputCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMouseInputCommandStaticState {
    pub reverse_handle: super::game_base::UIImReverseHandle,
    pub field_flag_changed0: u8,
}

pub trait UIMouseInputCommandStaticStateTrait: TypeObject {
    fn reverse_handle(&self) -> &super::game_base::UIImReverseHandle;
    fn reverse_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIMouseInputCommandStaticStateTrait for UIMouseInputCommandStaticState {
    fn reverse_handle(&self) -> &super::game_base::UIImReverseHandle {
        &self.reverse_handle
    }
    fn reverse_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle {
        &mut self.reverse_handle
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIMOUSEINPUTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMouseInputCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ReverseHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImReverseHandle",
                rust_offset: offset_of!(UIMouseInputCommandStaticState, reverse_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIMouseInputCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOUSEINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIMouseInputCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIMOUSEINPUTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIMOUSEINPUTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMouseInputCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMouseInputCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawSectionNameCommandDynamicState {
    pub name: String,
    pub field_flag_changed0: u8,
}

pub trait UIDrawSectionNameCommandDynamicStateTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawSectionNameCommandDynamicStateTrait for UIDrawSectionNameCommandDynamicState {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSectionNameCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawSectionNameCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIDrawSectionNameCommandDynamicState, name),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawSectionNameCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawSectionNameCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWSECTIONNAMECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSectionNameCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawSectionNameCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawClearCommandDynamicState {
    pub color: super::core::Vec4,
    pub stencil: u8,
    pub clear_mask: u32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawClearCommandDynamicStateTrait: TypeObject {
    fn color(&self) -> &super::core::Vec4;
    fn color_mut(&mut self) -> &mut super::core::Vec4;
    fn stencil(&self) -> &u8;
    fn stencil_mut(&mut self) -> &mut u8;
    fn clear_mask(&self) -> &u32;
    fn clear_mask_mut(&mut self) -> &mut u32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawClearCommandDynamicStateTrait for UIDrawClearCommandDynamicState {
    fn color(&self) -> &super::core::Vec4 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.color
    }
    fn stencil(&self) -> &u8 {
        &self.stencil
    }
    fn stencil_mut(&mut self) -> &mut u8 {
        &mut self.stencil
    }
    fn clear_mask(&self) -> &u32 {
        &self.clear_mask
    }
    fn clear_mask_mut(&mut self) -> &mut u32 {
        &mut self.clear_mask
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWCLEARCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawClearCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawClearCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, color),
            },
            FieldInfoData {
                name: "Stencil",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, stencil),
            },
            FieldInfoData {
                name: "ClearMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, clear_mask),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawClearCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWCLEARCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawClearCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWCLEARCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWCLEARCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawClearCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawClearCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawViewportCommandDynamicState {
    pub viewport: super::render_base::ViewportRect,
    pub field_flag_changed0: u8,
}

pub trait UIDrawViewportCommandDynamicStateTrait: TypeObject {
    fn viewport(&self) -> &super::render_base::ViewportRect;
    fn viewport_mut(&mut self) -> &mut super::render_base::ViewportRect;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawViewportCommandDynamicStateTrait for UIDrawViewportCommandDynamicState {
    fn viewport(&self) -> &super::render_base::ViewportRect {
        &self.viewport
    }
    fn viewport_mut(&mut self) -> &mut super::render_base::ViewportRect {
        &mut self.viewport
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawViewportCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Viewport",
                flags: MemberInfoFlags::new(0),
                field_type: "ViewportRect",
                rust_offset: offset_of!(UIDrawViewportCommandDynamicState, viewport),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawViewportCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawViewportCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWVIEWPORTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawViewportCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawViewportCommandStaticState {
    pub command_type: UIDrawViewportCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawViewportCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawViewportCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawViewportCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawViewportCommandStaticStateTrait for UIDrawViewportCommandStaticState {
    fn command_type(&self) -> &UIDrawViewportCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawViewportCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWVIEWPORTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawViewportCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawViewportCommandType",
                rust_offset: offset_of!(UIDrawViewportCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawViewportCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWVIEWPORTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawViewportCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWVIEWPORTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWVIEWPORTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawViewportCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawViewportCommandType {
    #[default]
    UIDrawViewportCommandType_Push = 0,
    UIDrawViewportCommandType_Pop = 1,
}

pub static UIDRAWVIEWPORTCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWVIEWPORTCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawViewportCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWVIEWPORTCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWVIEWPORTCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawViewportCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawViewportCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait UIDrawBlendCommandDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn separate_alpha_blend(&self) -> &bool;
    fn separate_alpha_blend_mut(&mut self) -> &mut bool;
    fn color_op(&self) -> &super::render::RenderBlendOp;
    fn color_op_mut(&mut self) -> &mut super::render::RenderBlendOp;
    fn alpha_op(&self) -> &super::render::RenderBlendOp;
    fn alpha_op_mut(&mut self) -> &mut super::render::RenderBlendOp;
    fn source_color(&self) -> &super::render::RenderBlendMode;
    fn source_color_mut(&mut self) -> &mut super::render::RenderBlendMode;
    fn dest_color(&self) -> &super::render::RenderBlendMode;
    fn dest_color_mut(&mut self) -> &mut super::render::RenderBlendMode;
    fn source_alpha(&self) -> &super::render::RenderBlendMode;
    fn source_alpha_mut(&mut self) -> &mut super::render::RenderBlendMode;
    fn dest_alpha(&self) -> &super::render::RenderBlendMode;
    fn dest_alpha_mut(&mut self) -> &mut super::render::RenderBlendMode;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawBlendCommandDynamicStateTrait for UIDrawBlendCommandDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn separate_alpha_blend(&self) -> &bool {
        &self.separate_alpha_blend
    }
    fn separate_alpha_blend_mut(&mut self) -> &mut bool {
        &mut self.separate_alpha_blend
    }
    fn color_op(&self) -> &super::render::RenderBlendOp {
        &self.color_op
    }
    fn color_op_mut(&mut self) -> &mut super::render::RenderBlendOp {
        &mut self.color_op
    }
    fn alpha_op(&self) -> &super::render::RenderBlendOp {
        &self.alpha_op
    }
    fn alpha_op_mut(&mut self) -> &mut super::render::RenderBlendOp {
        &mut self.alpha_op
    }
    fn source_color(&self) -> &super::render::RenderBlendMode {
        &self.source_color
    }
    fn source_color_mut(&mut self) -> &mut super::render::RenderBlendMode {
        &mut self.source_color
    }
    fn dest_color(&self) -> &super::render::RenderBlendMode {
        &self.dest_color
    }
    fn dest_color_mut(&mut self) -> &mut super::render::RenderBlendMode {
        &mut self.dest_color
    }
    fn source_alpha(&self) -> &super::render::RenderBlendMode {
        &self.source_alpha
    }
    fn source_alpha_mut(&mut self) -> &mut super::render::RenderBlendMode {
        &mut self.source_alpha
    }
    fn dest_alpha(&self) -> &super::render::RenderBlendMode {
        &self.dest_alpha
    }
    fn dest_alpha_mut(&mut self) -> &mut super::render::RenderBlendMode {
        &mut self.dest_alpha
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWBLENDCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawBlendCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, enabled),
            },
            FieldInfoData {
                name: "SeparateAlphaBlend",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, separate_alpha_blend),
            },
            FieldInfoData {
                name: "ColorOp",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBlendOp",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, color_op),
            },
            FieldInfoData {
                name: "AlphaOp",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBlendOp",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, alpha_op),
            },
            FieldInfoData {
                name: "SourceColor",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBlendMode",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, source_color),
            },
            FieldInfoData {
                name: "DestColor",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBlendMode",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, dest_color),
            },
            FieldInfoData {
                name: "SourceAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBlendMode",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, source_alpha),
            },
            FieldInfoData {
                name: "DestAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderBlendMode",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, dest_alpha),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawBlendCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWBLENDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawBlendCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWBLENDCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWBLENDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawBlendCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawBlendCommandStaticState {
    pub command_type: UIDrawBlendCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawBlendCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawBlendCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawBlendCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawBlendCommandStaticStateTrait for UIDrawBlendCommandStaticState {
    fn command_type(&self) -> &UIDrawBlendCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawBlendCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWBLENDCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawBlendCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawBlendCommandType",
                rust_offset: offset_of!(UIDrawBlendCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawBlendCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWBLENDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawBlendCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWBLENDCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWBLENDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawBlendCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawBlendCommandType {
    #[default]
    UIDrawBlendCommandType_Push = 0,
    UIDrawBlendCommandType_Pop = 1,
}

pub static UIDRAWBLENDCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWBLENDCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawBlendCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWBLENDCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWBLENDCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawBlendCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawBlendCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawDepthCommandDynamicState {
    pub mode: super::render::RenderDepthMode,
    pub field_flag_changed0: u8,
}

pub trait UIDrawDepthCommandDynamicStateTrait: TypeObject {
    fn mode(&self) -> &super::render::RenderDepthMode;
    fn mode_mut(&mut self) -> &mut super::render::RenderDepthMode;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawDepthCommandDynamicStateTrait for UIDrawDepthCommandDynamicState {
    fn mode(&self) -> &super::render::RenderDepthMode {
        &self.mode
    }
    fn mode_mut(&mut self) -> &mut super::render::RenderDepthMode {
        &mut self.mode
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWDEPTHCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawDepthCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Mode",
                flags: MemberInfoFlags::new(0),
                field_type: "RenderDepthMode",
                rust_offset: offset_of!(UIDrawDepthCommandDynamicState, mode),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawDepthCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDEPTHCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawDepthCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWDEPTHCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWDEPTHCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDepthCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawDepthCommandStaticState {
    pub command_type: UIDrawDepthCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawDepthCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawDepthCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawDepthCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawDepthCommandStaticStateTrait for UIDrawDepthCommandStaticState {
    fn command_type(&self) -> &UIDrawDepthCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawDepthCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWDEPTHCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawDepthCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawDepthCommandType",
                rust_offset: offset_of!(UIDrawDepthCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawDepthCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDEPTHCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawDepthCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWDEPTHCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWDEPTHCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDepthCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawDepthCommandType {
    #[default]
    UIDrawDepthCommandType_Push = 0,
    UIDrawDepthCommandType_Pop = 1,
}

pub static UIDRAWDEPTHCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWDEPTHCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawDepthCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWDEPTHCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWDEPTHCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDepthCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDepthCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawStencilMaskCommandDynamicState {
    pub invert_test: bool,
    pub rect: super::core::Vec4,
    pub texture: super::render_base::TextureResourceHandle,
    pub uv_rect: super::core::Vec4,
    pub alpha_threshold: f32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawStencilMaskCommandDynamicStateTrait: TypeObject {
    fn invert_test(&self) -> &bool;
    fn invert_test_mut(&mut self) -> &mut bool;
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn texture(&self) -> &super::render_base::TextureResourceHandle;
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn alpha_threshold(&self) -> &f32;
    fn alpha_threshold_mut(&mut self) -> &mut f32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawStencilMaskCommandDynamicStateTrait for UIDrawStencilMaskCommandDynamicState {
    fn invert_test(&self) -> &bool {
        &self.invert_test
    }
    fn invert_test_mut(&mut self) -> &mut bool {
        &mut self.invert_test
    }
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn texture(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture
    }
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
    fn alpha_threshold(&self) -> &f32 {
        &self.alpha_threshold
    }
    fn alpha_threshold_mut(&mut self) -> &mut f32 {
        &mut self.alpha_threshold
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawStencilMaskCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InvertTest",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, invert_test),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, uv_rect),
            },
            FieldInfoData {
                name: "AlphaThreshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, alpha_threshold),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawStencilMaskCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawStencilMaskCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWSTENCILMASKCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilMaskCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawStencilMaskCommandStaticState {
    pub command_type: UIDrawStencilMaskCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawStencilMaskCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawStencilMaskCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawStencilMaskCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawStencilMaskCommandStaticStateTrait for UIDrawStencilMaskCommandStaticState {
    fn command_type(&self) -> &UIDrawStencilMaskCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawStencilMaskCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSTENCILMASKCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawStencilMaskCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawStencilMaskCommandType",
                rust_offset: offset_of!(UIDrawStencilMaskCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawStencilMaskCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILMASKCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawStencilMaskCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSTENCILMASKCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWSTENCILMASKCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilMaskCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawStencilMaskCommandType {
    #[default]
    UIDrawStencilMaskCommandType_Push = 0,
    UIDrawStencilMaskCommandType_Pop = 1,
}

pub static UIDRAWSTENCILMASKCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWSTENCILMASKCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawStencilMaskCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSTENCILMASKCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWSTENCILMASKCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilMaskCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilMaskCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait UIDrawStencilCommandDynamicStateTrait: TypeObject {
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn write_color(&self) -> &bool;
    fn write_color_mut(&mut self) -> &mut bool;
    fn r#ref(&self) -> &u8;
    fn r#ref_mut(&mut self) -> &mut u8;
    fn stencil_mask(&self) -> &u8;
    fn stencil_mask_mut(&mut self) -> &mut u8;
    fn func(&self) -> &super::render::DepthStencilCompareFunc;
    fn func_mut(&mut self) -> &mut super::render::DepthStencilCompareFunc;
    fn fail_op(&self) -> &super::render::StencilOperation;
    fn fail_op_mut(&mut self) -> &mut super::render::StencilOperation;
    fn depth_fail_op(&self) -> &super::render::StencilOperation;
    fn depth_fail_op_mut(&mut self) -> &mut super::render::StencilOperation;
    fn pass_op(&self) -> &super::render::StencilOperation;
    fn pass_op_mut(&mut self) -> &mut super::render::StencilOperation;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawStencilCommandDynamicStateTrait for UIDrawStencilCommandDynamicState {
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn write_color(&self) -> &bool {
        &self.write_color
    }
    fn write_color_mut(&mut self) -> &mut bool {
        &mut self.write_color
    }
    fn r#ref(&self) -> &u8 {
        &self.r#ref
    }
    fn r#ref_mut(&mut self) -> &mut u8 {
        &mut self.r#ref
    }
    fn stencil_mask(&self) -> &u8 {
        &self.stencil_mask
    }
    fn stencil_mask_mut(&mut self) -> &mut u8 {
        &mut self.stencil_mask
    }
    fn func(&self) -> &super::render::DepthStencilCompareFunc {
        &self.func
    }
    fn func_mut(&mut self) -> &mut super::render::DepthStencilCompareFunc {
        &mut self.func
    }
    fn fail_op(&self) -> &super::render::StencilOperation {
        &self.fail_op
    }
    fn fail_op_mut(&mut self) -> &mut super::render::StencilOperation {
        &mut self.fail_op
    }
    fn depth_fail_op(&self) -> &super::render::StencilOperation {
        &self.depth_fail_op
    }
    fn depth_fail_op_mut(&mut self) -> &mut super::render::StencilOperation {
        &mut self.depth_fail_op
    }
    fn pass_op(&self) -> &super::render::StencilOperation {
        &self.pass_op
    }
    fn pass_op_mut(&mut self) -> &mut super::render::StencilOperation {
        &mut self.pass_op
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSTENCILCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawStencilCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, enabled),
            },
            FieldInfoData {
                name: "WriteColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, write_color),
            },
            FieldInfoData {
                name: "Ref",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, r#ref),
            },
            FieldInfoData {
                name: "StencilMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, stencil_mask),
            },
            FieldInfoData {
                name: "Func",
                flags: MemberInfoFlags::new(0),
                field_type: "DepthStencilCompareFunc",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, func),
            },
            FieldInfoData {
                name: "FailOp",
                flags: MemberInfoFlags::new(0),
                field_type: "StencilOperation",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, fail_op),
            },
            FieldInfoData {
                name: "DepthFailOp",
                flags: MemberInfoFlags::new(0),
                field_type: "StencilOperation",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, depth_fail_op),
            },
            FieldInfoData {
                name: "PassOp",
                flags: MemberInfoFlags::new(0),
                field_type: "StencilOperation",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, pass_op),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawStencilCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawStencilCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSTENCILCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWSTENCILCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawStencilCommandStaticState {
    pub command_type: UIDrawStencilCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawStencilCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawStencilCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawStencilCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawStencilCommandStaticStateTrait for UIDrawStencilCommandStaticState {
    fn command_type(&self) -> &UIDrawStencilCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawStencilCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSTENCILCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawStencilCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawStencilCommandType",
                rust_offset: offset_of!(UIDrawStencilCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawStencilCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSTENCILCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawStencilCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSTENCILCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWSTENCILCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawStencilCommandType {
    #[default]
    UIDrawStencilCommandType_Push = 0,
    UIDrawStencilCommandType_Pop = 1,
}

pub static UIDRAWSTENCILCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWSTENCILCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawStencilCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSTENCILCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWSTENCILCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawStencilCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawStencilCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIClipThresholdCommandDynamicState {
    pub threshold: f32,
    pub field_flag_changed0: u8,
}

pub trait UIClipThresholdCommandDynamicStateTrait: TypeObject {
    fn threshold(&self) -> &f32;
    fn threshold_mut(&mut self) -> &mut f32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIClipThresholdCommandDynamicStateTrait for UIClipThresholdCommandDynamicState {
    fn threshold(&self) -> &f32 {
        &self.threshold
    }
    fn threshold_mut(&mut self) -> &mut f32 {
        &mut self.threshold
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIClipThresholdCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIClipThresholdCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Threshold",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIClipThresholdCommandDynamicState, threshold),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIClipThresholdCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIClipThresholdCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UICLIPTHRESHOLDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIClipThresholdCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIClipThresholdCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawTransformCommandDynamicState {
    pub transform: super::core::LinearTransform,
    pub enabled: bool,
    pub field_flag_changed0: u8,
}

pub trait UIDrawTransformCommandDynamicStateTrait: TypeObject {
    fn transform(&self) -> &super::core::LinearTransform;
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform;
    fn enabled(&self) -> &bool;
    fn enabled_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawTransformCommandDynamicStateTrait for UIDrawTransformCommandDynamicState {
    fn transform(&self) -> &super::core::LinearTransform {
        &self.transform
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.transform
    }
    fn enabled(&self) -> &bool {
        &self.enabled
    }
    fn enabled_mut(&mut self) -> &mut bool {
        &mut self.enabled
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawTransformCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Transform",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(UIDrawTransformCommandDynamicState, transform),
            },
            FieldInfoData {
                name: "Enabled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawTransformCommandDynamicState, enabled),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawTransformCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawTransformCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWTRANSFORMCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTransformCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawTransformCommandStaticState {
    pub command_type: UIDrawTransformCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawTransformCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawTransformCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawTransformCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawTransformCommandStaticStateTrait for UIDrawTransformCommandStaticState {
    fn command_type(&self) -> &UIDrawTransformCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawTransformCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWTRANSFORMCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawTransformCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawTransformCommandType",
                rust_offset: offset_of!(UIDrawTransformCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawTransformCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTRANSFORMCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawTransformCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWTRANSFORMCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWTRANSFORMCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTransformCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawTransformCommandType {
    #[default]
    UIDrawTransformCommandType_Push = 0,
    UIDrawTransformCommandType_Replace = 1,
    UIDrawTransformCommandType_Pop = 2,
}

pub static UIDRAWTRANSFORMCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWTRANSFORMCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawTransformCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWTRANSFORMCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWTRANSFORMCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTransformCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTransformCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawColorCommandDynamicState {
    pub color: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub trait UIDrawColorCommandDynamicStateTrait: TypeObject {
    fn color(&self) -> &super::core::Vec4;
    fn color_mut(&mut self) -> &mut super::core::Vec4;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawColorCommandDynamicStateTrait for UIDrawColorCommandDynamicState {
    fn color(&self) -> &super::core::Vec4 {
        &self.color
    }
    fn color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.color
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWCOLORCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawColorCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Color",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawColorCommandDynamicState, color),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawColorCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWCOLORCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawColorCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWCOLORCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWCOLORCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawColorCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawColorCommandStaticState {
    pub command_type: UIDrawColorCommandType,
    pub field_flag_changed0: u8,
}

pub trait UIDrawColorCommandStaticStateTrait: TypeObject {
    fn command_type(&self) -> &UIDrawColorCommandType;
    fn command_type_mut(&mut self) -> &mut UIDrawColorCommandType;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawColorCommandStaticStateTrait for UIDrawColorCommandStaticState {
    fn command_type(&self) -> &UIDrawColorCommandType {
        &self.command_type
    }
    fn command_type_mut(&mut self) -> &mut UIDrawColorCommandType {
        &mut self.command_type
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWCOLORCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawColorCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CommandType",
                flags: MemberInfoFlags::new(0),
                field_type: "UIDrawColorCommandType",
                rust_offset: offset_of!(UIDrawColorCommandStaticState, command_type),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawColorCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWCOLORCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawColorCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWCOLORCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWCOLORCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawColorCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum UIDrawColorCommandType {
    #[default]
    UIDrawColorCommandType_Push = 0,
    UIDrawColorCommandType_Pop = 1,
}

pub static UIDRAWCOLORCOMMANDTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandType",
    flags: MemberInfoFlags::new(49429),
    module: "GameClientUI",
    data: TypeInfoData::Enum,
    array_type: Some(UIDRAWCOLORCOMMANDTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for UIDrawColorCommandType {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWCOLORCOMMANDTYPE_TYPE_INFO
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


pub static UIDRAWCOLORCOMMANDTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawColorCommandType-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawColorCommandType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMovieCommandDynamicState {
    pub rect: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub trait UIMovieCommandDynamicStateTrait: TypeObject {
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIMovieCommandDynamicStateTrait for UIMovieCommandDynamicState {
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIMOVIECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMovieCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIMovieCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIMovieCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOVIECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIMovieCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIMOVIECOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIMOVIECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMovieCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMovieCommandStaticState {
    pub movie: super::movie_base::MovieHandle,
    pub field_flag_changed0: u8,
}

pub trait UIMovieCommandStaticStateTrait: TypeObject {
    fn movie(&self) -> &super::movie_base::MovieHandle;
    fn movie_mut(&mut self) -> &mut super::movie_base::MovieHandle;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIMovieCommandStaticStateTrait for UIMovieCommandStaticState {
    fn movie(&self) -> &super::movie_base::MovieHandle {
        &self.movie
    }
    fn movie_mut(&mut self) -> &mut super::movie_base::MovieHandle {
        &mut self.movie
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIMOVIECOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMovieCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: "MovieHandle",
                rust_offset: offset_of!(UIMovieCommandStaticState, movie),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIMovieCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMOVIECOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 2,
};

impl TypeObject for UIMovieCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIMOVIECOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIMOVIECOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMovieCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMovieCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMeasureTextCommandDynamicState {
    pub text: String,
    pub measure_width: f32,
    pub custom_text_layout_callback: u64,
    pub field_flag_changed0: u8,
}

pub trait UIMeasureTextCommandDynamicStateTrait: TypeObject {
    fn text(&self) -> &String;
    fn text_mut(&mut self) -> &mut String;
    fn measure_width(&self) -> &f32;
    fn measure_width_mut(&mut self) -> &mut f32;
    fn custom_text_layout_callback(&self) -> &u64;
    fn custom_text_layout_callback_mut(&mut self) -> &mut u64;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIMeasureTextCommandDynamicStateTrait for UIMeasureTextCommandDynamicState {
    fn text(&self) -> &String {
        &self.text
    }
    fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
    fn measure_width(&self) -> &f32 {
        &self.measure_width
    }
    fn measure_width_mut(&mut self) -> &mut f32 {
        &mut self.measure_width
    }
    fn custom_text_layout_callback(&self) -> &u64 {
        &self.custom_text_layout_callback
    }
    fn custom_text_layout_callback_mut(&mut self) -> &mut u64 {
        &mut self.custom_text_layout_callback
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIMEASURETEXTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMeasureTextCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, text),
            },
            FieldInfoData {
                name: "MeasureWidth",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, measure_width),
            },
            FieldInfoData {
                name: "CustomTextLayoutCallback",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, custom_text_layout_callback),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIMeasureTextCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMEASURETEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIMeasureTextCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIMEASURETEXTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIMEASURETEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMeasureTextCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIMeasureTextCommandStaticState {
    pub config: super::game_base::UIImTextCommandConfig,
    pub measure_handle: super::game_base::UIImReverseHandle,
    pub measure_only_visible_glyphs: bool,
    pub field_flag_changed0: u8,
}

pub trait UIMeasureTextCommandStaticStateTrait: TypeObject {
    fn config(&self) -> &super::game_base::UIImTextCommandConfig;
    fn config_mut(&mut self) -> &mut super::game_base::UIImTextCommandConfig;
    fn measure_handle(&self) -> &super::game_base::UIImReverseHandle;
    fn measure_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle;
    fn measure_only_visible_glyphs(&self) -> &bool;
    fn measure_only_visible_glyphs_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIMeasureTextCommandStaticStateTrait for UIMeasureTextCommandStaticState {
    fn config(&self) -> &super::game_base::UIImTextCommandConfig {
        &self.config
    }
    fn config_mut(&mut self) -> &mut super::game_base::UIImTextCommandConfig {
        &mut self.config
    }
    fn measure_handle(&self) -> &super::game_base::UIImReverseHandle {
        &self.measure_handle
    }
    fn measure_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle {
        &mut self.measure_handle
    }
    fn measure_only_visible_glyphs(&self) -> &bool {
        &self.measure_only_visible_glyphs
    }
    fn measure_only_visible_glyphs_mut(&mut self) -> &mut bool {
        &mut self.measure_only_visible_glyphs
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIMEASURETEXTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMeasureTextCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Config",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImTextCommandConfig",
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, config),
            },
            FieldInfoData {
                name: "MeasureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImReverseHandle",
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, measure_handle),
            },
            FieldInfoData {
                name: "MeasureOnlyVisibleGlyphs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, measure_only_visible_glyphs),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIMeasureTextCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIMEASURETEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIMeasureTextCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIMEASURETEXTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIMEASURETEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMeasureTextCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIMeasureTextCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawTextCommandDynamicState {
    pub text: String,
    pub rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub custom_text_layout_callback: u64,
    pub field_flag_changed0: u8,
}

pub trait UIDrawTextCommandDynamicStateTrait: TypeObject {
    fn text(&self) -> &String;
    fn text_mut(&mut self) -> &mut String;
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn non_premultiplied_color(&self) -> &super::core::Vec4;
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4;
    fn custom_text_layout_callback(&self) -> &u64;
    fn custom_text_layout_callback_mut(&mut self) -> &mut u64;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawTextCommandDynamicStateTrait for UIDrawTextCommandDynamicState {
    fn text(&self) -> &String {
        &self.text
    }
    fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn non_premultiplied_color(&self) -> &super::core::Vec4 {
        &self.non_premultiplied_color
    }
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.non_premultiplied_color
    }
    fn custom_text_layout_callback(&self) -> &u64 {
        &self.custom_text_layout_callback
    }
    fn custom_text_layout_callback_mut(&mut self) -> &mut u64 {
        &mut self.custom_text_layout_callback
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWTEXTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawTextCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, text),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "CustomTextLayoutCallback",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint64",
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, custom_text_layout_callback),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawTextCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawTextCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWTEXTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWTEXTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTextCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawTextCommandStaticState {
    pub config: super::game_base::UIImTextCommandConfig,
    pub shader_program: i32,
    pub measure_handle: super::game_base::UIImReverseHandle,
    pub measure_only_visible_glyphs: bool,
    pub field_flag_changed0: u8,
}

pub trait UIDrawTextCommandStaticStateTrait: TypeObject {
    fn config(&self) -> &super::game_base::UIImTextCommandConfig;
    fn config_mut(&mut self) -> &mut super::game_base::UIImTextCommandConfig;
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn measure_handle(&self) -> &super::game_base::UIImReverseHandle;
    fn measure_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle;
    fn measure_only_visible_glyphs(&self) -> &bool;
    fn measure_only_visible_glyphs_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawTextCommandStaticStateTrait for UIDrawTextCommandStaticState {
    fn config(&self) -> &super::game_base::UIImTextCommandConfig {
        &self.config
    }
    fn config_mut(&mut self) -> &mut super::game_base::UIImTextCommandConfig {
        &mut self.config
    }
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn measure_handle(&self) -> &super::game_base::UIImReverseHandle {
        &self.measure_handle
    }
    fn measure_handle_mut(&mut self) -> &mut super::game_base::UIImReverseHandle {
        &mut self.measure_handle
    }
    fn measure_only_visible_glyphs(&self) -> &bool {
        &self.measure_only_visible_glyphs
    }
    fn measure_only_visible_glyphs_mut(&mut self) -> &mut bool {
        &mut self.measure_only_visible_glyphs
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWTEXTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawTextCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Config",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImTextCommandConfig",
                rust_offset: offset_of!(UIDrawTextCommandStaticState, config),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawTextCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "MeasureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIImReverseHandle",
                rust_offset: offset_of!(UIDrawTextCommandStaticState, measure_handle),
            },
            FieldInfoData {
                name: "MeasureOnlyVisibleGlyphs",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawTextCommandStaticState, measure_only_visible_glyphs),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawTextCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawTextCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWTEXTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWTEXTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTextCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTextCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawAACircleCommandDynamicState {
    pub center: super::core::Vec2,
    pub radius: f32,
    pub thickness: f32,
    pub non_premultiplied_color: super::core::Vec4,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawAACircleCommandDynamicStateTrait: TypeObject {
    fn center(&self) -> &super::core::Vec2;
    fn center_mut(&mut self) -> &mut super::core::Vec2;
    fn radius(&self) -> &f32;
    fn radius_mut(&mut self) -> &mut f32;
    fn thickness(&self) -> &f32;
    fn thickness_mut(&mut self) -> &mut f32;
    fn non_premultiplied_color(&self) -> &super::core::Vec4;
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4;
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawAACircleCommandDynamicStateTrait for UIDrawAACircleCommandDynamicState {
    fn center(&self) -> &super::core::Vec2 {
        &self.center
    }
    fn center_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.center
    }
    fn radius(&self) -> &f32 {
        &self.radius
    }
    fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }
    fn thickness(&self) -> &f32 {
        &self.thickness
    }
    fn thickness_mut(&mut self) -> &mut f32 {
        &mut self.thickness
    }
    fn non_premultiplied_color(&self) -> &super::core::Vec4 {
        &self.non_premultiplied_color
    }
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.non_premultiplied_color
    }
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWAACIRCLECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAACircleCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawAACircleCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Center",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, center),
            },
            FieldInfoData {
                name: "Radius",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, radius),
            },
            FieldInfoData {
                name: "Thickness",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, thickness),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawAACircleCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWAACIRCLECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawAACircleCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWAACIRCLECOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWAACIRCLECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAACircleCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAACircleCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawAALineStripCommandDynamicState {
    pub vertices: Vec<super::core::Vec2>,
    pub non_premultiplied_colors: Vec<super::core::Vec4>,
    pub width: f32,
    pub closed: bool,
    pub fill_left: bool,
    pub fill_right: bool,
    pub field_flag_changed0: u8,
}

pub trait UIDrawAALineStripCommandDynamicStateTrait: TypeObject {
    fn vertices(&self) -> &Vec<super::core::Vec2>;
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec2>;
    fn non_premultiplied_colors(&self) -> &Vec<super::core::Vec4>;
    fn non_premultiplied_colors_mut(&mut self) -> &mut Vec<super::core::Vec4>;
    fn width(&self) -> &f32;
    fn width_mut(&mut self) -> &mut f32;
    fn closed(&self) -> &bool;
    fn closed_mut(&mut self) -> &mut bool;
    fn fill_left(&self) -> &bool;
    fn fill_left_mut(&mut self) -> &mut bool;
    fn fill_right(&self) -> &bool;
    fn fill_right_mut(&mut self) -> &mut bool;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawAALineStripCommandDynamicStateTrait for UIDrawAALineStripCommandDynamicState {
    fn vertices(&self) -> &Vec<super::core::Vec2> {
        &self.vertices
    }
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec2> {
        &mut self.vertices
    }
    fn non_premultiplied_colors(&self) -> &Vec<super::core::Vec4> {
        &self.non_premultiplied_colors
    }
    fn non_premultiplied_colors_mut(&mut self) -> &mut Vec<super::core::Vec4> {
        &mut self.non_premultiplied_colors
    }
    fn width(&self) -> &f32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }
    fn closed(&self) -> &bool {
        &self.closed
    }
    fn closed_mut(&mut self) -> &mut bool {
        &mut self.closed
    }
    fn fill_left(&self) -> &bool {
        &self.fill_left
    }
    fn fill_left_mut(&mut self) -> &mut bool {
        &mut self.fill_left
    }
    fn fill_right(&self) -> &bool {
        &self.fill_right
    }
    fn fill_right_mut(&mut self) -> &mut bool {
        &mut self.fill_right
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAALineStripCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawAALineStripCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "NonPremultipliedColors",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, non_premultiplied_colors),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, width),
            },
            FieldInfoData {
                name: "Closed",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, closed),
            },
            FieldInfoData {
                name: "FillLeft",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, fill_left),
            },
            FieldInfoData {
                name: "FillRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, fill_right),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawAALineStripCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawAALineStripCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWAALINESTRIPCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAALineStripCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAALineStripCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawLineListCommandDynamicState {
    pub vertices: Vec<super::core::Vec3>,
    pub non_premultiplied_colors: Vec<super::core::Vec4>,
    pub indices: Vec<u16>,
    pub field_flag_changed0: u8,
}

pub trait UIDrawLineListCommandDynamicStateTrait: TypeObject {
    fn vertices(&self) -> &Vec<super::core::Vec3>;
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec3>;
    fn non_premultiplied_colors(&self) -> &Vec<super::core::Vec4>;
    fn non_premultiplied_colors_mut(&mut self) -> &mut Vec<super::core::Vec4>;
    fn indices(&self) -> &Vec<u16>;
    fn indices_mut(&mut self) -> &mut Vec<u16>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawLineListCommandDynamicStateTrait for UIDrawLineListCommandDynamicState {
    fn vertices(&self) -> &Vec<super::core::Vec3> {
        &self.vertices
    }
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec3> {
        &mut self.vertices
    }
    fn non_premultiplied_colors(&self) -> &Vec<super::core::Vec4> {
        &self.non_premultiplied_colors
    }
    fn non_premultiplied_colors_mut(&mut self) -> &mut Vec<super::core::Vec4> {
        &mut self.non_premultiplied_colors
    }
    fn indices(&self) -> &Vec<u16> {
        &self.indices
    }
    fn indices_mut(&mut self) -> &mut Vec<u16> {
        &mut self.indices
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWLINELISTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawLineListCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "NonPremultipliedColors",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, non_premultiplied_colors),
            },
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, indices),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawLineListCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWLINELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawLineListCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWLINELISTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWLINELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawLineListCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawLineListCommandStaticState {
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawLineListCommandStaticStateTrait: TypeObject {
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawLineListCommandStaticStateTrait for UIDrawLineListCommandStaticState {
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWLINELISTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawLineListCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawLineListCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawLineListCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWLINELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawLineListCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWLINELISTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWLINELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawLineListCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawLineListCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawSolidTriangleListCommandDynamicState {
    pub vertices: Vec<super::core::Vec3>,
    pub non_premultiplied_colors: Vec<super::core::Vec4>,
    pub indices: Vec<u16>,
    pub field_flag_changed0: u8,
}

pub trait UIDrawSolidTriangleListCommandDynamicStateTrait: TypeObject {
    fn vertices(&self) -> &Vec<super::core::Vec3>;
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec3>;
    fn non_premultiplied_colors(&self) -> &Vec<super::core::Vec4>;
    fn non_premultiplied_colors_mut(&mut self) -> &mut Vec<super::core::Vec4>;
    fn indices(&self) -> &Vec<u16>;
    fn indices_mut(&mut self) -> &mut Vec<u16>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawSolidTriangleListCommandDynamicStateTrait for UIDrawSolidTriangleListCommandDynamicState {
    fn vertices(&self) -> &Vec<super::core::Vec3> {
        &self.vertices
    }
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec3> {
        &mut self.vertices
    }
    fn non_premultiplied_colors(&self) -> &Vec<super::core::Vec4> {
        &self.non_premultiplied_colors
    }
    fn non_premultiplied_colors_mut(&mut self) -> &mut Vec<super::core::Vec4> {
        &mut self.non_premultiplied_colors
    }
    fn indices(&self) -> &Vec<u16> {
        &self.indices
    }
    fn indices_mut(&mut self) -> &mut Vec<u16> {
        &mut self.indices
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawSolidTriangleListCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "NonPremultipliedColors",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec4-Array",
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, non_premultiplied_colors),
            },
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, indices),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawSolidTriangleListCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWSOLIDTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawSolidTriangleListCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawSolidTriangleListCommandStaticState {
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawSolidTriangleListCommandStaticStateTrait: TypeObject {
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawSolidTriangleListCommandStaticStateTrait for UIDrawSolidTriangleListCommandStaticState {
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawSolidTriangleListCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawSolidTriangleListCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawSolidTriangleListCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWSOLIDTRIANGLELISTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSolidTriangleListCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawSolidTriangleListCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawTriangleListCommandDynamicState {
    pub texture: super::render_base::TextureResourceHandle,
    pub vertices: Vec<super::core::Vec3>,
    pub uvs: Vec<super::core::Vec2>,
    pub indices: Vec<u16>,
    pub field_flag_changed0: u8,
}

pub trait UIDrawTriangleListCommandDynamicStateTrait: TypeObject {
    fn texture(&self) -> &super::render_base::TextureResourceHandle;
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn vertices(&self) -> &Vec<super::core::Vec3>;
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec3>;
    fn uvs(&self) -> &Vec<super::core::Vec2>;
    fn uvs_mut(&mut self) -> &mut Vec<super::core::Vec2>;
    fn indices(&self) -> &Vec<u16>;
    fn indices_mut(&mut self) -> &mut Vec<u16>;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawTriangleListCommandDynamicStateTrait for UIDrawTriangleListCommandDynamicState {
    fn texture(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture
    }
    fn vertices(&self) -> &Vec<super::core::Vec3> {
        &self.vertices
    }
    fn vertices_mut(&mut self) -> &mut Vec<super::core::Vec3> {
        &mut self.vertices
    }
    fn uvs(&self) -> &Vec<super::core::Vec2> {
        &self.uvs
    }
    fn uvs_mut(&mut self) -> &mut Vec<super::core::Vec2> {
        &mut self.uvs
    }
    fn indices(&self) -> &Vec<u16> {
        &self.indices
    }
    fn indices_mut(&mut self) -> &mut Vec<u16> {
        &mut self.indices
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTriangleListCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawTriangleListCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "Vertices",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, vertices),
            },
            FieldInfoData {
                name: "Uvs",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec2-Array",
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, uvs),
            },
            FieldInfoData {
                name: "Indices",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint16-Array",
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, indices),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawTriangleListCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIDrawTriangleListCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWTRIANGLELISTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawTriangleListCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawTriangleListCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawDistanceFieldCommandDynamicState {
    pub texture: super::render_base::TextureResourceHandle,
    pub corners: Vec<super::core::Vec3>,
    pub uv_rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub trait UIDrawDistanceFieldCommandDynamicStateTrait: TypeObject {
    fn texture(&self) -> &super::render_base::TextureResourceHandle;
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn corners(&self) -> &Vec<super::core::Vec3>;
    fn corners_mut(&mut self) -> &mut Vec<super::core::Vec3>;
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn non_premultiplied_color(&self) -> &super::core::Vec4;
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawDistanceFieldCommandDynamicStateTrait for UIDrawDistanceFieldCommandDynamicState {
    fn texture(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture
    }
    fn corners(&self) -> &Vec<super::core::Vec3> {
        &self.corners
    }
    fn corners_mut(&mut self) -> &mut Vec<super::core::Vec3> {
        &mut self.corners
    }
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
    fn non_premultiplied_color(&self) -> &super::core::Vec4 {
        &self.non_premultiplied_color
    }
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.non_premultiplied_color
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawDistanceFieldCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "Corners",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3-Array",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, corners),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, uv_rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawDistanceFieldCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWDISTANCEFIELDCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDistanceFieldCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawDistanceFieldCommandStaticState {
    pub params: super::game_shared_u_i::UIElementBitmapDistanceFieldParams,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawDistanceFieldCommandStaticStateTrait: TypeObject {
    fn params(&self) -> &super::game_shared_u_i::UIElementBitmapDistanceFieldParams;
    fn params_mut(&mut self) -> &mut super::game_shared_u_i::UIElementBitmapDistanceFieldParams;
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawDistanceFieldCommandStaticStateTrait for UIDrawDistanceFieldCommandStaticState {
    fn params(&self) -> &super::game_shared_u_i::UIElementBitmapDistanceFieldParams {
        &self.params
    }
    fn params_mut(&mut self) -> &mut super::game_shared_u_i::UIElementBitmapDistanceFieldParams {
        &mut self.params
    }
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawDistanceFieldCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Params",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementBitmapDistanceFieldParams",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandStaticState, params),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawDistanceFieldCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawDistanceFieldCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWDISTANCEFIELDCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawDistanceFieldCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawDistanceFieldCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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

pub trait UIDrawAdvancedRectCommandDynamicStateTrait: TypeObject {
    fn texture1(&self) -> &super::render_base::TextureResourceHandle;
    fn texture1_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn uv_rect1(&self) -> &super::core::Vec4;
    fn uv_rect1_mut(&mut self) -> &mut super::core::Vec4;
    fn texture2(&self) -> &super::render_base::TextureResourceHandle;
    fn texture2_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn uv_rect2(&self) -> &super::core::Vec4;
    fn uv_rect2_mut(&mut self) -> &mut super::core::Vec4;
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn non_premultiplied_color(&self) -> &super::core::Vec4;
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4;
    fn gradient_params(&self) -> &UIGradientRectParams;
    fn gradient_params_mut(&mut self) -> &mut UIGradientRectParams;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawAdvancedRectCommandDynamicStateTrait for UIDrawAdvancedRectCommandDynamicState {
    fn texture1(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture1
    }
    fn texture1_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture1
    }
    fn uv_rect1(&self) -> &super::core::Vec4 {
        &self.uv_rect1
    }
    fn uv_rect1_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect1
    }
    fn texture2(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture2
    }
    fn texture2_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture2
    }
    fn uv_rect2(&self) -> &super::core::Vec4 {
        &self.uv_rect2
    }
    fn uv_rect2_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect2
    }
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn non_premultiplied_color(&self) -> &super::core::Vec4 {
        &self.non_premultiplied_color
    }
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.non_premultiplied_color
    }
    fn gradient_params(&self) -> &UIGradientRectParams {
        &self.gradient_params
    }
    fn gradient_params_mut(&mut self) -> &mut UIGradientRectParams {
        &mut self.gradient_params
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawAdvancedRectCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Texture1",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, texture1),
            },
            FieldInfoData {
                name: "UvRect1",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, uv_rect1),
            },
            FieldInfoData {
                name: "Texture2",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, texture2),
            },
            FieldInfoData {
                name: "UvRect2",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, uv_rect2),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "GradientParams",
                flags: MemberInfoFlags::new(0),
                field_type: "UIGradientRectParams",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, gradient_params),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawAdvancedRectCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWADVANCEDRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAdvancedRectCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawAdvancedRectCommandStaticState {
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub filled: bool,
    pub outlined: bool,
    pub gradient: bool,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawAdvancedRectCommandStaticStateTrait: TypeObject {
    fn address_u(&self) -> &super::render_base::TextureAddress;
    fn address_u_mut(&mut self) -> &mut super::render_base::TextureAddress;
    fn address_v(&self) -> &super::render_base::TextureAddress;
    fn address_v_mut(&mut self) -> &mut super::render_base::TextureAddress;
    fn filled(&self) -> &bool;
    fn filled_mut(&mut self) -> &mut bool;
    fn outlined(&self) -> &bool;
    fn outlined_mut(&mut self) -> &mut bool;
    fn gradient(&self) -> &bool;
    fn gradient_mut(&mut self) -> &mut bool;
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawAdvancedRectCommandStaticStateTrait for UIDrawAdvancedRectCommandStaticState {
    fn address_u(&self) -> &super::render_base::TextureAddress {
        &self.address_u
    }
    fn address_u_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address_u
    }
    fn address_v(&self) -> &super::render_base::TextureAddress {
        &self.address_v
    }
    fn address_v_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address_v
    }
    fn filled(&self) -> &bool {
        &self.filled
    }
    fn filled_mut(&mut self) -> &mut bool {
        &mut self.filled
    }
    fn outlined(&self) -> &bool {
        &self.outlined
    }
    fn outlined_mut(&mut self) -> &mut bool {
        &mut self.outlined
    }
    fn gradient(&self) -> &bool {
        &self.gradient
    }
    fn gradient_mut(&mut self) -> &mut bool {
        &mut self.gradient
    }
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawAdvancedRectCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, address_v),
            },
            FieldInfoData {
                name: "Filled",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, filled),
            },
            FieldInfoData {
                name: "Outlined",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, outlined),
            },
            FieldInfoData {
                name: "Gradient",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, gradient),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawAdvancedRectCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for UIDrawAdvancedRectCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_TYPE_INFO
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


pub static UIDRAWADVANCEDRECTCOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawAdvancedRectCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawAdvancedRectCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIGradientRectParams {
    pub top_left_color: super::core::Vec4,
    pub top_right_color: super::core::Vec4,
    pub bottom_left_color: super::core::Vec4,
    pub bottom_right_color: super::core::Vec4,
}

pub trait UIGradientRectParamsTrait: TypeObject {
    fn top_left_color(&self) -> &super::core::Vec4;
    fn top_left_color_mut(&mut self) -> &mut super::core::Vec4;
    fn top_right_color(&self) -> &super::core::Vec4;
    fn top_right_color_mut(&mut self) -> &mut super::core::Vec4;
    fn bottom_left_color(&self) -> &super::core::Vec4;
    fn bottom_left_color_mut(&mut self) -> &mut super::core::Vec4;
    fn bottom_right_color(&self) -> &super::core::Vec4;
    fn bottom_right_color_mut(&mut self) -> &mut super::core::Vec4;
}

impl UIGradientRectParamsTrait for UIGradientRectParams {
    fn top_left_color(&self) -> &super::core::Vec4 {
        &self.top_left_color
    }
    fn top_left_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.top_left_color
    }
    fn top_right_color(&self) -> &super::core::Vec4 {
        &self.top_right_color
    }
    fn top_right_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.top_right_color
    }
    fn bottom_left_color(&self) -> &super::core::Vec4 {
        &self.bottom_left_color
    }
    fn bottom_left_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.bottom_left_color
    }
    fn bottom_right_color(&self) -> &super::core::Vec4 {
        &self.bottom_right_color
    }
    fn bottom_right_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.bottom_right_color
    }
}

pub static UIGRADIENTRECTPARAMS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGradientRectParams",
    flags: MemberInfoFlags::new(36937),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIGradientRectParams as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TopLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIGradientRectParams, top_left_color),
            },
            FieldInfoData {
                name: "TopRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIGradientRectParams, top_right_color),
            },
            FieldInfoData {
                name: "BottomLeftColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIGradientRectParams, bottom_left_color),
            },
            FieldInfoData {
                name: "BottomRightColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIGradientRectParams, bottom_right_color),
            },
        ],
    }),
    array_type: Some(UIGRADIENTRECTPARAMS_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIGradientRectParams {
    fn type_info(&self) -> &'static TypeInfo {
        UIGRADIENTRECTPARAMS_TYPE_INFO
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


pub static UIGRADIENTRECTPARAMS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIGradientRectParams-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIGradientRectParams"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawRectCommandDynamicState {
    pub texture: super::render_base::TextureResourceHandle,
    pub rect: super::core::Vec4,
    pub uv_rect: super::core::Vec4,
    pub non_premultiplied_color: super::core::Vec4,
    pub shader_program: i32,
    pub field_flag_changed0: u8,
}

pub trait UIDrawRectCommandDynamicStateTrait: TypeObject {
    fn texture(&self) -> &super::render_base::TextureResourceHandle;
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn non_premultiplied_color(&self) -> &super::core::Vec4;
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4;
    fn shader_program(&self) -> &i32;
    fn shader_program_mut(&mut self) -> &mut i32;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawRectCommandDynamicStateTrait for UIDrawRectCommandDynamicState {
    fn texture(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture
    }
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
    fn non_premultiplied_color(&self) -> &super::core::Vec4 {
        &self.non_premultiplied_color
    }
    fn non_premultiplied_color_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.non_premultiplied_color
    }
    fn shader_program(&self) -> &i32 {
        &self.shader_program
    }
    fn shader_program_mut(&mut self) -> &mut i32 {
        &mut self.shader_program
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWRECTCOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawRectCommandDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "GameClientUI",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawRectCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, texture),
            },
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, uv_rect),
            },
            FieldInfoData {
                name: "NonPremultipliedColor",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, non_premultiplied_color),
            },
            FieldInfoData {
                name: "ShaderProgram",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, shader_program),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawRectCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawRectCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWRECTCOMMANDDYNAMICSTATE_TYPE_INFO
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


pub static UIDRAWRECTCOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawRectCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UIDrawRectCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientUIInputEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientUIInputEntityTrait: super::entity::EntityTrait {
}

impl ClientUIInputEntityTrait for ClientUIInputEntity {
}

impl super::entity::EntityTrait for ClientUIInputEntity {
}

impl super::entity::EntityBusPeerTrait for ClientUIInputEntity {
}

pub static CLIENTUIINPUTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIInputEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientUIInputEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTUIINPUTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientUIInputEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTUIINPUTENTITY_TYPE_INFO
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


pub static CLIENTUIINPUTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientUIInputEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientUIInputEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientMovieTrack {
    pub _glacier_base: super::timeline::TimelineTrack,
}

pub trait ClientMovieTrackTrait: super::timeline::TimelineTrackTrait {
}

impl ClientMovieTrackTrait for ClientMovieTrack {
}

impl super::timeline::TimelineTrackTrait for ClientMovieTrack {
}

pub static CLIENTMOVIETRACK_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieTrack",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::timeline::TIMELINETRACK_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientMovieTrack as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTMOVIETRACK_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientMovieTrack {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTMOVIETRACK_TYPE_INFO
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


pub static CLIENTMOVIETRACK_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientMovieTrack-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientMovieTrack"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UISystem {
    pub _glacier_base: super::game_client::IUISystem,
}

pub trait UISystemTrait: super::game_client::IUISystemTrait {
}

impl UISystemTrait for UISystem {
}

impl super::game_client::IUISystemTrait for UISystem {
}

pub static UISYSTEM_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystem",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client::IUISYSTEM_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UISystem as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UISYSTEM_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UISystem {
    fn type_info(&self) -> &'static TypeInfo {
        UISYSTEM_TYPE_INFO
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


pub static UISYSTEM_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UISystem-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UISystem"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ClientPlayVideoEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait ClientPlayVideoEntityTrait: super::entity::EntityTrait {
}

impl ClientPlayVideoEntityTrait for ClientPlayVideoEntity {
}

impl super::entity::EntityTrait for ClientPlayVideoEntity {
}

impl super::entity::EntityBusPeerTrait for ClientPlayVideoEntity {
}

pub static CLIENTPLAYVIDEOENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVideoEntity",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ClientPlayVideoEntity as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CLIENTPLAYVIDEOENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ClientPlayVideoEntity {
    fn type_info(&self) -> &'static TypeInfo {
        CLIENTPLAYVIDEOENTITY_TYPE_INFO
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


pub static CLIENTPLAYVIDEOENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ClientPlayVideoEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("ClientPlayVideoEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UITtfFontFile {
}

pub trait UITtfFontFileTrait: TypeObject {
}

impl UITtfFontFileTrait for UITtfFontFile {
}

pub static UITTFFONTFILE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfFontFile",
    flags: MemberInfoFlags::new(101),
    module: "GameClientUI",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UITtfFontFile as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(UITTFFONTFILE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UITtfFontFile {
    fn type_info(&self) -> &'static TypeInfo {
        UITTFFONTFILE_TYPE_INFO
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


pub static UITTFFONTFILE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UITtfFontFile-Array",
    flags: MemberInfoFlags::new(145),
    module: "GameClientUI",
    data: TypeInfoData::Array("UITtfFontFile"),
    array_type: None,
    alignment: 8,
};


