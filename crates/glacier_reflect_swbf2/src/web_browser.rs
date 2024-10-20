use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LevelWebBrowserDescriptionComponent {
    pub create_web_browser: super::core::PlatformScalableBool,
}

pub const LEVELWEBBROWSERDESCRIPTIONCOMPONENT_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelWebBrowserDescriptionComponent",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LEVELDESCRIPTIONCOMPONENT_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CreateWebBrowser",
                flags: MemberInfoFlags::new(0),
                field_type: PLATFORMSCALABLEBOOL_TYPE_INFO,
                rust_offset: offset_of!(LevelWebBrowserDescriptionComponent, create_web_browser),
            },
        ],
    }),
    array_type: Some(LEVELWEBBROWSERDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LevelWebBrowserDescriptionComponent {
    fn type_info() -> &'static TypeInfo {
        LEVELWEBBROWSERDESCRIPTIONCOMPONENT_TYPE_INFO
    }
}


pub const LEVELWEBBROWSERDESCRIPTIONCOMPONENT_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LevelWebBrowserDescriptionComponent-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("LevelWebBrowserDescriptionComponent-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WebBrowserSettings {
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
    pub web_browser_bundle: WebBrowserBundleAsset,
    pub per_level_web_browser_enable: bool,
}

pub const WEBBROWSERSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserSettings",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ApplicationName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, application_name),
            },
            FieldInfoData {
                name: "BrowserHeapSize",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, browser_heap_size),
            },
            FieldInfoData {
                name: "AllocateHeapOnModuleStartup",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, allocate_heap_on_module_startup),
            },
            FieldInfoData {
                name: "StandardFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, standard_font),
            },
            FieldInfoData {
                name: "SerifFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, serif_font),
            },
            FieldInfoData {
                name: "SansSerifFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, sans_serif_font),
            },
            FieldInfoData {
                name: "MonospaceFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, monospace_font),
            },
            FieldInfoData {
                name: "CursiveFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, cursive_font),
            },
            FieldInfoData {
                name: "FantasyFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, fantasy_font),
            },
            FieldInfoData {
                name: "SystemFont",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, system_font),
            },
            FieldInfoData {
                name: "SystemFontBold",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, system_font_bold),
            },
            FieldInfoData {
                name: "DefaultCSS",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, default_c_s_s),
            },
            FieldInfoData {
                name: "WebBrowserBundle",
                flags: MemberInfoFlags::new(0),
                field_type: WEBBROWSERBUNDLEASSET_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, web_browser_bundle),
            },
            FieldInfoData {
                name: "PerLevelWebBrowserEnable",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserSettings, per_level_web_browser_enable),
            },
        ],
    }),
    array_type: Some(WEBBROWSERSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WebBrowserSettings {
    fn type_info() -> &'static TypeInfo {
        WEBBROWSERSETTINGS_TYPE_INFO
    }
}


pub const WEBBROWSERSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWebViewNotificationErrorMessage {
}

pub const UIWEBVIEWNOTIFICATIONERRORMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewNotificationErrorMessage",
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewNotificationErrorMessage {
    fn type_info() -> &'static TypeInfo {
        UIWEBVIEWNOTIFICATIONERRORMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WebViewError {
    #[default]
    WEBVIEW_ERR_TIMEOUT = 0,
    WEBVIEW_ERR_SSLCERT = 1,
    WEBVIEW_ERR_SERVER = 2,
    WEBVIEW_ERR_UNEXPECTED_RESPONSE = 3,
    WEBVIEW_ERR_UNKNOWN = 4,
}

pub const WEBVIEWERROR_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebViewError",
    flags: MemberInfoFlags::new(49429),
    module: "WebBrowser",
    data: TypeInfoData::Enum,
    array_type: Some(WEBVIEWERROR_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WebViewError {
    fn type_info() -> &'static TypeInfo {
        WEBVIEWERROR_TYPE_INFO
    }
}


pub const WEBVIEWERROR_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebViewError-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebViewError-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWebViewNotificationUrlChangedMessage {
}

pub const UIWEBVIEWNOTIFICATIONURLCHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewNotificationUrlChangedMessage",
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewNotificationUrlChangedMessage {
    fn type_info() -> &'static TypeInfo {
        UIWEBVIEWNOTIFICATIONURLCHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWebViewRequestCloseViewMessage {
}

pub const UIWEBVIEWREQUESTCLOSEVIEWMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewRequestCloseViewMessage",
    flags: MemberInfoFlags::new(36937),
    module: "WebBrowser",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewRequestCloseViewMessage {
    fn type_info() -> &'static TypeInfo {
        UIWEBVIEWREQUESTCLOSEVIEWMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWebViewRequestViewPageMessage {
}

pub const UIWEBVIEWREQUESTVIEWPAGEMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewRequestViewPageMessage",
    flags: MemberInfoFlags::new(73),
    module: "WebBrowser",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for UIWebViewRequestViewPageMessage {
    fn type_info() -> &'static TypeInfo {
        UIWEBVIEWREQUESTVIEWPAGEMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct UIWebViewWidgetData {
}

pub const UIWEBVIEWWIDGETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidgetData",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIWIDGETENTITYDATA_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIWEBVIEWWIDGETDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIWebViewWidgetData {
    fn type_info() -> &'static TypeInfo {
        UIWEBVIEWWIDGETDATA_TYPE_INFO
    }
}


pub const UIWEBVIEWWIDGETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidgetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("UIWebViewWidgetData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WebBrowserBundleAsset {
    pub bundle_path: String,
    pub fonts: Vec<String>,
    pub local_u_r_ls: Vec<String>,
}

pub const WEBBROWSERBUNDLEASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserBundleAsset",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "BundlePath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserBundleAsset, bundle_path),
            },
            FieldInfoData {
                name: "Fonts",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserBundleAsset, fonts),
            },
            FieldInfoData {
                name: "LocalURLs",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserBundleAsset, local_u_r_ls),
            },
        ],
    }),
    array_type: Some(WEBBROWSERBUNDLEASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WebBrowserBundleAsset {
    fn type_info() -> &'static TypeInfo {
        WEBBROWSERBUNDLEASSET_TYPE_INFO
    }
}


pub const WEBBROWSERBUNDLEASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserBundleAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserBundleAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WebBrowserLocalURLAsset {
    pub local_path: String,
    pub file: super::game_shared::RawFileDataAsset,
}

pub const WEBBROWSERLOCALURLASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserLocalURLAsset",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "LocalPath",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserLocalURLAsset, local_path),
            },
            FieldInfoData {
                name: "File",
                flags: MemberInfoFlags::new(0),
                field_type: RAWFILEDATAASSET_TYPE_INFO,
                rust_offset: offset_of!(WebBrowserLocalURLAsset, file),
            },
        ],
    }),
    array_type: Some(WEBBROWSERLOCALURLASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WebBrowserLocalURLAsset {
    fn type_info() -> &'static TypeInfo {
        WEBBROWSERLOCALURLASSET_TYPE_INFO
    }
}


pub const WEBBROWSERLOCALURLASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WebBrowserLocalURLAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("WebBrowserLocalURLAsset-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UIWebViewWidget {
}

pub const UIWEBVIEWWIDGET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidget",
    flags: MemberInfoFlags::new(101),
    module: "WebBrowser",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UIWIDGETENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(UIWEBVIEWWIDGET_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for UIWebViewWidget {
    fn type_info() -> &'static TypeInfo {
        UIWEBVIEWWIDGET_TYPE_INFO
    }
}


pub const UIWEBVIEWWIDGET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIWebViewWidget-Array",
    flags: MemberInfoFlags::new(145),
    module: "WebBrowser",
    data: TypeInfoData::Array("UIWebViewWidget-Array"),
    array_type: None,
    alignment: 8,
};


