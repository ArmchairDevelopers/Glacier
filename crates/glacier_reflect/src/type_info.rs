use std::{any::Any, fmt::Debug, sync::Arc};

use glacier_util::hash::QuickHashExt;
use tokio::sync::Mutex;

use crate::{data_container::DataContainerCore, member::MemberInfoFlags, type_registry::TypeRegistry};

#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfoData {
    pub name: &'static str,
    pub flags: MemberInfoFlags,
    /// I really hate that this is a string, but there are some cyclic issues and I don't want to deal with setting up
    /// an option resolution system or something like that.
    pub field_type: &'static str,
    pub rust_offset: usize,
}

impl FieldInfoData {
    pub fn field_type(&self, registry: &TypeRegistry) -> &'static TypeInfo {
        registry.type_by_name(self.field_type).expect(&format!("Field type not found: {}/{}", self.name, self.field_type))
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctions {
    pub create: fn() -> LockedTypeObject,
}

#[derive(Debug, PartialEq)]
pub struct ClassInfoData {
    pub functions: TypeFunctions,
    pub super_class: Option<&'static TypeInfo>,
    pub fields: &'static [FieldInfoData],
}

impl ClassInfoData {
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
            if field.name.quick_hash() == hash {
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
            if field.name.quick_hash() == hash {
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
    pub flags: MemberInfoFlags,
    pub module: &'static str,
    pub data: TypeInfoData,
    pub array_type: Option<&'static TypeInfo>,
    pub alignment: u16,
}

impl TypeInfo {
    pub fn name_hash(&self) -> u32 {
        self.name.quick_hash()
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
