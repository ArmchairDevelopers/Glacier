use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
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


pub const GETWENTDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetWentDown(Boolean,InputDeviceKeys)",
    flags: MemberInfoFlags::new(793),
    module: "Input",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub const GETISDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetIsDown(Boolean,InputDeviceKeys)",
    flags: MemberInfoFlags::new(793),
    module: "Input",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputHandle {
}

pub const INPUTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputHandle",
    flags: MemberInfoFlags::new(73),
    module: "Input",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(INPUTHANDLE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputHandle {
    fn type_info() -> &'static TypeInfo {
        INPUTHANDLE_TYPE_INFO
    }
}


pub const INPUTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputHandle-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputHandle-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputSimDynamicState {
    pub input_sim_state: Vec<u8>,
    pub input_sim_state_size: u16,
    pub field_flag_changed0: u8,
}

pub const INPUTSIMDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimDynamicState",
    flags: MemberInfoFlags::new(73),
    module: "Input",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "inputSimState",
                flags: MemberInfoFlags::new(144),
                field_type: UINT8_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(InputSimDynamicState, input_sim_state),
            },
            FieldInfoData {
                name: "inputSimStateSize",
                flags: MemberInfoFlags::new(0),
                field_type: UINT16_TYPE_INFO,
                rust_offset: offset_of!(InputSimDynamicState, input_sim_state_size),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: UINT8_TYPE_INFO,
                rust_offset: offset_of!(InputSimDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(INPUTSIMDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for InputSimDynamicState {
    fn type_info() -> &'static TypeInfo {
        INPUTSIMDYNAMICSTATE_TYPE_INFO
    }
}


pub const INPUTSIMDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputSimDynamicState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputSimStaticState {
}

pub const INPUTSIMSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimStaticState",
    flags: MemberInfoFlags::new(36937),
    module: "Input",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: Some(INPUTSIMSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for InputSimStaticState {
    fn type_info() -> &'static TypeInfo {
        INPUTSIMSTATICSTATE_TYPE_INFO
    }
}


pub const INPUTSIMSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputSimStaticState-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IInputDevice {
}

pub const IINPUTDEVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IInputDevice",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(IINPUTDEVICE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IInputDevice {
    fn type_info() -> &'static TypeInfo {
        IINPUTDEVICE_TYPE_INFO
    }
}


pub const IINPUTDEVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IInputDevice-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IInputDevice-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputManTouch {
}

pub const INPUTMANTOUCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManTouch",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITOUCH_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INPUTMANTOUCH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InputManTouch {
    fn type_info() -> &'static TypeInfo {
        INPUTMANTOUCH_TYPE_INFO
    }
}


pub const INPUTMANTOUCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManTouch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManTouch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputManMouse {
}

pub const INPUTMANMOUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManMouse",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOUSE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INPUTMANMOUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InputManMouse {
    fn type_info() -> &'static TypeInfo {
        INPUTMANMOUSE_TYPE_INFO
    }
}


pub const INPUTMANMOUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManMouse-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManMouse-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InputManKeyboard {
}

pub const INPUTMANKEYBOARD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManKeyboard",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IKEYBOARD_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(INPUTMANKEYBOARD_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for InputManKeyboard {
    fn type_info() -> &'static TypeInfo {
        INPUTMANKEYBOARD_TYPE_INFO
    }
}


pub const INPUTMANKEYBOARD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManKeyboard-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManKeyboard-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ITouch {
}

pub const ITOUCH_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITouch",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOUSE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(ITOUCH_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ITouch {
    fn type_info() -> &'static TypeInfo {
        ITOUCH_TYPE_INFO
    }
}


pub const ITOUCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITouch-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("ITouch-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IMouse {
}

pub const IMOUSE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMouse",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IINPUTDEVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IMOUSE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IMouse {
    fn type_info() -> &'static TypeInfo {
        IMOUSE_TYPE_INFO
    }
}


pub const IMOUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMouse-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IMouse-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IKeyboard {
}

pub const IKEYBOARD_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKeyboard",
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IINPUTDEVICE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(IKEYBOARD_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for IKeyboard {
    fn type_info() -> &'static TypeInfo {
        IKEYBOARD_TYPE_INFO
    }
}


pub const IKEYBOARD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKeyboard-Array",
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IKeyboard-Array"),
    array_type: None,
    alignment: 8,
};


