use std::{any::Any, fmt::Debug, sync::Arc};

use tokio::sync::Mutex;

use crate::{
    data_container::DataContainerCore, member::MemberInfoFlags, type_registry::TypeRegistry,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfoData {
    pub name: &'static str,
    pub name_hash: u32,
    pub flags: MemberInfoFlags,
    /// I really hate that this is a string, but there are some cyclic issues and I don't want to deal with setting up
    /// an option resolution system or something like that.
    pub field_type: &'static str,
    pub rust_offset: usize,
}

impl FieldInfoData {
    pub fn field_type(&self, registry: &TypeRegistry) -> &'static TypeInfo {
        registry.type_by_name(self.field_type).expect(&format!(
            "Field type not found: {}/{}",
            self.name, self.field_type
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctions {
    pub create: fn() -> LockedTypeObject,
    pub create_boxed: fn() -> BoxedTypeObject,
    ///// Expensive! This currently constructs a new object for every call.
    //pub trait_vtable: fn() -> *const (),
}

#[derive(Debug, PartialEq)]
pub struct ClassInfoData {
    pub super_class: Option<&'static TypeInfo>,
    pub super_class_offset: usize,
    pub functions: TypeFunctions,
    pub fields: &'static [FieldInfoData],
}

impl ClassInfoData {
    pub fn create(&self) -> LockedTypeObject {
        (self.functions.create)()
    }

    // pub fn trait_vtable(&self) -> *const () {
    //     (self.functions.trait_vtable)()
    // }

    pub fn field(&self, name: &str) -> Option<&FieldInfoData> {
        for field in self.fields {
            if field.name == name {
                return Some(field);
            }
        }

        if let Some(super_class) = self.super_class {
            if let TypeInfoData::Class(super_class_info) = &super_class.data {
                return super_class_info.field(name);
            }
        }

        None
    }

    pub fn field_by_hash(&self, hash: u32) -> Option<&FieldInfoData> {
        for field in self.fields {
            if field.name_hash == hash {
                return Some(field);
            }
        }

        if let Some(super_class) = self.super_class {
            if let TypeInfoData::Class(super_class_info) = &super_class.data {
                return super_class_info.field_by_hash(hash);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub struct ValueTypeInfoData {
    pub functions: TypeFunctions,
    pub fields: &'static [FieldInfoData],
}

impl ValueTypeInfoData {
    pub fn create(&self) -> LockedTypeObject {
        (self.functions.create)()
    }

    pub fn create_boxed(&self) -> BoxedTypeObject {
        (self.functions.create_boxed)()
    }

    pub fn field(&self, name: &str) -> Option<&FieldInfoData> {
        for field in self.fields {
            if field.name == name {
                return Some(field);
            }
        }

        None
    }

    pub fn field_by_hash(&self, hash: u32) -> Option<&FieldInfoData> {
        for field in self.fields {
            if field.name_hash == hash {
                return Some(field);
            }
        }

        None
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeInfoData {
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Float32,
    Float64,
    Boolean,
    CString,
    ResourceRef,
    TypeRef,
    FileRef, // Alias for CString
    BoxedValueRef,
    SHA1,
    Guid,
    Array(&'static str), // TODO: This should be a reference to a TypeInfo, not a name, but that causes a cycle
    Class(ClassInfoData),
    ValueType(ValueTypeInfoData),
    Enum,
    Unknown,
}

#[derive(Debug, PartialEq)]
pub struct TypeInfo {
    pub name: &'static str,
    pub name_hash: u32,
    pub flags: MemberInfoFlags,
    pub module: &'static str,
    pub data: TypeInfoData,
    pub array_type: Option<&'static TypeInfo>,
    pub alignment: u16,
}

impl TypeInfo {
    /// Returns {module}.{name}
    pub fn full_name(&self) -> String {
        format!("{}.{}", self.module, self.name)
    }

    pub const fn array_type(&self) -> Option<&'static TypeInfo> {
        match &self.data {
            TypeInfoData::Array(_) => self.array_type,
            _ => None,
        }
    }
}

pub trait TypeObject: Any + Send + Sync + Debug {
    fn type_info(&self) -> &'static TypeInfo;

    /// I would much rather these exist in DataContainer itself,
    /// but rust doesn't yet support trait-to-trait downcasting,
    /// so we would have no way to access it without knowing
    /// the concrete type of every object we're working with.
    fn data_container_core(&self) -> Option<&DataContainerCore>;
    fn data_container_core_mut(&mut self) -> Option<&mut DataContainerCore>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub type LockedTypeObject = Arc<Mutex<dyn TypeObject>>;
pub type BoxedTypeObject = Box<dyn TypeObject>;
