use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData,
        TypeObject, TypeFunctions, LockedTypeObject, BoxedTypeObject,
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

#[derive(Debug, Default)]
#[repr(C)]
pub struct LuaRunnerSharedVarsEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LuaRunnerSharedVarsEntityTrait: super::entity::EntityTrait {
}

impl LuaRunnerSharedVarsEntityTrait for LuaRunnerSharedVarsEntity {
}

impl super::entity::EntityTrait for LuaRunnerSharedVarsEntity {
}

impl super::entity::EntityBusPeerTrait for LuaRunnerSharedVarsEntity {
}

pub static LUARUNNERSHAREDVARSENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntity",
    name_hash: 2396660585,
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LuaRunnerSharedVarsEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LuaRunnerSharedVarsEntity as Default>::default())),
            create_boxed: || Box::new(<LuaRunnerSharedVarsEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LUARUNNERSHAREDVARSENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LuaRunnerSharedVarsEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LUARUNNERSHAREDVARSENTITY_TYPE_INFO
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


pub static LUARUNNERSHAREDVARSENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntity-Array",
    name_hash: 3608967773,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerSharedVarsEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LuaRunnerScriptEntity {
    pub _glacier_base: super::entity::Entity,
}

pub trait LuaRunnerScriptEntityTrait: super::entity::EntityTrait {
}

impl LuaRunnerScriptEntityTrait for LuaRunnerScriptEntity {
}

impl super::entity::EntityTrait for LuaRunnerScriptEntity {
}

impl super::entity::EntityBusPeerTrait for LuaRunnerScriptEntity {
}

pub static LUARUNNERSCRIPTENTITY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntity",
    name_hash: 1602952281,
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITY_TYPE_INFO),
        super_class_offset: offset_of!(LuaRunnerScriptEntity, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LuaRunnerScriptEntity as Default>::default())),
            create_boxed: || Box::new(<LuaRunnerScriptEntity as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(LUARUNNERSCRIPTENTITY_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LuaRunnerScriptEntity {
    fn type_info(&self) -> &'static TypeInfo {
        LUARUNNERSCRIPTENTITY_TYPE_INFO
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


pub static LUARUNNERSCRIPTENTITY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntity-Array",
    name_hash: 1656154733,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerScriptEntity"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CompiledLuaResource {
}

pub trait CompiledLuaResourceTrait: TypeObject {
}

impl CompiledLuaResourceTrait for CompiledLuaResource {
}

pub static COMPILEDLUARESOURCE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompiledLuaResource",
    name_hash: 2250795906,
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: None,
        super_class_offset: 0,
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CompiledLuaResource as Default>::default())),
            create_boxed: || Box::new(<CompiledLuaResource as Default>::default()),
        },
        fields: &[
        ],
    }),
    array_type: Some(COMPILEDLUARESOURCE_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for CompiledLuaResource {
    fn type_info(&self) -> &'static TypeInfo {
        COMPILEDLUARESOURCE_TYPE_INFO
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


pub static COMPILEDLUARESOURCE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CompiledLuaResource-Array",
    name_hash: 2532698806,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("CompiledLuaResource"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LuaRunnerSharedVarsEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub add_to_debug_display: bool,
}

pub trait LuaRunnerSharedVarsEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn add_to_debug_display(&self) -> &bool;
    fn add_to_debug_display_mut(&mut self) -> &mut bool;
}

impl LuaRunnerSharedVarsEntityDataTrait for LuaRunnerSharedVarsEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn add_to_debug_display(&self) -> &bool {
        &self.add_to_debug_display
    }
    fn add_to_debug_display_mut(&mut self) -> &mut bool {
        &mut self.add_to_debug_display
    }
}

impl super::entity::EntityDataTrait for LuaRunnerSharedVarsEntityData {
}

impl super::entity::GameObjectDataTrait for LuaRunnerSharedVarsEntityData {
}

impl super::core::DataBusPeerTrait for LuaRunnerSharedVarsEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LuaRunnerSharedVarsEntityData {
}

impl super::core::DataContainerTrait for LuaRunnerSharedVarsEntityData {
}

pub static LUARUNNERSHAREDVARSENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntityData",
    name_hash: 3972229785,
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LuaRunnerSharedVarsEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LuaRunnerSharedVarsEntityData as Default>::default())),
            create_boxed: || Box::new(<LuaRunnerSharedVarsEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LuaRunnerSharedVarsEntityData, realm),
            },
            FieldInfoData {
                name: "AddToDebugDisplay",
                name_hash: 2153105844,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LuaRunnerSharedVarsEntityData, add_to_debug_display),
            },
        ],
    }),
    array_type: Some(LUARUNNERSHAREDVARSENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LuaRunnerSharedVarsEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LUARUNNERSHAREDVARSENTITYDATA_TYPE_INFO
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


pub static LUARUNNERSHAREDVARSENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerSharedVarsEntityData-Array",
    name_hash: 2143610925,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerSharedVarsEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LuaRunnerScriptEntityData {
    pub _glacier_base: super::entity::EntityData,
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
    pub input_custom_properties: Vec<BoxedTypeObject /* CustomProperty */>,
    pub output_custom_properties: Vec<BoxedTypeObject /* CustomProperty */>,
    pub auto_start_executing_per_frame: bool,
    pub auto_start_for_initialization: bool,
    pub run_on_property_change: bool,
    pub execute_on_property_change: ExecuteOnPropertyChangeType,
    pub priority_for_executing_per_frame: i32,
    pub realm: super::core::Realm,
    pub add_to_debug_display: bool,
    pub compiled_lua: Option<LockedTypeObject /* LuaRunnerCompiledLua */>,
}

pub trait LuaRunnerScriptEntityDataTrait: super::entity::EntityDataTrait {
    fn script(&self) -> &String;
    fn script_mut(&mut self) -> &mut String;
    fn input_events(&self) -> &Vec<String>;
    fn input_events_mut(&mut self) -> &mut Vec<String>;
    fn output_events(&self) -> &Vec<String>;
    fn output_events_mut(&mut self) -> &mut Vec<String>;
    fn input_float_properties(&self) -> &Vec<String>;
    fn input_float_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_float_properties(&self) -> &Vec<String>;
    fn output_float_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_int_properties(&self) -> &Vec<String>;
    fn input_int_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_int_properties(&self) -> &Vec<String>;
    fn output_int_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_bool_properties(&self) -> &Vec<String>;
    fn input_bool_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_bool_properties(&self) -> &Vec<String>;
    fn output_bool_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_string_properties(&self) -> &Vec<String>;
    fn input_string_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_string_properties(&self) -> &Vec<String>;
    fn output_string_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_transform_properties(&self) -> &Vec<String>;
    fn input_transform_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_transform_properties(&self) -> &Vec<String>;
    fn output_transform_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_vec2_properties(&self) -> &Vec<String>;
    fn input_vec2_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_vec2_properties(&self) -> &Vec<String>;
    fn output_vec2_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_vec3_properties(&self) -> &Vec<String>;
    fn input_vec3_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_vec3_properties(&self) -> &Vec<String>;
    fn output_vec3_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_vec4_properties(&self) -> &Vec<String>;
    fn input_vec4_properties_mut(&mut self) -> &mut Vec<String>;
    fn output_vec4_properties(&self) -> &Vec<String>;
    fn output_vec4_properties_mut(&mut self) -> &mut Vec<String>;
    fn input_custom_properties(&self) -> &Vec<BoxedTypeObject /* CustomProperty */>;
    fn input_custom_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CustomProperty */>;
    fn output_custom_properties(&self) -> &Vec<BoxedTypeObject /* CustomProperty */>;
    fn output_custom_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CustomProperty */>;
    fn auto_start_executing_per_frame(&self) -> &bool;
    fn auto_start_executing_per_frame_mut(&mut self) -> &mut bool;
    fn auto_start_for_initialization(&self) -> &bool;
    fn auto_start_for_initialization_mut(&mut self) -> &mut bool;
    fn run_on_property_change(&self) -> &bool;
    fn run_on_property_change_mut(&mut self) -> &mut bool;
    fn execute_on_property_change(&self) -> &ExecuteOnPropertyChangeType;
    fn execute_on_property_change_mut(&mut self) -> &mut ExecuteOnPropertyChangeType;
    fn priority_for_executing_per_frame(&self) -> &i32;
    fn priority_for_executing_per_frame_mut(&mut self) -> &mut i32;
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn add_to_debug_display(&self) -> &bool;
    fn add_to_debug_display_mut(&mut self) -> &mut bool;
    fn compiled_lua(&self) -> &Option<LockedTypeObject /* LuaRunnerCompiledLua */>;
    fn compiled_lua_mut(&mut self) -> &mut Option<LockedTypeObject /* LuaRunnerCompiledLua */>;
}

impl LuaRunnerScriptEntityDataTrait for LuaRunnerScriptEntityData {
    fn script(&self) -> &String {
        &self.script
    }
    fn script_mut(&mut self) -> &mut String {
        &mut self.script
    }
    fn input_events(&self) -> &Vec<String> {
        &self.input_events
    }
    fn input_events_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_events
    }
    fn output_events(&self) -> &Vec<String> {
        &self.output_events
    }
    fn output_events_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_events
    }
    fn input_float_properties(&self) -> &Vec<String> {
        &self.input_float_properties
    }
    fn input_float_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_float_properties
    }
    fn output_float_properties(&self) -> &Vec<String> {
        &self.output_float_properties
    }
    fn output_float_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_float_properties
    }
    fn input_int_properties(&self) -> &Vec<String> {
        &self.input_int_properties
    }
    fn input_int_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_int_properties
    }
    fn output_int_properties(&self) -> &Vec<String> {
        &self.output_int_properties
    }
    fn output_int_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_int_properties
    }
    fn input_bool_properties(&self) -> &Vec<String> {
        &self.input_bool_properties
    }
    fn input_bool_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_bool_properties
    }
    fn output_bool_properties(&self) -> &Vec<String> {
        &self.output_bool_properties
    }
    fn output_bool_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_bool_properties
    }
    fn input_string_properties(&self) -> &Vec<String> {
        &self.input_string_properties
    }
    fn input_string_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_string_properties
    }
    fn output_string_properties(&self) -> &Vec<String> {
        &self.output_string_properties
    }
    fn output_string_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_string_properties
    }
    fn input_transform_properties(&self) -> &Vec<String> {
        &self.input_transform_properties
    }
    fn input_transform_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_transform_properties
    }
    fn output_transform_properties(&self) -> &Vec<String> {
        &self.output_transform_properties
    }
    fn output_transform_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_transform_properties
    }
    fn input_vec2_properties(&self) -> &Vec<String> {
        &self.input_vec2_properties
    }
    fn input_vec2_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_vec2_properties
    }
    fn output_vec2_properties(&self) -> &Vec<String> {
        &self.output_vec2_properties
    }
    fn output_vec2_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_vec2_properties
    }
    fn input_vec3_properties(&self) -> &Vec<String> {
        &self.input_vec3_properties
    }
    fn input_vec3_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_vec3_properties
    }
    fn output_vec3_properties(&self) -> &Vec<String> {
        &self.output_vec3_properties
    }
    fn output_vec3_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_vec3_properties
    }
    fn input_vec4_properties(&self) -> &Vec<String> {
        &self.input_vec4_properties
    }
    fn input_vec4_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.input_vec4_properties
    }
    fn output_vec4_properties(&self) -> &Vec<String> {
        &self.output_vec4_properties
    }
    fn output_vec4_properties_mut(&mut self) -> &mut Vec<String> {
        &mut self.output_vec4_properties
    }
    fn input_custom_properties(&self) -> &Vec<BoxedTypeObject /* CustomProperty */> {
        &self.input_custom_properties
    }
    fn input_custom_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CustomProperty */> {
        &mut self.input_custom_properties
    }
    fn output_custom_properties(&self) -> &Vec<BoxedTypeObject /* CustomProperty */> {
        &self.output_custom_properties
    }
    fn output_custom_properties_mut(&mut self) -> &mut Vec<BoxedTypeObject /* CustomProperty */> {
        &mut self.output_custom_properties
    }
    fn auto_start_executing_per_frame(&self) -> &bool {
        &self.auto_start_executing_per_frame
    }
    fn auto_start_executing_per_frame_mut(&mut self) -> &mut bool {
        &mut self.auto_start_executing_per_frame
    }
    fn auto_start_for_initialization(&self) -> &bool {
        &self.auto_start_for_initialization
    }
    fn auto_start_for_initialization_mut(&mut self) -> &mut bool {
        &mut self.auto_start_for_initialization
    }
    fn run_on_property_change(&self) -> &bool {
        &self.run_on_property_change
    }
    fn run_on_property_change_mut(&mut self) -> &mut bool {
        &mut self.run_on_property_change
    }
    fn execute_on_property_change(&self) -> &ExecuteOnPropertyChangeType {
        &self.execute_on_property_change
    }
    fn execute_on_property_change_mut(&mut self) -> &mut ExecuteOnPropertyChangeType {
        &mut self.execute_on_property_change
    }
    fn priority_for_executing_per_frame(&self) -> &i32 {
        &self.priority_for_executing_per_frame
    }
    fn priority_for_executing_per_frame_mut(&mut self) -> &mut i32 {
        &mut self.priority_for_executing_per_frame
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn add_to_debug_display(&self) -> &bool {
        &self.add_to_debug_display
    }
    fn add_to_debug_display_mut(&mut self) -> &mut bool {
        &mut self.add_to_debug_display
    }
    fn compiled_lua(&self) -> &Option<LockedTypeObject /* LuaRunnerCompiledLua */> {
        &self.compiled_lua
    }
    fn compiled_lua_mut(&mut self) -> &mut Option<LockedTypeObject /* LuaRunnerCompiledLua */> {
        &mut self.compiled_lua
    }
}

impl super::entity::EntityDataTrait for LuaRunnerScriptEntityData {
}

impl super::entity::GameObjectDataTrait for LuaRunnerScriptEntityData {
}

impl super::core::DataBusPeerTrait for LuaRunnerScriptEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LuaRunnerScriptEntityData {
}

impl super::core::DataContainerTrait for LuaRunnerScriptEntityData {
}

pub static LUARUNNERSCRIPTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntityData",
    name_hash: 769797929,
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        super_class_offset: offset_of!(LuaRunnerScriptEntityData, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LuaRunnerScriptEntityData as Default>::default())),
            create_boxed: || Box::new(<LuaRunnerScriptEntityData as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "Script",
                name_hash: 3334736010,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, script),
            },
            FieldInfoData {
                name: "InputEvents",
                name_hash: 1542460652,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_events),
            },
            FieldInfoData {
                name: "OutputEvents",
                name_hash: 3452448709,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_events),
            },
            FieldInfoData {
                name: "InputFloatProperties",
                name_hash: 3858234242,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_float_properties),
            },
            FieldInfoData {
                name: "OutputFloatProperties",
                name_hash: 170617739,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_float_properties),
            },
            FieldInfoData {
                name: "InputIntProperties",
                name_hash: 938633633,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_int_properties),
            },
            FieldInfoData {
                name: "OutputIntProperties",
                name_hash: 3225717736,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_int_properties),
            },
            FieldInfoData {
                name: "InputBoolProperties",
                name_hash: 506747484,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_bool_properties),
            },
            FieldInfoData {
                name: "OutputBoolProperties",
                name_hash: 2567902133,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_bool_properties),
            },
            FieldInfoData {
                name: "InputStringProperties",
                name_hash: 427644231,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_string_properties),
            },
            FieldInfoData {
                name: "OutputStringProperties",
                name_hash: 1267708078,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_string_properties),
            },
            FieldInfoData {
                name: "InputTransformProperties",
                name_hash: 2153019390,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_transform_properties),
            },
            FieldInfoData {
                name: "OutputTransformProperties",
                name_hash: 1466652727,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_transform_properties),
            },
            FieldInfoData {
                name: "InputVec2Properties",
                name_hash: 2127747472,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_vec2_properties),
            },
            FieldInfoData {
                name: "OutputVec2Properties",
                name_hash: 1231599353,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_vec2_properties),
            },
            FieldInfoData {
                name: "InputVec3Properties",
                name_hash: 4064474385,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_vec3_properties),
            },
            FieldInfoData {
                name: "OutputVec3Properties",
                name_hash: 3491998200,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_vec3_properties),
            },
            FieldInfoData {
                name: "InputVec4Properties",
                name_hash: 1951568662,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_vec4_properties),
            },
            FieldInfoData {
                name: "OutputVec4Properties",
                name_hash: 1055412095,
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_vec4_properties),
            },
            FieldInfoData {
                name: "InputCustomProperties",
                name_hash: 1806454465,
                flags: MemberInfoFlags::new(144),
                field_type: "CustomProperty-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, input_custom_properties),
            },
            FieldInfoData {
                name: "OutputCustomProperties",
                name_hash: 1549158696,
                flags: MemberInfoFlags::new(144),
                field_type: "CustomProperty-Array",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, output_custom_properties),
            },
            FieldInfoData {
                name: "AutoStartExecutingPerFrame",
                name_hash: 1160805898,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, auto_start_executing_per_frame),
            },
            FieldInfoData {
                name: "AutoStartForInitialization",
                name_hash: 3776863489,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, auto_start_for_initialization),
            },
            FieldInfoData {
                name: "RunOnPropertyChange",
                name_hash: 1226425388,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, run_on_property_change),
            },
            FieldInfoData {
                name: "ExecuteOnPropertyChange",
                name_hash: 3984504826,
                flags: MemberInfoFlags::new(0),
                field_type: "ExecuteOnPropertyChangeType",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, execute_on_property_change),
            },
            FieldInfoData {
                name: "PriorityForExecutingPerFrame",
                name_hash: 2324867020,
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, priority_for_executing_per_frame),
            },
            FieldInfoData {
                name: "Realm",
                name_hash: 229961746,
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, realm),
            },
            FieldInfoData {
                name: "AddToDebugDisplay",
                name_hash: 2153105844,
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, add_to_debug_display),
            },
            FieldInfoData {
                name: "CompiledLua",
                name_hash: 3159714056,
                flags: MemberInfoFlags::new(0),
                field_type: "LuaRunnerCompiledLua",
                rust_offset: offset_of!(LuaRunnerScriptEntityData, compiled_lua),
            },
        ],
    }),
    array_type: Some(LUARUNNERSCRIPTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LuaRunnerScriptEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LUARUNNERSCRIPTENTITYDATA_TYPE_INFO
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


pub static LUARUNNERSCRIPTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerScriptEntityData-Array",
    name_hash: 253629853,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerScriptEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct CustomProperty {
    pub type_name: String,
    pub property_name: String,
}

pub trait CustomPropertyTrait: TypeObject {
    fn type_name(&self) -> &String;
    fn type_name_mut(&mut self) -> &mut String;
    fn property_name(&self) -> &String;
    fn property_name_mut(&mut self) -> &mut String;
}

impl CustomPropertyTrait for CustomProperty {
    fn type_name(&self) -> &String {
        &self.type_name
    }
    fn type_name_mut(&mut self) -> &mut String {
        &mut self.type_name
    }
    fn property_name(&self) -> &String {
        &self.property_name
    }
    fn property_name_mut(&mut self) -> &mut String {
        &mut self.property_name
    }
}

pub static CUSTOMPROPERTY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomProperty",
    name_hash: 3058528177,
    flags: MemberInfoFlags::new(73),
    module: "LuaRunner",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<CustomProperty as Default>::default())),
            create_boxed: || Box::new(<CustomProperty as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "TypeName",
                name_hash: 351160634,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CustomProperty, type_name),
            },
            FieldInfoData {
                name: "PropertyName",
                name_hash: 3998208485,
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(CustomProperty, property_name),
            },
        ],
    }),
    array_type: Some(CUSTOMPROPERTY_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for CustomProperty {
    fn type_info(&self) -> &'static TypeInfo {
        CUSTOMPROPERTY_TYPE_INFO
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


pub static CUSTOMPROPERTY_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "CustomProperty-Array",
    name_hash: 2502631941,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("CustomProperty"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum ExecuteOnPropertyChangeType {
    #[default]
    ExecuteOnPropertyChangeType_DontExecute = 0,
    ExecuteOnPropertyChangeType_Immediate = 1,
    ExecuteOnPropertyChangeType_Queued = 2,
}

pub static EXECUTEONPROPERTYCHANGETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExecuteOnPropertyChangeType",
    name_hash: 2110478658,
    flags: MemberInfoFlags::new(49429),
    module: "LuaRunner",
    data: TypeInfoData::Enum,
    array_type: Some(EXECUTEONPROPERTYCHANGETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for ExecuteOnPropertyChangeType {
    fn type_info(&self) -> &'static TypeInfo {
        EXECUTEONPROPERTYCHANGETYPE_TYPE_INFO
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


pub static EXECUTEONPROPERTYCHANGETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ExecuteOnPropertyChangeType-Array",
    name_hash: 2941544694,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("ExecuteOnPropertyChangeType"),
    array_type: None,
    alignment: 8,
};


#[derive(Debug, Default)]
#[repr(C)]
pub struct LuaRunnerCompiledLua {
    pub _glacier_base: super::core::Asset,
    pub compiled_lua_resource: glacier_reflect::builtin::ResourceRef,
}

pub trait LuaRunnerCompiledLuaTrait: super::core::AssetTrait {
    fn compiled_lua_resource(&self) -> &glacier_reflect::builtin::ResourceRef;
    fn compiled_lua_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef;
}

impl LuaRunnerCompiledLuaTrait for LuaRunnerCompiledLua {
    fn compiled_lua_resource(&self) -> &glacier_reflect::builtin::ResourceRef {
        &self.compiled_lua_resource
    }
    fn compiled_lua_resource_mut(&mut self) -> &mut glacier_reflect::builtin::ResourceRef {
        &mut self.compiled_lua_resource
    }
}

impl super::core::AssetTrait for LuaRunnerCompiledLua {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for LuaRunnerCompiledLua {
}

pub static LUARUNNERCOMPILEDLUA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerCompiledLua",
    name_hash: 1278146496,
    flags: MemberInfoFlags::new(101),
    module: "LuaRunner",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        super_class_offset: offset_of!(LuaRunnerCompiledLua, _glacier_base),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LuaRunnerCompiledLua as Default>::default())),
            create_boxed: || Box::new(<LuaRunnerCompiledLua as Default>::default()),
        },
        fields: &[
            FieldInfoData {
                name: "CompiledLuaResource",
                name_hash: 2250795906,
                flags: MemberInfoFlags::new(0),
                field_type: "ResourceRef",
                rust_offset: offset_of!(LuaRunnerCompiledLua, compiled_lua_resource),
            },
        ],
    }),
    array_type: Some(LUARUNNERCOMPILEDLUA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LuaRunnerCompiledLua {
    fn type_info(&self) -> &'static TypeInfo {
        LUARUNNERCOMPILEDLUA_TYPE_INFO
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


pub static LUARUNNERCOMPILEDLUA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LuaRunnerCompiledLua-Array",
    name_hash: 3495718516,
    flags: MemberInfoFlags::new(145),
    module: "LuaRunner",
    data: TypeInfoData::Array("LuaRunnerCompiledLua"),
    array_type: None,
    alignment: 8,
};


