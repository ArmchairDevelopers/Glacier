use std::any::Any;

use crate::member::MemberInfoFlags;

pub struct FieldInfoData {
    pub name: &'static str,
    pub flags: MemberInfoFlags,
    pub field_type: &'static TypeInfo,
    pub rust_offset: usize,
}

pub struct ClassInfoData {
    pub super_class: Option<&'static TypeInfo>,
    pub fields: &'static [FieldInfoData],
}

pub struct ValueTypeInfoData {
    pub fields: &'static [FieldInfoData],
}

pub enum TypeInfoData {
    Uint8,
    Int8,
    Uint16,
    Int16,
    Uint32,
    Int32,
    Uint64,
    Int64,
    Array(&'static str), // TODO: This should be a reference to a TypeInfo, not a name, but that causes a cycle
    Class(ClassInfoData),
    ValueType(ValueTypeInfoData),
    Enum,
    Unknown,
}

pub struct TypeInfo {
    pub name: &'static str,
    pub flags: MemberInfoFlags,
    pub module: &'static str,
    pub data: TypeInfoData,
    pub array_type: Option<&'static TypeInfo>,
    pub alignment: u16,
}

pub trait TypeObject: Any + Send + Sync {
    fn type_info() -> &'static TypeInfo;
}
