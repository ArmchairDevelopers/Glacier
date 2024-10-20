use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_app_types(registry: &mut TypeRegistry) {
    registry.register_type(INPUTCONFIGURATION_TYPE_INFO);
    registry.register_type(INPUTCONFIGURATION_ARRAY_TYPE_INFO);
    registry.register_type(INPUTSET_TYPE_INFO);
    registry.register_type(INPUTSET_ARRAY_TYPE_INFO);
    registry.register_type(INPUTBINDING_TYPE_INFO);
    registry.register_type(INPUTBINDING_ARRAY_TYPE_INFO);
    registry.register_type(WINDOWSETTINGS_TYPE_INFO);
    registry.register_type(WINDOWSETTINGS_ARRAY_TYPE_INFO);
    registry.register_type(APPLICATIONWINDOWFULLSCREENREQUESTMESSAGE_TYPE_INFO);
    registry.register_type(APPLICATIONWINDOWRESIZEENDMESSAGE_TYPE_INFO);
    registry.register_type(APPLICATIONWINDOWSTYLECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(APPLICATIONWINDOWCLOSEDMESSAGE_TYPE_INFO);
    registry.register_type(APPLICATIONWINDOWCREATEDMESSAGE_TYPE_INFO);
    registry.register_type(WINDOWFULLSCREENMODE_TYPE_INFO);
    registry.register_type(WINDOWFULLSCREENMODE_ARRAY_TYPE_INFO);
    registry.register_type(WINDOWRESIZETYPE_TYPE_INFO);
    registry.register_type(WINDOWRESIZETYPE_ARRAY_TYPE_INFO);
    registry.register_type(WINDOW_TYPE_INFO);
    registry.register_type(WINDOW_ARRAY_TYPE_INFO);
    registry.register_type(WIN32WINDOW_TYPE_INFO);
    registry.register_type(WIN32WINDOW_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputConfiguration {
    pub custom_input_sets: Vec<InputSet>,
}

pub const INPUTCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConfiguration",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CustomInputSets",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTSET_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InputConfiguration, custom_input_sets),
            },
        ],
    }),
    array_type: Some(INPUTCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputConfiguration {
    fn type_info() -> &'static TypeInfo {
        INPUTCONFIGURATION_TYPE_INFO
    }
}


pub const INPUTCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("InputConfiguration-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputSet {
    pub configuration_key: String,
    pub bindings: Vec<InputBinding>,
}

pub const INPUTSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSet",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "ConfigurationKey",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(InputSet, configuration_key),
            },
            FieldInfoData {
                name: "Bindings",
                flags: MemberInfoFlags::new(144),
                field_type: INPUTBINDING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InputSet, bindings),
            },
        ],
    }),
    array_type: Some(INPUTSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputSet {
    fn type_info() -> &'static TypeInfo {
        INPUTSET_TYPE_INFO
    }
}


pub const INPUTSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSet-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("InputSet-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputBinding {
    pub input_gesture: String,
    pub command_uri: String,
}

pub const INPUTBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputBinding",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(DATACONTAINER_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "InputGesture",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(InputBinding, input_gesture),
            },
            FieldInfoData {
                name: "CommandUri",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(InputBinding, command_uri),
            },
        ],
    }),
    array_type: Some(INPUTBINDING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputBinding {
    fn type_info() -> &'static TypeInfo {
        INPUTBINDING_TYPE_INFO
    }
}


pub const INPUTBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("InputBinding-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WindowSettings {
    pub pos_x: i32,
    pub pos_y: i32,
    pub width: u32,
    pub height: u32,
    pub auto_size: bool,
    pub fullscreen_auto_size: bool,
    pub enable_escape: bool,
    pub enable_input_on_activate: bool,
    pub hibernate_on_close: bool,
    pub hidden: bool,
    pub minimized: bool,
    pub allow_windows_larger_than_desktop: bool,
}

pub const WINDOWSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowSettings",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(SYSTEMSETTINGS_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "PosX",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, pos_x),
            },
            FieldInfoData {
                name: "PosY",
                flags: MemberInfoFlags::new(8192),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, pos_y),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(8192),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(8192),
                field_type: UINT32_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, height),
            },
            FieldInfoData {
                name: "AutoSize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, auto_size),
            },
            FieldInfoData {
                name: "FullscreenAutoSize",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, fullscreen_auto_size),
            },
            FieldInfoData {
                name: "EnableEscape",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, enable_escape),
            },
            FieldInfoData {
                name: "EnableInputOnActivate",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, enable_input_on_activate),
            },
            FieldInfoData {
                name: "HibernateOnClose",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, hibernate_on_close),
            },
            FieldInfoData {
                name: "Hidden",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, hidden),
            },
            FieldInfoData {
                name: "Minimized",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, minimized),
            },
            FieldInfoData {
                name: "AllowWindowsLargerThanDesktop",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(WindowSettings, allow_windows_larger_than_desktop),
            },
        ],
    }),
    array_type: Some(WINDOWSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WindowSettings {
    fn type_info() -> &'static TypeInfo {
        WINDOWSETTINGS_TYPE_INFO
    }
}


pub const WINDOWSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("WindowSettings-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplicationWindowFullscreenRequestMessage {
}

pub const APPLICATIONWINDOWFULLSCREENREQUESTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowFullscreenRequestMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowFullscreenRequestMessage {
    fn type_info() -> &'static TypeInfo {
        APPLICATIONWINDOWFULLSCREENREQUESTMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplicationWindowResizeEndMessage {
}

pub const APPLICATIONWINDOWRESIZEENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowResizeEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowResizeEndMessage {
    fn type_info() -> &'static TypeInfo {
        APPLICATIONWINDOWRESIZEENDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplicationWindowStyleChangedMessage {
}

pub const APPLICATIONWINDOWSTYLECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowStyleChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowStyleChangedMessage {
    fn type_info() -> &'static TypeInfo {
        APPLICATIONWINDOWSTYLECHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplicationWindowClosedMessage {
}

pub const APPLICATIONWINDOWCLOSEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowClosedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowClosedMessage {
    fn type_info() -> &'static TypeInfo {
        APPLICATIONWINDOWCLOSEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ApplicationWindowCreatedMessage {
}

pub const APPLICATIONWINDOWCREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowCreatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowCreatedMessage {
    fn type_info() -> &'static TypeInfo {
        APPLICATIONWINDOWCREATEDMESSAGE_TYPE_INFO
    }
}

#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WindowFullscreenMode {
    #[default]
    WindowFullscreenMode_Windowed = 0,
    WindowFullscreenMode_Fullscreen = 1,
    WindowFullscreenMode_FullscreenBorderless = 2,
}

pub const WINDOWFULLSCREENMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowFullscreenMode",
    flags: MemberInfoFlags::new(49429),
    module: "App",
    data: TypeInfoData::Enum,
    array_type: Some(WINDOWFULLSCREENMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WindowFullscreenMode {
    fn type_info() -> &'static TypeInfo {
        WINDOWFULLSCREENMODE_TYPE_INFO
    }
}


pub const WINDOWFULLSCREENMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowFullscreenMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("WindowFullscreenMode-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum WindowResizeType {
    #[default]
    WindowResizeType_Minimized = 0,
    WindowResizeType_Maxmized = 1,
    WindowResizeType_Restored = 2,
    WindowResizeType_Drag = 3,
}

pub const WINDOWRESIZETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowResizeType",
    flags: MemberInfoFlags::new(49429),
    module: "App",
    data: TypeInfoData::Enum,
    array_type: Some(WINDOWRESIZETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WindowResizeType {
    fn type_info() -> &'static TypeInfo {
        WINDOWRESIZETYPE_TYPE_INFO
    }
}


pub const WINDOWRESIZETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowResizeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("WindowResizeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Window {
}

pub const WINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Window",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(WINDOW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Window {
    fn type_info() -> &'static TypeInfo {
        WINDOW_TYPE_INFO
    }
}


pub const WINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Window-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("Window-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Win32Window {
}

pub const WIN32WINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32Window",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDOW_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(WIN32WINDOW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Win32Window {
    fn type_info() -> &'static TypeInfo {
        WIN32WINDOW_TYPE_INFO
    }
}


pub const WIN32WINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32Window-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("Win32Window-Array"),
    array_type: None,
    alignment: 8,
};


