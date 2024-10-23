use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_web_browser_types(registry: &mut TypeRegistry) {
    registry.register_type(LEVELWEBBROWSERDESCRIPTIONCOMPONENT_TYPE_INFO);
    registry.register_type(LEVELWEBBROWSERDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO);
    registry.register_type(WEBBROWSERSETTINGS_TYPE_INFO);
    registry.register_type(WEBBROWSERSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(UIWEBVIEWNOTIFICATIONERRORMESSAGE_TYPE_INFO);
    registry.register_type(WEBVIEWERROR_TYPE_INFO);
    registry.register_type(WEBVIEWERROR_ARRAY_TYPE_INFO);
    registry.register_type(UIWEBVIEWNOTIFICATIONURLCHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(UIWEBVIEWREQUESTCLOSEVIEWMESSAGE_TYPE_INFO);
    registry.register_type(UIWEBVIEWREQUESTVIEWPAGEMESSAGE_TYPE_INFO);
    registry.register_type(UIWEBVIEWWIDGETDATA_TYPE_INFO);
    registry.register_type(UIWEBVIEWWIDGETDATA_ARRAY_TYPE_INFO);
    registry.register_type(WEBBROWSERBUNDLEASSET_TYPE_INFO);
    registry.register_type(WEBBROWSERBUNDLEASSET_ARRAY_TYPE_INFO);
    registry.register_type(WEBBROWSERLOCALURLASSET_TYPE_INFO);
    registry.register_type(WEBBROWSERLOCALURLASSET_ARRAY_TYPE_INFO);
    registry.register_type(UIWEBVIEWWIDGET_TYPE_INFO);
    registry.register_type(UIWEBVIEWWIDGET_ARRAY_TYPE_INFO);
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct LevelWebBrowserDescriptionComponent {
    pub _glacier_base: super::gameplay_sim::LevelDescriptionComponent,
    pub create_web_browser: super::core::PlatformScalableBool,
}

pub trait LevelWebBrowserDescriptionComponentTrait: super::gameplay_sim::LevelDescriptionComponentTrait {
    fn create_web_browser(&self) -> &super::core::PlatformScalableBool;
    fn create_web_browser_mut(&mut self) -> &mut super::core::PlatformScalableBool;
}

impl LevelWebBrowserDescriptionComponentTrait for LevelWebBrowserDescriptionComponent {
    fn create_web_browser(&self) -> &super::core::PlatformScalableBool {
        &self.create_web_browser
    }
    fn create_web_browser_mut(&mut self) -> &mut super::core::PlatformScalableBool {
        &mut self.create_web_browser
    }
}

impl super::gameplay_sim::LevelDescriptionComponentTrait for LevelWebBrowserDescriptionComponent {
}

impl super::core::DataContainerTrait for LevelWebBrowserDescriptionComponent {
}

pub static LEVELWEBBROWSERDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelWebBrowserDescriptionComponent",
    name_hash: 3353698534,
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        super_class_offset: offset_of!(LevelWebBrowserDescriptionComponent, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelWebBrowserDescriptionComponent as Default>::default())),
            create_boxed: || Box::new(<LevelWebBrowserDescriptionComponent as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CreateWebBrowser",
                name_hash: 2299670173,
                flags: MemberInfoFlags::new(0),
                field_type: "PlatformScalableBool",
                rust_offset: offset_of!(LevelWebBrowserDescriptionComponent, create_web_browser),
            },
        ],
    }),
    array_type: Some(LEVELWEBBROWSERDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelWebBrowserDescriptionComponent {
    fn type_info(&self) -> &'static TypeInfo {
        LEVELWEBBROWSERDESCRIPTIONCOMPONENT_TYPE_INFO
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


pub static LEVELWEBBROWSERDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelWebBrowserDescriptionComponent-Array",
    name_hash: 461430226,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("LevelWebBrowserDescriptionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WebBrowserSettings {
    pub _glacier_base: super::core::SystemSettings,
    pub application_name: String,
    pub browser_heap_size: i32,
    pub allocate_heap_on_module_startup: bool,
    pub standard_font: String,
    pub serif_font: String,
    pub sans_serif_font: String,
    pub monospace_font: String,
    pub cursive_font: String,
    pub fantasy_font: String,
    pub system_font: String,
    pub system_font_bold: bool,
    pub default_c_s_s: String,
    pub web_browser_bundle: Option<LockedTypeObject /* WebBrowserBundleAsset */>,
    pub per_level_web_browser_enable: bool,
}

pub trait WebBrowserSettingsTrait: super::core::SystemSettingsTrait {
    fn application_name(&self) -> &String;
    fn application_name_mut(&mut self) -> &mut String;
    fn browser_heap_size(&self) -> &i32;
    fn browser_heap_size_mut(&mut self) -> &mut i32;
    fn allocate_heap_on_module_startup(&self) -> &bool;
    fn allocate_heap_on_module_startup_mut(&mut self) -> &mut bool;
    fn standard_font(&self) -> &String;
    fn standard_font_mut(&mut self) -> &mut String;
    fn serif_font(&self) -> &String;
    fn serif_font_mut(&mut self) -> &mut String;
    fn sans_serif_font(&self) -> &String;
    fn sans_serif_font_mut(&mut self) -> &mut String;
    fn monospace_font(&self) -> &String;
    fn monospace_font_mut(&mut self) -> &mut String;
    fn cursive_font(&self) -> &String;
    fn cursive_font_mut(&mut self) -> &mut String;
    fn fantasy_font(&self) -> &String;
    fn fantasy_font_mut(&mut self) -> &mut String;
    fn system_font(&self) -> &String;
    fn system_font_mut(&mut self) -> &mut String;
    fn system_font_bold(&self) -> &bool;
    fn system_font_bold_mut(&mut self) -> &mut bool;
    fn default_c_s_s(&self) -> &String;
    fn default_c_s_s_mut(&mut self) -> &mut String;
    fn web_browser_bundle(&self) -> &Option<LockedTypeObject /* WebBrowserBundleAsset */>;
    fn web_browser_bundle_mut(&mut self) -> &mut Option<LockedTypeObject /* WebBrowserBundleAsset */>;
    fn per_level_web_browser_enable(&self) -> &bool;
    fn per_level_web_browser_enable_mut(&mut self) -> &mut bool;
}

impl WebBrowserSettingsTrait for WebBrowserSettings {
    fn application_name(&self) -> &String {
        &self.application_name
    }
    fn application_name_mut(&mut self) -> &mut String {
        &mut self.application_name
    }
    fn browser_heap_size(&self) -> &i32 {
        &self.browser_heap_size
    }
    fn browser_heap_size_mut(&mut self) -> &mut i32 {
        &mut self.browser_heap_size
    }
    fn allocate_heap_on_module_startup(&self) -> &bool {
        &self.allocate_heap_on_module_startup
    }
    fn allocate_heap_on_module_startup_mut(&mut self) -> &mut bool {
        &mut self.allocate_heap_on_module_startup
    }
    fn standard_font(&self) -> &String {
        &self.standard_font
    }
    fn standard_font_mut(&mut self) -> &mut String {
        &mut self.standard_font
    }
    fn serif_font(&self) -> &String {
        &self.serif_font
    }
    fn serif_font_mut(&mut self) -> &mut String {
        &mut self.serif_font
    }
    fn sans_serif_font(&self) -> &String {
        &self.sans_serif_font
    }
    fn sans_serif_font_mut(&mut self) -> &mut String {
        &mut self.sans_serif_font
    }
    fn monospace_font(&self) -> &String {
        &self.monospace_font
    }
    fn monospace_font_mut(&mut self) -> &mut String {
        &mut self.monospace_font
    }
    fn cursive_font(&self) -> &String {
        &self.cursive_font
    }
    fn cursive_font_mut(&mut self) -> &mut String {
        &mut self.cursive_font
    }
    fn fantasy_font(&self) -> &String {
        &self.fantasy_font
    }
    fn fantasy_font_mut(&mut self) -> &mut String {
        &mut self.fantasy_font
    }
    fn system_font(&self) -> &String {
        &self.system_font
    }
    fn system_font_mut(&mut self) -> &mut String {
        &mut self.system_font
    }
    fn system_font_bold(&self) -> &bool {
        &self.system_font_bold
    }
    fn system_font_bold_mut(&mut self) -> &mut bool {
        &mut self.system_font_bold
    }
    fn default_c_s_s(&self) -> &String {
        &self.default_c_s_s
    }
    fn default_c_s_s_mut(&mut self) -> &mut String {
        &mut self.default_c_s_s
    }
    fn web_browser_bundle(&self) -> &Option<LockedTypeObject /* WebBrowserBundleAsset */> {
        &self.web_browser_bundle
    }
    fn web_browser_bundle_mut(&mut self) -> &mut Option<LockedTypeObject /* WebBrowserBundleAsset */> {
        &mut self.web_browser_bundle
    }
    fn per_level_web_browser_enable(&self) -> &bool {
        &self.per_level_web_browser_enable
    }
    fn per_level_web_browser_enable_mut(&mut self) -> &mut bool {
        &mut self.per_level_web_browser_enable
    }
}

impl super::core::SystemSettingsTrait for WebBrowserSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for WebBrowserSettings {
}

pub static WEBBROWSERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserSettings",
    name_hash: 1369423292,
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        super_class_offset: offset_of!(WebBrowserSettings, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WebBrowserSettings as Default>::default())),
            create_boxed: || Box::new(<WebBrowserSettings as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "ApplicationName",
                name_hash: 3823246520,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, application_name),
            },
            FieldInfoData {
                name: "BrowserHeapSize",
                name_hash: 1489197936,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WebBrowserSettings, browser_heap_size),
            },
            FieldInfoData {
                name: "AllocateHeapOnModuleStartup",
                name_hash: 2211253562,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WebBrowserSettings, allocate_heap_on_module_startup),
            },
            FieldInfoData {
                name: "StandardFont",
                name_hash: 4053334349,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, standard_font),
            },
            FieldInfoData {
                name: "SerifFont",
                name_hash: 4267279581,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, serif_font),
            },
            FieldInfoData {
                name: "SansSerifFont",
                name_hash: 67885938,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, sans_serif_font),
            },
            FieldInfoData {
                name: "MonospaceFont",
                name_hash: 721201329,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, monospace_font),
            },
            FieldInfoData {
                name: "CursiveFont",
                name_hash: 3479533915,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, cursive_font),
            },
            FieldInfoData {
                name: "FantasyFont",
                name_hash: 3666783744,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, fantasy_font),
            },
            FieldInfoData {
                name: "SystemFont",
                name_hash: 279800883,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, system_font),
            },
            FieldInfoData {
                name: "SystemFontBold",
                name_hash: 1161750038,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WebBrowserSettings, system_font_bold),
            },
            FieldInfoData {
                name: "DefaultCSS",
                name_hash: 2143458765,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, default_c_s_s),
            },
            FieldInfoData {
                name: "WebBrowserBundle",
                name_hash: 4051360781,
                flags: MemberInfoFlags::new(0),
                field_type: "WebBrowserBundleAsset",
                rust_offset: offset_of!(WebBrowserSettings, web_browser_bundle),
            },
            FieldInfoData {
                name: "PerLevelWebBrowserEnable",
                name_hash: 3965730249,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WebBrowserSettings, per_level_web_browser_enable),
            },
        ],
    }),
    array_type: Some(WEBBROWSERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WebBrowserSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WEBBROWSERSETTINGS_TYPE_INFO
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


pub static WEBBROWSERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserSettings-Array",
    name_hash: 3009918216,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIWebViewNotificationErrorMessage {
}

pub trait UIWebViewNotificationErrorMessageTrait: TypeObject {
}

impl UIWebViewNotificationErrorMessageTrait for UIWebViewNotificationErrorMessage {
}

pub static UIWEBVIEWNOTIFICATIONERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewNotificationErrorMessage",
    name_hash: 2252716378,
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewNotificationErrorMessage as Default>::default())),
            create_boxed: || Box::new(<UIWebViewNotificationErrorMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewNotificationErrorMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIWEBVIEWNOTIFICATIONERRORMESSAGE_TYPE_INFO
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

#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WebViewError {
    #[default]
    WEBVIEW_ERR_TIMEOUT = 0,
    WEBVIEW_ERR_SSLCERT = 1,
    WEBVIEW_ERR_SERVER = 2,
    WEBVIEW_ERR_UNEXPECTED_RESPONSE = 3,
    WEBVIEW_ERR_UNKNOWN = 4,
}

pub static WEBVIEWERROR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebViewError",
    name_hash: 197573536,
    flags: MemberInfoFlags::new(49429),
    module: "WebBrowser",
    data: TypeInfoData::Enum,
    array_type: Some(WEBVIEWERROR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WebViewError {
    fn type_info(&self) -> &'static TypeInfo {
        WEBVIEWERROR_TYPE_INFO
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


pub static WEBVIEWERROR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebViewError-Array",
    name_hash: 117661972,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebViewError"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIWebViewNotificationUrlChangedMessage {
}

pub trait UIWebViewNotificationUrlChangedMessageTrait: TypeObject {
}

impl UIWebViewNotificationUrlChangedMessageTrait for UIWebViewNotificationUrlChangedMessage {
}

pub static UIWEBVIEWNOTIFICATIONURLCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewNotificationUrlChangedMessage",
    name_hash: 3300684075,
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewNotificationUrlChangedMessage as Default>::default())),
            create_boxed: || Box::new(<UIWebViewNotificationUrlChangedMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewNotificationUrlChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIWEBVIEWNOTIFICATIONURLCHANGEDMESSAGE_TYPE_INFO
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
pub struct UIWebViewRequestCloseViewMessage {
}

pub trait UIWebViewRequestCloseViewMessageTrait: TypeObject {
}

impl UIWebViewRequestCloseViewMessageTrait for UIWebViewRequestCloseViewMessage {
}

pub static UIWEBVIEWREQUESTCLOSEVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewRequestCloseViewMessage",
    name_hash: 3841627621,
    flags: MemberInfoFlags::new(36937),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewRequestCloseViewMessage as Default>::default())),
            create_boxed: || Box::new(<UIWebViewRequestCloseViewMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewRequestCloseViewMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIWEBVIEWREQUESTCLOSEVIEWMESSAGE_TYPE_INFO
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
pub struct UIWebViewRequestViewPageMessage {
}

pub trait UIWebViewRequestViewPageMessageTrait: TypeObject {
}

impl UIWebViewRequestViewPageMessageTrait for UIWebViewRequestViewPageMessage {
}

pub static UIWEBVIEWREQUESTVIEWPAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewRequestViewPageMessage",
    name_hash: 1771425920,
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewRequestViewPageMessage as Default>::default())),
            create_boxed: || Box::new(<UIWebViewRequestViewPageMessage as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewRequestViewPageMessage {
    fn type_info(&self) -> &'static TypeInfo {
        UIWEBVIEWREQUESTVIEWPAGEMESSAGE_TYPE_INFO
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
pub struct UIWebViewWidgetData {
    pub _glacier_base: super::game_shared_u_i::UIWidgetEntityData,
}

pub trait UIWebViewWidgetDataTrait: super::game_shared_u_i::UIWidgetEntityDataTrait {
}

impl UIWebViewWidgetDataTrait for UIWebViewWidgetData {
}

impl super::game_shared_u_i::UIWidgetEntityDataTrait for UIWebViewWidgetData {
    fn size(&self) -> &super::game_shared_u_i::UIElementSize {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::game_shared_u_i::UIElementSize {
        self._glacier_base.size_mut()
    }
    fn layers(&self) -> &Vec<Option<LockedTypeObject /* super::game_shared_u_i::UIElementLayerEntityData */>> {
        self._glacier_base.layers()
    }
    fn layers_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::game_shared_u_i::UIElementLayerEntityData */>> {
        self._glacier_base.layers_mut()
    }
    fn texture_mappings(&self) -> &Vec<Option<LockedTypeObject /* super::game_shared_u_i::UITextureMappingAsset */>> {
        self._glacier_base.texture_mappings()
    }
    fn texture_mappings_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::game_shared_u_i::UITextureMappingAsset */>> {
        self._glacier_base.texture_mappings_mut()
    }
    fn components(&self) -> &Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<LockedTypeObject /* super::entity::GameObjectData */>> {
        self._glacier_base.components_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
}

impl super::entity::EntityDataTrait for UIWebViewWidgetData {
}

impl super::entity::GameObjectDataTrait for UIWebViewWidgetData {
}

impl super::core::DataBusPeerTrait for UIWebViewWidgetData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIWebViewWidgetData {
}

impl super::core::DataContainerTrait for UIWebViewWidgetData {
}

pub static UIWEBVIEWWIDGETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidgetData",
    name_hash: 164301048,
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIWIDGETENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(UIWebViewWidgetData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewWidgetData as Default>::default())),
            create_boxed: || Box::new(<UIWebViewWidgetData as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIWEBVIEWWIDGETDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIWebViewWidgetData {
    fn type_info(&self) -> &'static TypeInfo {
        UIWEBVIEWWIDGETDATA_TYPE_INFO
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


pub static UIWEBVIEWWIDGETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidgetData-Array",
    name_hash: 69911244,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("UIWebViewWidgetData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WebBrowserBundleAsset {
    pub _glacier_base: super::core::Asset,
    pub bundle_path: String,
    pub fonts: Vec<String>,
    pub local_u_r_ls: Vec<String>,
}

pub trait WebBrowserBundleAssetTrait: super::core::AssetTrait {
    fn bundle_path(&self) -> &String;
    fn bundle_path_mut(&mut self) -> &mut String;
    fn fonts(&self) -> &Vec<String>;
    fn fonts_mut(&mut self) -> &mut Vec<String>;
    fn local_u_r_ls(&self) -> &Vec<String>;
    fn local_u_r_ls_mut(&mut self) -> &mut Vec<String>;
}

impl WebBrowserBundleAssetTrait for WebBrowserBundleAsset {
    fn bundle_path(&self) -> &String {
        &self.bundle_path
    }
    fn bundle_path_mut(&mut self) -> &mut String {
        &mut self.bundle_path
    }
    fn fonts(&self) -> &Vec<String> {
        &self.fonts
    }
    fn fonts_mut(&mut self) -> &mut Vec<String> {
        &mut self.fonts
    }
    fn local_u_r_ls(&self) -> &Vec<String> {
        &self.local_u_r_ls
    }
    fn local_u_r_ls_mut(&mut self) -> &mut Vec<String> {
        &mut self.local_u_r_ls
    }
}

impl super::core::AssetTrait for WebBrowserBundleAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for WebBrowserBundleAsset {
}

pub static WEBBROWSERBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserBundleAsset",
    name_hash: 1061117437,
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(WebBrowserBundleAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WebBrowserBundleAsset as Default>::default())),
            create_boxed: || Box::new(<WebBrowserBundleAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "BundlePath",
                name_hash: 459940508,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserBundleAsset, bundle_path),
            },
            FieldInfoData {
                name: "Fonts",
                name_hash: 206880581,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(WebBrowserBundleAsset, fonts),
            },
            FieldInfoData {
                name: "LocalURLs",
                name_hash: 773227504,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(WebBrowserBundleAsset, local_u_r_ls),
            },
        ],
    }),
    array_type: Some(WEBBROWSERBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WebBrowserBundleAsset {
    fn type_info(&self) -> &'static TypeInfo {
        WEBBROWSERBUNDLEASSET_TYPE_INFO
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


pub static WEBBROWSERBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserBundleAsset-Array",
    name_hash: 2666924745,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserBundleAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct WebBrowserLocalURLAsset {
    pub _glacier_base: super::core::Asset,
    pub local_path: String,
    pub file: Option<LockedTypeObject /* super::game_shared::RawFileDataAsset */>,
}

pub trait WebBrowserLocalURLAssetTrait: super::core::AssetTrait {
    fn local_path(&self) -> &String;
    fn local_path_mut(&mut self) -> &mut String;
    fn file(&self) -> &Option<LockedTypeObject /* super::game_shared::RawFileDataAsset */>;
    fn file_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::RawFileDataAsset */>;
}

impl WebBrowserLocalURLAssetTrait for WebBrowserLocalURLAsset {
    fn local_path(&self) -> &String {
        &self.local_path
    }
    fn local_path_mut(&mut self) -> &mut String {
        &mut self.local_path
    }
    fn file(&self) -> &Option<LockedTypeObject /* super::game_shared::RawFileDataAsset */> {
        &self.file
    }
    fn file_mut(&mut self) -> &mut Option<LockedTypeObject /* super::game_shared::RawFileDataAsset */> {
        &mut self.file
    }
}

impl super::core::AssetTrait for WebBrowserLocalURLAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for WebBrowserLocalURLAsset {
}

pub static WEBBROWSERLOCALURLASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserLocalURLAsset",
    name_hash: 3231029679,
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(WebBrowserLocalURLAsset, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WebBrowserLocalURLAsset as Default>::default())),
            create_boxed: || Box::new(<WebBrowserLocalURLAsset as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPath",
                name_hash: 773167557,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserLocalURLAsset, local_path),
            },
            FieldInfoData {
                name: "File",
                name_hash: 2088671139,
                flags: MemberInfoFlags::new(0),
                field_type: "RawFileDataAsset",
                rust_offset: offset_of!(WebBrowserLocalURLAsset, file),
            },
        ],
    }),
    array_type: Some(WEBBROWSERLOCALURLASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WebBrowserLocalURLAsset {
    fn type_info(&self) -> &'static TypeInfo {
        WEBBROWSERLOCALURLASSET_TYPE_INFO
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


pub static WEBBROWSERLOCALURLASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserLocalURLAsset-Array",
    name_hash: 1909425947,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserLocalURLAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct UIWebViewWidget {
    pub _glacier_base: super::game_client_u_i::UIWidgetEntity,
}

pub trait UIWebViewWidgetTrait: super::game_client_u_i::UIWidgetEntityTrait {
}

impl UIWebViewWidgetTrait for UIWebViewWidget {
}

impl super::game_client_u_i::UIWidgetEntityTrait for UIWebViewWidget {
}

impl super::entity::EntityTrait for UIWebViewWidget {
}

impl super::entity::EntityBusPeerTrait for UIWebViewWidget {
}

pub static UIWEBVIEWWIDGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidget",
    name_hash: 4114030792,
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIWIDGETENTITY_TYPE_INFO),
        super_class_offset: offset_of!(UIWebViewWidget, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewWidget as Default>::default())),
            create_boxed: || Box::new(<UIWebViewWidget as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(UIWEBVIEWWIDGET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIWebViewWidget {
    fn type_info(&self) -> &'static TypeInfo {
        UIWEBVIEWWIDGET_TYPE_INFO
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


pub static UIWEBVIEWWIDGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidget-Array",
    name_hash: 1973334908,
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("UIWebViewWidget"),
    array_type: None,
    alignment: 8,
};


