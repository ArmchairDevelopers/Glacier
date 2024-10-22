use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
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

#[derive(Clone, Debug, Default)]
pub struct InputConfiguration {
    pub _glacier_base: super::core::DataContainer,
    pub custom_input_sets: Vec<Option<Arc<Mutex<dyn InputSetTrait>>>>,
}

pub trait InputConfigurationTrait: super::core::DataContainerTrait {
    fn custom_input_sets(&self) -> &Vec<Option<Arc<Mutex<dyn InputSetTrait>>>>;
    fn custom_input_sets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputSetTrait>>>>;
}

impl InputConfigurationTrait for InputConfiguration {
    fn custom_input_sets(&self) -> &Vec<Option<Arc<Mutex<dyn InputSetTrait>>>> {
        &self.custom_input_sets
    }
    fn custom_input_sets_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputSetTrait>>>> {
        &mut self.custom_input_sets
    }
}

impl super::core::DataContainerTrait for InputConfiguration {
}

pub static INPUTCONFIGURATION_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConfiguration",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputConfiguration as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "CustomInputSets",
                flags: MemberInfoFlags::new(144),
                field_type: "InputSet-Array",
                rust_offset: offset_of!(InputConfiguration, custom_input_sets),
            },
        ],
    }),
    array_type: Some(INPUTCONFIGURATION_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputConfiguration {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTCONFIGURATION_TYPE_INFO
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


pub static INPUTCONFIGURATION_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputConfiguration-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("InputConfiguration"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputSet {
    pub _glacier_base: super::core::DataContainer,
    pub configuration_key: String,
    pub bindings: Vec<Option<Arc<Mutex<dyn InputBindingTrait>>>>,
}

pub trait InputSetTrait: super::core::DataContainerTrait {
    fn configuration_key(&self) -> &String;
    fn configuration_key_mut(&mut self) -> &mut String;
    fn bindings(&self) -> &Vec<Option<Arc<Mutex<dyn InputBindingTrait>>>>;
    fn bindings_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputBindingTrait>>>>;
}

impl InputSetTrait for InputSet {
    fn configuration_key(&self) -> &String {
        &self.configuration_key
    }
    fn configuration_key_mut(&mut self) -> &mut String {
        &mut self.configuration_key
    }
    fn bindings(&self) -> &Vec<Option<Arc<Mutex<dyn InputBindingTrait>>>> {
        &self.bindings
    }
    fn bindings_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn InputBindingTrait>>>> {
        &mut self.bindings
    }
}

impl super::core::DataContainerTrait for InputSet {
}

pub static INPUTSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSet",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputSet as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ConfigurationKey",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(InputSet, configuration_key),
            },
            FieldInfoData {
                name: "Bindings",
                flags: MemberInfoFlags::new(144),
                field_type: "InputBinding-Array",
                rust_offset: offset_of!(InputSet, bindings),
            },
        ],
    }),
    array_type: Some(INPUTSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputSet {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTSET_TYPE_INFO
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


pub static INPUTSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSet-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("InputSet"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputBinding {
    pub _glacier_base: super::core::DataContainer,
    pub input_gesture: String,
    pub command_uri: String,
}

pub trait InputBindingTrait: super::core::DataContainerTrait {
    fn input_gesture(&self) -> &String;
    fn input_gesture_mut(&mut self) -> &mut String;
    fn command_uri(&self) -> &String;
    fn command_uri_mut(&mut self) -> &mut String;
}

impl InputBindingTrait for InputBinding {
    fn input_gesture(&self) -> &String {
        &self.input_gesture
    }
    fn input_gesture_mut(&mut self) -> &mut String {
        &mut self.input_gesture
    }
    fn command_uri(&self) -> &String {
        &self.command_uri
    }
    fn command_uri_mut(&mut self) -> &mut String {
        &mut self.command_uri
    }
}

impl super::core::DataContainerTrait for InputBinding {
}

pub static INPUTBINDING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputBinding",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputBinding as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InputGesture",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(InputBinding, input_gesture),
            },
            FieldInfoData {
                name: "CommandUri",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(InputBinding, command_uri),
            },
        ],
    }),
    array_type: Some(INPUTBINDING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputBinding {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTBINDING_TYPE_INFO
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


pub static INPUTBINDING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputBinding-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("InputBinding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct WindowSettings {
    pub _glacier_base: super::core::SystemSettings,
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

pub trait WindowSettingsTrait: super::core::SystemSettingsTrait {
    fn pos_x(&self) -> &i32;
    fn pos_x_mut(&mut self) -> &mut i32;
    fn pos_y(&self) -> &i32;
    fn pos_y_mut(&mut self) -> &mut i32;
    fn width(&self) -> &u32;
    fn width_mut(&mut self) -> &mut u32;
    fn height(&self) -> &u32;
    fn height_mut(&mut self) -> &mut u32;
    fn auto_size(&self) -> &bool;
    fn auto_size_mut(&mut self) -> &mut bool;
    fn fullscreen_auto_size(&self) -> &bool;
    fn fullscreen_auto_size_mut(&mut self) -> &mut bool;
    fn enable_escape(&self) -> &bool;
    fn enable_escape_mut(&mut self) -> &mut bool;
    fn enable_input_on_activate(&self) -> &bool;
    fn enable_input_on_activate_mut(&mut self) -> &mut bool;
    fn hibernate_on_close(&self) -> &bool;
    fn hibernate_on_close_mut(&mut self) -> &mut bool;
    fn hidden(&self) -> &bool;
    fn hidden_mut(&mut self) -> &mut bool;
    fn minimized(&self) -> &bool;
    fn minimized_mut(&mut self) -> &mut bool;
    fn allow_windows_larger_than_desktop(&self) -> &bool;
    fn allow_windows_larger_than_desktop_mut(&mut self) -> &mut bool;
}

impl WindowSettingsTrait for WindowSettings {
    fn pos_x(&self) -> &i32 {
        &self.pos_x
    }
    fn pos_x_mut(&mut self) -> &mut i32 {
        &mut self.pos_x
    }
    fn pos_y(&self) -> &i32 {
        &self.pos_y
    }
    fn pos_y_mut(&mut self) -> &mut i32 {
        &mut self.pos_y
    }
    fn width(&self) -> &u32 {
        &self.width
    }
    fn width_mut(&mut self) -> &mut u32 {
        &mut self.width
    }
    fn height(&self) -> &u32 {
        &self.height
    }
    fn height_mut(&mut self) -> &mut u32 {
        &mut self.height
    }
    fn auto_size(&self) -> &bool {
        &self.auto_size
    }
    fn auto_size_mut(&mut self) -> &mut bool {
        &mut self.auto_size
    }
    fn fullscreen_auto_size(&self) -> &bool {
        &self.fullscreen_auto_size
    }
    fn fullscreen_auto_size_mut(&mut self) -> &mut bool {
        &mut self.fullscreen_auto_size
    }
    fn enable_escape(&self) -> &bool {
        &self.enable_escape
    }
    fn enable_escape_mut(&mut self) -> &mut bool {
        &mut self.enable_escape
    }
    fn enable_input_on_activate(&self) -> &bool {
        &self.enable_input_on_activate
    }
    fn enable_input_on_activate_mut(&mut self) -> &mut bool {
        &mut self.enable_input_on_activate
    }
    fn hibernate_on_close(&self) -> &bool {
        &self.hibernate_on_close
    }
    fn hibernate_on_close_mut(&mut self) -> &mut bool {
        &mut self.hibernate_on_close
    }
    fn hidden(&self) -> &bool {
        &self.hidden
    }
    fn hidden_mut(&mut self) -> &mut bool {
        &mut self.hidden
    }
    fn minimized(&self) -> &bool {
        &self.minimized
    }
    fn minimized_mut(&mut self) -> &mut bool {
        &mut self.minimized
    }
    fn allow_windows_larger_than_desktop(&self) -> &bool {
        &self.allow_windows_larger_than_desktop
    }
    fn allow_windows_larger_than_desktop_mut(&mut self) -> &mut bool {
        &mut self.allow_windows_larger_than_desktop
    }
}

impl super::core::SystemSettingsTrait for WindowSettings {
    fn platform(&self) -> &super::core::GamePlatform {
        self._glacier_base.platform()
    }
    fn platform_mut(&mut self) -> &mut super::core::GamePlatform {
        self._glacier_base.platform_mut()
    }
}

impl super::core::DataContainerTrait for WindowSettings {
}

pub static WINDOWSETTINGS_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowSettings",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::SYSTEMSETTINGS_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<WindowSettings as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "PosX",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(WindowSettings, pos_x),
            },
            FieldInfoData {
                name: "PosY",
                flags: MemberInfoFlags::new(8192),
                field_type: "Int32",
                rust_offset: offset_of!(WindowSettings, pos_y),
            },
            FieldInfoData {
                name: "Width",
                flags: MemberInfoFlags::new(8192),
                field_type: "Uint32",
                rust_offset: offset_of!(WindowSettings, width),
            },
            FieldInfoData {
                name: "Height",
                flags: MemberInfoFlags::new(8192),
                field_type: "Uint32",
                rust_offset: offset_of!(WindowSettings, height),
            },
            FieldInfoData {
                name: "AutoSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, auto_size),
            },
            FieldInfoData {
                name: "FullscreenAutoSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, fullscreen_auto_size),
            },
            FieldInfoData {
                name: "EnableEscape",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, enable_escape),
            },
            FieldInfoData {
                name: "EnableInputOnActivate",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, enable_input_on_activate),
            },
            FieldInfoData {
                name: "HibernateOnClose",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, hibernate_on_close),
            },
            FieldInfoData {
                name: "Hidden",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, hidden),
            },
            FieldInfoData {
                name: "Minimized",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, minimized),
            },
            FieldInfoData {
                name: "AllowWindowsLargerThanDesktop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(WindowSettings, allow_windows_larger_than_desktop),
            },
        ],
    }),
    array_type: Some(WINDOWSETTINGS_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for WindowSettings {
    fn type_info(&self) -> &'static TypeInfo {
        WINDOWSETTINGS_TYPE_INFO
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


pub static WINDOWSETTINGS_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowSettings-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("WindowSettings"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ApplicationWindowFullscreenRequestMessage {
}

pub trait ApplicationWindowFullscreenRequestMessageTrait: TypeObject {
}

impl ApplicationWindowFullscreenRequestMessageTrait for ApplicationWindowFullscreenRequestMessage {
}

pub static APPLICATIONWINDOWFULLSCREENREQUESTMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowFullscreenRequestMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ApplicationWindowFullscreenRequestMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowFullscreenRequestMessage {
    fn type_info(&self) -> &'static TypeInfo {
        APPLICATIONWINDOWFULLSCREENREQUESTMESSAGE_TYPE_INFO
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
pub struct ApplicationWindowResizeEndMessage {
}

pub trait ApplicationWindowResizeEndMessageTrait: TypeObject {
}

impl ApplicationWindowResizeEndMessageTrait for ApplicationWindowResizeEndMessage {
}

pub static APPLICATIONWINDOWRESIZEENDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowResizeEndMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ApplicationWindowResizeEndMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowResizeEndMessage {
    fn type_info(&self) -> &'static TypeInfo {
        APPLICATIONWINDOWRESIZEENDMESSAGE_TYPE_INFO
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
pub struct ApplicationWindowStyleChangedMessage {
}

pub trait ApplicationWindowStyleChangedMessageTrait: TypeObject {
}

impl ApplicationWindowStyleChangedMessageTrait for ApplicationWindowStyleChangedMessage {
}

pub static APPLICATIONWINDOWSTYLECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowStyleChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ApplicationWindowStyleChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowStyleChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        APPLICATIONWINDOWSTYLECHANGEDMESSAGE_TYPE_INFO
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
pub struct ApplicationWindowClosedMessage {
}

pub trait ApplicationWindowClosedMessageTrait: TypeObject {
}

impl ApplicationWindowClosedMessageTrait for ApplicationWindowClosedMessage {
}

pub static APPLICATIONWINDOWCLOSEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowClosedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ApplicationWindowClosedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowClosedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        APPLICATIONWINDOWCLOSEDMESSAGE_TYPE_INFO
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
pub struct ApplicationWindowCreatedMessage {
}

pub trait ApplicationWindowCreatedMessageTrait: TypeObject {
}

impl ApplicationWindowCreatedMessageTrait for ApplicationWindowCreatedMessage {
}

pub static APPLICATIONWINDOWCREATEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ApplicationWindowCreatedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "App",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ApplicationWindowCreatedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for ApplicationWindowCreatedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        APPLICATIONWINDOWCREATEDMESSAGE_TYPE_INFO
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
pub enum WindowFullscreenMode {
    #[default]
    WindowFullscreenMode_Windowed = 0,
    WindowFullscreenMode_Fullscreen = 1,
    WindowFullscreenMode_FullscreenBorderless = 2,
}

pub static WINDOWFULLSCREENMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowFullscreenMode",
    flags: MemberInfoFlags::new(49429),
    module: "App",
    data: TypeInfoData::Enum,
    array_type: Some(WINDOWFULLSCREENMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WindowFullscreenMode {
    fn type_info(&self) -> &'static TypeInfo {
        WINDOWFULLSCREENMODE_TYPE_INFO
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


pub static WINDOWFULLSCREENMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowFullscreenMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("WindowFullscreenMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum WindowResizeType {
    #[default]
    WindowResizeType_Minimized = 0,
    WindowResizeType_Maxmized = 1,
    WindowResizeType_Restored = 2,
    WindowResizeType_Drag = 3,
}

pub static WINDOWRESIZETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowResizeType",
    flags: MemberInfoFlags::new(49429),
    module: "App",
    data: TypeInfoData::Enum,
    array_type: Some(WINDOWRESIZETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for WindowResizeType {
    fn type_info(&self) -> &'static TypeInfo {
        WINDOWRESIZETYPE_TYPE_INFO
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


pub static WINDOWRESIZETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "WindowResizeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("WindowResizeType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Window {
}

pub trait WindowTrait: TypeObject {
}

impl WindowTrait for Window {
}

pub static WINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Window",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Window as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WINDOW_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for Window {
    fn type_info(&self) -> &'static TypeInfo {
        WINDOW_TYPE_INFO
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


pub static WINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Window-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("Window"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Win32Window {
    pub _glacier_base: Window,
}

pub trait Win32WindowTrait: WindowTrait {
}

impl Win32WindowTrait for Win32Window {
}

impl WindowTrait for Win32Window {
}

pub static WIN32WINDOW_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32Window",
    flags: MemberInfoFlags::new(101),
    module: "App",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(WINDOW_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Win32Window as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(WIN32WINDOW_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Win32Window {
    fn type_info(&self) -> &'static TypeInfo {
        WIN32WINDOW_TYPE_INFO
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


pub static WIN32WINDOW_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Win32Window-Array",
    flags: MemberInfoFlags::new(145),
    module: "App",
    data: TypeInfoData::Array("Win32Window"),
    array_type: None,
    alignment: 8,
};


