use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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
    name_hash: 2187233866,
    flags: MemberInfoFlags::new(793),
    module: "Input",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};



pub static GETISDOWN_BOOLEAN_INPUTDEVICEKEYS__TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "GetIsDown(Boolean,InputDeviceKeys)",
    name_hash: 2038532376,
    flags: MemberInfoFlags::new(793),
    module: "Input",
    data: TypeInfoData::Unknown,
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputHandle {
}

pub trait InputHandleTrait: TypeObject {
}

impl InputHandleTrait for InputHandle {
}

pub static INPUTHANDLE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputHandle",
    name_hash: 1987894969,
    flags: MemberInfoFlags::new(73),
    module: "Input",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputHandle as Default>::default())),
            create_boxed: || Box::new(<InputHandle as Default>::default()),
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


pub static INPUTHANDLE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputHandle-Array",
    name_hash: 1064381709,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputHandle"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputSimDynamicState {
    pub input_sim_state: Vec<u8>,
    pub input_sim_state_size: u16,
    pub field_flag_changed0: u8,
}

pub trait InputSimDynamicStateTrait: TypeObject {
    fn input_sim_state(&self) -> &Vec<u8>;
    fn input_sim_state_mut(&mut self) -> &mut Vec<u8>;
    fn input_sim_state_size(&self) -> &u16;
    fn input_sim_state_size_mut(&mut self) -> &mut u16;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl InputSimDynamicStateTrait for InputSimDynamicState {
    fn input_sim_state(&self) -> &Vec<u8> {
        &self.input_sim_state
    }
    fn input_sim_state_mut(&mut self) -> &mut Vec<u8> {
        &mut self.input_sim_state
    }
    fn input_sim_state_size(&self) -> &u16 {
        &self.input_sim_state_size
    }
    fn input_sim_state_size_mut(&mut self) -> &mut u16 {
        &mut self.input_sim_state_size
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static INPUTSIMDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimDynamicState",
    name_hash: 2022874566,
    flags: MemberInfoFlags::new(73),
    module: "Input",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputSimDynamicState as Default>::default())),
            create_boxed: || Box::new(<InputSimDynamicState as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "inputSimState",
                name_hash: 282344627,
                flags: MemberInfoFlags::new(144),
                field_type: "Uint8-Array",
                rust_offset: offset_of!(InputSimDynamicState, input_sim_state),
            },
            FieldInfoData {
                name: "inputSimStateSize",
                name_hash: 2773591798,
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(InputSimDynamicState, input_sim_state_size),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                name_hash: 4279507097,
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


pub static INPUTSIMDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimDynamicState-Array",
    name_hash: 2044509042,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputSimDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct InputSimStaticState {
}

pub trait InputSimStaticStateTrait: TypeObject {
}

impl InputSimStaticStateTrait for InputSimStaticState {
}

pub static INPUTSIMSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimStaticState",
    name_hash: 681777291,
    flags: MemberInfoFlags::new(36937),
    module: "Input",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputSimStaticState as Default>::default())),
            create_boxed: || Box::new(<InputSimStaticState as Default>::default()),
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


pub static INPUTSIMSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputSimStaticState-Array",
    name_hash: 3335833407,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputSimStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct IInputDevice {
}

pub trait IInputDeviceTrait: TypeObject {
}

impl IInputDeviceTrait for IInputDevice {
}

pub static IINPUTDEVICE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IInputDevice",
    name_hash: 618347266,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IInputDevice as Default>::default())),
            create_boxed: || Box::new(<IInputDevice as Default>::default()),
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


pub static IINPUTDEVICE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IInputDevice-Array",
    name_hash: 3345373750,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IInputDevice"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1968663220,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ITOUCH_TYPE_INFO),
        super_class_offset: offset_of!(InputManTouch, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputManTouch as Default>::default())),
            create_boxed: || Box::new(<InputManTouch as Default>::default()),
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


pub static INPUTMANTOUCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManTouch-Array",
    name_hash: 1559086592,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManTouch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1988358448,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOUSE_TYPE_INFO),
        super_class_offset: offset_of!(InputManMouse, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputManMouse as Default>::default())),
            create_boxed: || Box::new(<InputManMouse as Default>::default()),
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


pub static INPUTMANMOUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManMouse-Array",
    name_hash: 3871702660,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManMouse"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 8360668,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IKEYBOARD_TYPE_INFO),
        super_class_offset: offset_of!(InputManKeyboard, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<InputManKeyboard as Default>::default())),
            create_boxed: || Box::new(<InputManKeyboard as Default>::default()),
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


pub static INPUTMANKEYBOARD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "InputManKeyboard-Array",
    name_hash: 919018344,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("InputManKeyboard"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2850130537,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IMOUSE_TYPE_INFO),
        super_class_offset: offset_of!(ITouch, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ITouch as Default>::default())),
            create_boxed: || Box::new(<ITouch as Default>::default()),
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


pub static ITOUCH_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ITouch-Array",
    name_hash: 2312446301,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("ITouch"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 2820948205,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IINPUTDEVICE_TYPE_INFO),
        super_class_offset: offset_of!(IMouse, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IMouse as Default>::default())),
            create_boxed: || Box::new(<IMouse as Default>::default()),
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


pub static IMOUSE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IMouse-Array",
    name_hash: 925263321,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IMouse"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
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
    name_hash: 1991459297,
    flags: MemberInfoFlags::new(101),
    module: "Input",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(IINPUTDEVICE_TYPE_INFO),
        super_class_offset: offset_of!(IKeyboard, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IKeyboard as Default>::default())),
            create_boxed: || Box::new(<IKeyboard as Default>::default()),
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


pub static IKEYBOARD_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IKeyboard-Array",
    name_hash: 192093397,
    flags: MemberInfoFlags::new(145),
    module: "Input",
    data: TypeInfoData::Array("IKeyboard"),
    array_type: None,
    alignment: 8,
};


