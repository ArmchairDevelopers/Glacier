use std::{any::Any, fmt::Debug, sync::Arc};

use tokio::sync::Mutex;

use crate::{member::MemberInfoFlags, type_registry::TypeRegistry};

#[derive(Debug, PartialEq)]
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
        registry.get_type(self.field_type).expect(&format!("Field type not found: {}/{}", self.name, self.field_type))
    }
}

#[derive(Debug, PartialEq)]
pub struct TypeFunctions {
    pub create: fn() -> Arc<Mutex<dyn TypeObject>>,
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
}

#[derive(Debug, PartialEq)]
pub struct ValueTypeInfoData {
    pub functions: TypeFunctions,
    pub fields: &'static [FieldInfoData],
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
    pub const fn array_type(&self) -> Option<&'static TypeInfo> {
        match &self.data {
            TypeInfoData::Array(_) => self.array_type,
            _ => None,
        }
    }
}

pub trait TypeObject: Any + Send + Sync + Debug {
    fn type_info(&self) -> &'static TypeInfo;
    fn as_any(&self) -> &dyn Any;
}
