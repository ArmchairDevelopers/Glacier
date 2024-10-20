use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_input_types(registry: &mut TypeRegistry) {
    registry.register_type(GETWENTDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO);
    registry.register_type(GETISDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO);
    registry.register_type(INPUTHANDLE_TYPE_INFO);
    registry.register_type(INPUTHANDLE_ARRAY_TYPE_INFO);
    registry.register_type(INPUTSIMDYNAMICSTATE_TYPE_INFO);
    registry.register_type(INPUTSIMDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(INPUTSIMSTATICSTATE_TYPE_INFO);
    registry.register_type(INPUTSIMSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(IINPUTDEVICE_TYPE_INFO);
    registry.register_type(IINPUTDEVICE_ARRAY_TYPE_INFO);
    registry.register_type(INPUTMANTOUCH_TYPE_INFO);
    registry.register_type(INPUTMANTOUCH_ARRAY_TYPE_INFO);
    registry.register_type(INPUTMANMOUSE_TYPE_INFO);
    registry.register_type(INPUTMANMOUSE_ARRAY_TYPE_INFO);
    registry.register_type(INPUTMANKEYBOARD_TYPE_INFO);
    registry.register_type(INPUTMANKEYBOARD_ARRAY_TYPE_INFO);
    registry.register_type(ITOUCH_TYPE_INFO);
    registry.register_type(ITOUCH_ARRAY_TYPE_INFO);
    registry.register_type(IMOUSE_TYPE_INFO);
    registry.register_type(IMOUSE_ARRAY_TYPE_INFO);
    registry.register_type(IKEYBOARD_TYPE_INFO);
    registry.register_type(IKEYBOARD_ARRAY_TYPE_INFO);
}


pub static GETWENTDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetWentDown(Boolean,InputDeviceKeys)",
    flags: MemberInfoFlags::new(793),
    module: "Input",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static GETISDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetIsDown(Boolean,InputDeviceKeys)",
    flags: MemberInfoFlags::new(793),
    module: "Input",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputHandle {
}

pub trait InputHandleTrait: TypeObject {
}

impl InputHandleTrait for InputHandle {
}

pub static INPUTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputHandle",
    flags: MemberInfoFlags::new(73),
    module: "Input",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputHandle as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INPUTHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputHandle {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTHANDLE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputSimDynamicState {
    pub input_sim_state: Vec<u8>,
    pub input_sim_state_size: u16,
    pub field_flag_changed0: u8,
}

pub trait InputSimDynamicStateTrait: TypeObject {
    fn input_sim_state(&self) -> &Vec<u8>;
    fn input_sim_state_size(&self) -> &u16;
    fn field_flag_changed0(&self) -> &u8;
}

impl InputSimDynamicStateTrait for InputSimDynamicState {
    fn input_sim_state(&self) -> &Vec<u8> {
        &self.input_sim_state
    }
    fn input_sim_state_size(&self) -> &u16 {
        &self.input_sim_state_size
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
}

pub static INPUTSIMDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Input",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputSimDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "inputSimState",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(InputSimDynamicState, input_sim_state),
            },
            FieldInfoData {
                name: "inputSimStateSize",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(InputSimDynamicState, input_sim_state_size),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(InputSimDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(INPUTSIMDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputSimDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTSIMDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTSIMDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputSimDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputSimStaticState {
}

pub trait InputSimStaticStateTrait: TypeObject {
}

impl InputSimStaticStateTrait for InputSimStaticState {
}

pub static INPUTSIMSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "Input",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputSimStaticState as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INPUTSIMSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputSimStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTSIMSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTSIMSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputSimStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IInputDevice {
}

pub trait IInputDeviceTrait: TypeObject {
}

impl IInputDeviceTrait for IInputDevice {
}

pub static IINPUTDEVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IInputDevice",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IInputDevice as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IINPUTDEVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IInputDevice {
    fn type_info(&self) -> &'static TypeInfo {
        IINPUTDEVICE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IINPUTDEVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IInputDevice-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IInputDevice"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputManTouch {
    pub _glacier_base: ITouch,
}

pub trait InputManTouchTrait: ITouchTrait {
}

impl InputManTouchTrait for InputManTouch {
}

impl ITouchTrait for InputManTouch {
}

impl IMouseTrait for InputManTouch {
}

impl IInputDeviceTrait for InputManTouch {
}

pub static INPUTMANTOUCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManTouch",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITOUCH_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputManTouch as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INPUTMANTOUCH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InputManTouch {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMANTOUCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTMANTOUCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManTouch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManTouch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputManMouse {
    pub _glacier_base: IMouse,
}

pub trait InputManMouseTrait: IMouseTrait {
}

impl InputManMouseTrait for InputManMouse {
}

impl IMouseTrait for InputManMouse {
}

impl IInputDeviceTrait for InputManMouse {
}

pub static INPUTMANMOUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManMouse",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOUSE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputManMouse as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INPUTMANMOUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InputManMouse {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMANMOUSE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTMANMOUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManMouse-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManMouse"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct InputManKeyboard {
    pub _glacier_base: IKeyboard,
}

pub trait InputManKeyboardTrait: IKeyboardTrait {
}

impl InputManKeyboardTrait for InputManKeyboard {
}

impl IKeyboardTrait for InputManKeyboard {
}

impl IInputDeviceTrait for InputManKeyboard {
}

pub static INPUTMANKEYBOARD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManKeyboard",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IKEYBOARD_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputManKeyboard as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(INPUTMANKEYBOARD_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InputManKeyboard {
    fn type_info(&self) -> &'static TypeInfo {
        INPUTMANKEYBOARD_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static INPUTMANKEYBOARD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManKeyboard-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManKeyboard"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ITouch {
    pub _glacier_base: IMouse,
}

pub trait ITouchTrait: IMouseTrait {
}

impl ITouchTrait for ITouch {
}

impl IMouseTrait for ITouch {
}

impl IInputDeviceTrait for ITouch {
}

pub static ITOUCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITouch",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOUSE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ITouch as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(ITOUCH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ITouch {
    fn type_info(&self) -> &'static TypeInfo {
        ITOUCH_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static ITOUCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITouch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("ITouch"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IMouse {
    pub _glacier_base: IInputDevice,
}

pub trait IMouseTrait: IInputDeviceTrait {
}

impl IMouseTrait for IMouse {
}

impl IInputDeviceTrait for IMouse {
}

pub static IMOUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMouse",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IINPUTDEVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IMouse as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IMOUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IMouse {
    fn type_info(&self) -> &'static TypeInfo {
        IMOUSE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IMOUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMouse-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IMouse"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IKeyboard {
    pub _glacier_base: IInputDevice,
}

pub trait IKeyboardTrait: IInputDeviceTrait {
}

impl IKeyboardTrait for IKeyboard {
}

impl IInputDeviceTrait for IKeyboard {
}

pub static IKEYBOARD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKeyboard",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IINPUTDEVICE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IKeyboard as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(IKEYBOARD_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IKeyboard {
    fn type_info(&self) -> &'static TypeInfo {
        IKEYBOARD_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static IKEYBOARD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKeyboard-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IKeyboard"),
    array_type: None,
    alignment: 8,
};


