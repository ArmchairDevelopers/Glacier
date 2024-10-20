use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_lua_runner_types(registry: &mut TypeRegistry) {
    registry.register_type(LUARUNNERSHAREDVARSENTITY_TYPE_INFO);
    registry.register_type(LUARUNNERSHAREDVARSENTITY_ARRAY_TYPE_INFO);
    registry.register_type(LUARUNNERSCRIPTENTITY_TYPE_INFO);
    registry.register_type(LUARUNNERSCRIPTENTITY_ARRAY_TYPE_INFO);
    registry.register_type(COMPILEDLUARESOURCE_TYPE_INFO);
    registry.register_type(COMPILEDLUARESOURCE_ARRAY_TYPE_INFO);
    registry.register_type(LUARUNNERSHAREDVARSENTITYDATA_TYPE_INFO);
    registry.register_type(LUARUNNERSHAREDVARSENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LUARUNNERSCRIPTENTITYDATA_TYPE_INFO);
    registry.register_type(LUARUNNERSCRIPTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CUSTOMPROPERTY_TYPE_INFO);
    registry.register_type(CUSTOMPROPERTY_ARRAY_TYPE_INFO);
    registry.register_type(EXECUTEONPROPERTYCHANGETYPE_TYPE_INFO);
    registry.register_type(EXECUTEONPROPERTYCHANGETYPE_ARRAY_TYPE_INFO);
    registry.register_type(LUARUNNERCOMPILEDLUA_TYPE_INFO);
    registry.register_type(LUARUNNERCOMPILEDLUA_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LuaRunnerSharedVarsEntity {
}

pub const LUARUNNERSHAREDVARSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntity",
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LUARUNNERSHAREDVARSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LuaRunnerSharedVarsEntity {
    fn type_info() -> &'static TypeInfo {
        LUARUNNERSHAREDVARSENTITY_TYPE_INFO
    }
}


pub const LUARUNNERSHAREDVARSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerSharedVarsEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LuaRunnerScriptEntity {
}

pub const LUARUNNERSCRIPTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntity",
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITY_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(LUARUNNERSCRIPTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LuaRunnerScriptEntity {
    fn type_info() -> &'static TypeInfo {
        LUARUNNERSCRIPTENTITY_TYPE_INFO
    }
}


pub const LUARUNNERSCRIPTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntity-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerScriptEntity-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CompiledLuaResource {
}

pub const COMPILEDLUARESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompiledLuaResource",
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        fields: &[
        ],
    }),
    array_type: Some(COMPILEDLUARESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompiledLuaResource {
    fn type_info() -> &'static TypeInfo {
        COMPILEDLUARESOURCE_TYPE_INFO
    }
}


pub const COMPILEDLUARESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompiledLuaResource-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("CompiledLuaResource-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LuaRunnerSharedVarsEntityData {
    pub realm: super::core::Realm,
    pub add_to_debug_display: bool,
}

pub const LUARUNNERSHAREDVARSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerSharedVarsEntityData, realm),
            },
            FieldInfoData {
                name: "AddToDebugDisplay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerSharedVarsEntityData, add_to_debug_display),
            },
        ],
    }),
    array_type: Some(LUARUNNERSHAREDVARSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LuaRunnerSharedVarsEntityData {
    fn type_info() -> &'static TypeInfo {
        LUARUNNERSHAREDVARSENTITYDATA_TYPE_INFO
    }
}


pub const LUARUNNERSHAREDVARSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerSharedVarsEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LuaRunnerScriptEntityData {
    pub script: String,
    pub input_events: Vec<String>,
    pub output_events: Vec<String>,
    pub input_float_properties: Vec<String>,
    pub output_float_properties: Vec<String>,
    pub input_int_properties: Vec<String>,
    pub output_int_properties: Vec<String>,
    pub input_bool_properties: Vec<String>,
    pub output_bool_properties: Vec<String>,
    pub input_string_properties: Vec<String>,
    pub output_string_properties: Vec<String>,
    pub input_transform_properties: Vec<String>,
    pub output_transform_properties: Vec<String>,
    pub input_vec2_properties: Vec<String>,
    pub output_vec2_properties: Vec<String>,
    pub input_vec3_properties: Vec<String>,
    pub output_vec3_properties: Vec<String>,
    pub input_vec4_properties: Vec<String>,
    pub output_vec4_properties: Vec<String>,
    pub input_custom_properties: Vec<CustomProperty>,
    pub output_custom_properties: Vec<CustomProperty>,
    pub auto_start_executing_per_frame: bool,
    pub auto_start_for_initialization: bool,
    pub run_on_property_change: bool,
    pub execute_on_property_change: ExecuteOnPropertyChangeType,
    pub priority_for_executing_per_frame: i32,
    pub realm: super::core::Realm,
    pub add_to_debug_display: bool,
    pub compiled_lua: LuaRunnerCompiledLua,
}

pub const LUARUNNERSCRIPTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntityData",
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ENTITYDATA_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "Script",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, script),
            },
            FieldInfoData {
                name: "InputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_events),
            },
            FieldInfoData {
                name: "OutputEvents",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_events),
            },
            FieldInfoData {
                name: "InputFloatProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_float_properties),
            },
            FieldInfoData {
                name: "OutputFloatProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_float_properties),
            },
            FieldInfoData {
                name: "InputIntProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_int_properties),
            },
            FieldInfoData {
                name: "OutputIntProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_int_properties),
            },
            FieldInfoData {
                name: "InputBoolProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_bool_properties),
            },
            FieldInfoData {
                name: "OutputBoolProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_bool_properties),
            },
            FieldInfoData {
                name: "InputStringProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_string_properties),
            },
            FieldInfoData {
                name: "OutputStringProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_string_properties),
            },
            FieldInfoData {
                name: "InputTransformProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_transform_properties),
            },
            FieldInfoData {
                name: "OutputTransformProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_transform_properties),
            },
            FieldInfoData {
                name: "InputVec2Properties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_vec2_properties),
            },
            FieldInfoData {
                name: "OutputVec2Properties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_vec2_properties),
            },
            FieldInfoData {
                name: "InputVec3Properties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_vec3_properties),
            },
            FieldInfoData {
                name: "OutputVec3Properties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_vec3_properties),
            },
            FieldInfoData {
                name: "InputVec4Properties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_vec4_properties),
            },
            FieldInfoData {
                name: "OutputVec4Properties",
                flags: MemberInfoFlags::new(144),
                field_type: CSTRING_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_vec4_properties),
            },
            FieldInfoData {
                name: "InputCustomProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMPROPERTY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_custom_properties),
            },
            FieldInfoData {
                name: "OutputCustomProperties",
                flags: MemberInfoFlags::new(144),
                field_type: CUSTOMPROPERTY_ARRAY_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_custom_properties),
            },
            FieldInfoData {
                name: "AutoStartExecutingPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, auto_start_executing_per_frame),
            },
            FieldInfoData {
                name: "AutoStartForInitialization",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, auto_start_for_initialization),
            },
            FieldInfoData {
                name: "RunOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, run_on_property_change),
            },
            FieldInfoData {
                name: "ExecuteOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: EXECUTEONPROPERTYCHANGETYPE_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, execute_on_property_change),
            },
            FieldInfoData {
                name: "PriorityForExecutingPerFrame",
                flags: MemberInfoFlags::new(0),
                field_type: INT32_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, priority_for_executing_per_frame),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: REALM_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, realm),
            },
            FieldInfoData {
                name: "AddToDebugDisplay",
                flags: MemberInfoFlags::new(0),
                field_type: BOOLEAN_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, add_to_debug_display),
            },
            FieldInfoData {
                name: "CompiledLua",
                flags: MemberInfoFlags::new(0),
                field_type: LUARUNNERCOMPILEDLUA_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerScriptEntityData, compiled_lua),
            },
        ],
    }),
    array_type: Some(LUARUNNERSCRIPTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LuaRunnerScriptEntityData {
    fn type_info() -> &'static TypeInfo {
        LUARUNNERSCRIPTENTITYDATA_TYPE_INFO
    }
}


pub const LUARUNNERSCRIPTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerScriptEntityData-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct CustomProperty {
    pub type_name: String,
    pub property_name: String,
}

pub const CUSTOMPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomProperty",
    flags: MemberInfoFlags::new(73),
    module: "LuaRunner",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
            FieldInfoData {
                name: "TypeName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomProperty, type_name),
            },
            FieldInfoData {
                name: "PropertyName",
                flags: MemberInfoFlags::new(0),
                field_type: CSTRING_TYPE_INFO,
                rust_offset: offset_of!(CustomProperty, property_name),
            },
        ],
    }),
    array_type: Some(CUSTOMPROPERTY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomProperty {
    fn type_info() -> &'static TypeInfo {
        CUSTOMPROPERTY_TYPE_INFO
    }
}


pub const CUSTOMPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomProperty-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("CustomProperty-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, PartialEq, Eq, Default, Debug)]
#[repr(i32)]
pub enum ExecuteOnPropertyChangeType {
    #[default]
    ExecuteOnPropertyChangeType_DontExecute = 0,
    ExecuteOnPropertyChangeType_Immediate = 1,
    ExecuteOnPropertyChangeType_Queued = 2,
}

pub const EXECUTEONPROPERTYCHANGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExecuteOnPropertyChangeType",
    flags: MemberInfoFlags::new(49429),
    module: "LuaRunner",
    data: TypeInfoData::Enum,
    array_type: Some(EXECUTEONPROPERTYCHANGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExecuteOnPropertyChangeType {
    fn type_info() -> &'static TypeInfo {
        EXECUTEONPROPERTYCHANGETYPE_TYPE_INFO
    }
}


pub const EXECUTEONPROPERTYCHANGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExecuteOnPropertyChangeType-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("ExecuteOnPropertyChangeType-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LuaRunnerCompiledLua {
    pub compiled_lua_resource: super::core::ResourceRef,
}

pub const LUARUNNERCOMPILEDLUA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerCompiledLua",
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(ASSET_TYPE_INFO),
        fields: &[
            FieldInfoData {
                name: "CompiledLuaResource",
                flags: MemberInfoFlags::new(0),
                field_type: RESOURCEREF_TYPE_INFO,
                rust_offset: offset_of!(LuaRunnerCompiledLua, compiled_lua_resource),
            },
        ],
    }),
    array_type: Some(LUARUNNERCOMPILEDLUA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LuaRunnerCompiledLua {
    fn type_info() -> &'static TypeInfo {
        LUARUNNERCOMPILEDLUA_TYPE_INFO
    }
}


pub const LUARUNNERCOMPILEDLUA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerCompiledLua-Array",
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerCompiledLua-Array"),
    array_type: None,
    alignment: 8,
};


