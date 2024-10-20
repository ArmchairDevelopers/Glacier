use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct LevelWebBrowserDescriptionComponent {
    pub _glacier_base: super::gameplay_sim::LevelDescriptionComponent,
    pub create_web_browser: super::core::PlatformScalableBool,
}

pub trait LevelWebBrowserDescriptionComponentTrait: super::gameplay_sim::LevelDescriptionComponentTrait {
    fn create_web_browser(&self) -> &super::core::PlatformScalableBool;
}

impl LevelWebBrowserDescriptionComponentTrait for LevelWebBrowserDescriptionComponent {
    fn create_web_browser(&self) -> &super::core::PlatformScalableBool {
        &self.create_web_browser
    }
}

impl super::gameplay_sim::LevelDescriptionComponentTrait for LevelWebBrowserDescriptionComponent {
}

impl super::core::DataContainerTrait for LevelWebBrowserDescriptionComponent {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static LEVELWEBBROWSERDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelWebBrowserDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::gameplay_sim::LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LevelWebBrowserDescriptionComponent as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CreateWebBrowser",
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
}


pub static LEVELWEBBROWSERDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelWebBrowserDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("LevelWebBrowserDescriptionComponent"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    pub web_browser_bundle: Option<Arc<Mutex<dyn WebBrowserBundleAssetTrait>>>,
    pub per_level_web_browser_enable: bool,
}

pub trait WebBrowserSettingsTrait: super::core::SystemSettingsTrait {
    fn application_name(&self) -> &String;
    fn browser_heap_size(&self) -> &i32;
    fn allocate_heap_on_module_startup(&self) -> &bool;
    fn standard_font(&self) -> &String;
    fn serif_font(&self) -> &String;
    fn sans_serif_font(&self) -> &String;
    fn monospace_font(&self) -> &String;
    fn cursive_font(&self) -> &String;
    fn fantasy_font(&self) -> &String;
    fn system_font(&self) -> &String;
    fn system_font_bold(&self) -> &bool;
    fn default_c_s_s(&self) -> &String;
    fn web_browser_bundle(&self) -> &Option<Arc<Mutex<dyn WebBrowserBundleAssetTrait>>>;
    fn per_level_web_browser_enable(&self) -> &bool;
}

impl WebBrowserSettingsTrait for WebBrowserSettings {
    fn application_name(&self) -> &String {
        &self.application_name
    }
    fn browser_heap_size(&self) -> &i32 {
        &self.browser_heap_size
    }
    fn allocate_heap_on_module_startup(&self) -> &bool {
        &self.allocate_heap_on_module_startup
    }
    fn standard_font(&self) -> &String {
        &self.standard_font
    }
    fn serif_font(&self) -> &String {
        &self.serif_font
    }
    fn sans_serif_font(&self) -> &String {
        &self.sans_serif_font
    }
    fn monospace_font(&self) -> &String {
        &self.monospace_font
    }
    fn cursive_font(&self) -> &String {
        &self.cursive_font
    }
    fn fantasy_font(&self) -> &String {
        &self.fantasy_font
    }
    fn system_font(&self) -> &String {
        &self.system_font
    }
    fn system_font_bold(&self) -> &bool {
        &self.system_font_bold
    }
    fn default_c_s_s(&self) -> &String {
        &self.default_c_s_s
    }
    fn web_browser_bundle(&self) -> &Option<Arc<Mutex<dyn WebBrowserBundleAssetTrait>>> {
        &self.web_browser_bundle
    }
    fn per_level_web_browser_enable(&self) -> &bool {
        &self.per_level_web_browser_enable
    }
}

impl super::core::SystemSettingsTrait for WebBrowserSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
}

impl super::core::DataContainerTrait for WebBrowserSettings {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WEBBROWSERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserSettings",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WebBrowserSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ApplicationName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, application_name),
            },
            FieldInfoData {
                name: "BrowserHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(WebBrowserSettings, browser_heap_size),
            },
            FieldInfoData {
                name: "AllocateHeapOnModuleStartup",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WebBrowserSettings, allocate_heap_on_module_startup),
            },
            FieldInfoData {
                name: "StandardFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, standard_font),
            },
            FieldInfoData {
                name: "SerifFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, serif_font),
            },
            FieldInfoData {
                name: "SansSerifFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, sans_serif_font),
            },
            FieldInfoData {
                name: "MonospaceFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, monospace_font),
            },
            FieldInfoData {
                name: "CursiveFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, cursive_font),
            },
            FieldInfoData {
                name: "FantasyFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, fantasy_font),
            },
            FieldInfoData {
                name: "SystemFont",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, system_font),
            },
            FieldInfoData {
                name: "SystemFontBold",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WebBrowserSettings, system_font_bold),
            },
            FieldInfoData {
                name: "DefaultCSS",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserSettings, default_c_s_s),
            },
            FieldInfoData {
                name: "WebBrowserBundle",
                flags: MemberInfoFlags::new(0),
                field_type: "WebBrowserBundleAsset",
                rust_offset: offset_of!(WebBrowserSettings, web_browser_bundle),
            },
            FieldInfoData {
                name: "PerLevelWebBrowserEnable",
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
}


pub static WEBBROWSERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIWebViewNotificationErrorMessage {
}

pub trait UIWebViewNotificationErrorMessageTrait: TypeObject {
}

impl UIWebViewNotificationErrorMessageTrait for UIWebViewNotificationErrorMessage {
}

pub static UIWEBVIEWNOTIFICATIONERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewNotificationErrorMessage",
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewNotificationErrorMessage as Default>::default())),
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
}


pub static WEBVIEWERROR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebViewError-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebViewError"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIWebViewNotificationUrlChangedMessage {
}

pub trait UIWebViewNotificationUrlChangedMessageTrait: TypeObject {
}

impl UIWebViewNotificationUrlChangedMessageTrait for UIWebViewNotificationUrlChangedMessage {
}

pub static UIWEBVIEWNOTIFICATIONURLCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewNotificationUrlChangedMessage",
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewNotificationUrlChangedMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct UIWebViewRequestCloseViewMessage {
}

pub trait UIWebViewRequestCloseViewMessageTrait: TypeObject {
}

impl UIWebViewRequestCloseViewMessageTrait for UIWebViewRequestCloseViewMessage {
}

pub static UIWEBVIEWREQUESTCLOSEVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewRequestCloseViewMessage",
    flags: MemberInfoFlags::new(36937),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewRequestCloseViewMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
pub struct UIWebViewRequestViewPageMessage {
}

pub trait UIWebViewRequestViewPageMessageTrait: TypeObject {
}

impl UIWebViewRequestViewPageMessageTrait for UIWebViewRequestViewPageMessage {
}

pub static UIWEBVIEWREQUESTVIEWPAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewRequestViewPageMessage",
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewRequestViewPageMessage as Default>::default())),
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
}

#[derive(Clone, Debug, Default)]
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
    fn layers(&self) -> &Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementLayerEntityDataTrait>>>> {
        self._glacier_base.layers()
    }
    fn texture_mappings(&self) -> &Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UITextureMappingAssetTrait>>>> {
        self._glacier_base.texture_mappings()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
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
}

impl super::core::GameDataContainerTrait for UIWebViewWidgetData {
}

impl super::core::DataContainerTrait for UIWebViewWidgetData {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static UIWEBVIEWWIDGETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidgetData",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIWIDGETENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewWidgetData as Default>::default())),
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
}


pub static UIWEBVIEWWIDGETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidgetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("UIWebViewWidgetData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WebBrowserBundleAsset {
    pub _glacier_base: super::core::Asset,
    pub bundle_path: String,
    pub fonts: Vec<String>,
    pub local_u_r_ls: Vec<String>,
}

pub trait WebBrowserBundleAssetTrait: super::core::AssetTrait {
    fn bundle_path(&self) -> &String;
    fn fonts(&self) -> &Vec<String>;
    fn local_u_r_ls(&self) -> &Vec<String>;
}

impl WebBrowserBundleAssetTrait for WebBrowserBundleAsset {
    fn bundle_path(&self) -> &String {
        &self.bundle_path
    }
    fn fonts(&self) -> &Vec<String> {
        &self.fonts
    }
    fn local_u_r_ls(&self) -> &Vec<String> {
        &self.local_u_r_ls
    }
}

impl super::core::AssetTrait for WebBrowserBundleAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for WebBrowserBundleAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WEBBROWSERBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserBundleAsset",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WebBrowserBundleAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "BundlePath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserBundleAsset, bundle_path),
            },
            FieldInfoData {
                name: "Fonts",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(WebBrowserBundleAsset, fonts),
            },
            FieldInfoData {
                name: "LocalURLs",
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
}


pub static WEBBROWSERBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserBundleAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserBundleAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WebBrowserLocalURLAsset {
    pub _glacier_base: super::core::Asset,
    pub local_path: String,
    pub file: Option<Arc<Mutex<dyn super::game_shared::RawFileDataAssetTrait>>>,
}

pub trait WebBrowserLocalURLAssetTrait: super::core::AssetTrait {
    fn local_path(&self) -> &String;
    fn file(&self) -> &Option<Arc<Mutex<dyn super::game_shared::RawFileDataAssetTrait>>>;
}

impl WebBrowserLocalURLAssetTrait for WebBrowserLocalURLAsset {
    fn local_path(&self) -> &String {
        &self.local_path
    }
    fn file(&self) -> &Option<Arc<Mutex<dyn super::game_shared::RawFileDataAssetTrait>>> {
        &self.file
    }
}

impl super::core::AssetTrait for WebBrowserLocalURLAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for WebBrowserLocalURLAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static WEBBROWSERLOCALURLASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserLocalURLAsset",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WebBrowserLocalURLAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "LocalPath",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(WebBrowserLocalURLAsset, local_path),
            },
            FieldInfoData {
                name: "File",
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
}


pub static WEBBROWSERLOCALURLASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserLocalURLAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserLocalURLAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
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
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_client_u_i::UIWIDGETENTITY_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIWebViewWidget as Default>::default())),
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
}


pub static UIWEBVIEWWIDGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidget-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("UIWebViewWidget"),
    array_type: None,
    alignment: 8,
};


