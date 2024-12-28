use glacier_util::{guid::Guid, sha1::Sha1};

use crate::io::native_reader::NativeReader;

use std::collections::HashMap;
use std::convert::TryInto;
use std::rc::Rc;

const DB_TYPE_INVALID: u8 = 0;
const DB_TYPE_LIST: u8 = 1;
const DB_TYPE_OBJECT: u8 = 2;
const DB_TYPE_BOOLEAN: u8 = 6;
const DB_TYPE_STRING: u8 = 7;
const DB_TYPE_INT: u8 = 8;
const DB_TYPE_LONG: u8 = 9;
const DB_TYPE_FLOAT: u8 = 11;
const DB_TYPE_DOUBLE: u8 = 12;
const DB_TYPE_GUID: u8 = 15;
const DB_TYPE_SHA1: u8 = 16;
const DB_TYPE_BYTE_ARRAY: u8 = 19;

#[derive(thiserror::Error, Debug)]
pub enum DbObjectError {
    #[error("Invalid type {0}")]
    InvalidType(u8),
}

#[derive(Debug)]
pub enum DbObject {
    Hash(HashMap<String, Rc<DbType>>),
    List(Vec<Rc<DbType>>),
}

impl DbObject {
    pub fn get(&self, key: &str) -> Option<Rc<DbType>> {
        if let DbObject::Hash(hash) = self {
            return hash.get(key).cloned();
        }

        None
    }
}

#[derive(Debug)]
pub enum DbType {
    Invalid,
    List(DbObject),
    Object(DbObject),
    Boolean(bool),
    String(String),
    Int(u32),
    Long(i64),
    Float(f32),
    Double(f64),
    Guid(Guid),
    Sha1(Sha1),
    ByteArray(Vec<u8>),
}

macro_rules! db_type_getter {
    ($name:ident, $variant:ident, $type:ty) => {
        paste::paste! {
            pub fn [<get_as_ $name>](&self) -> Result<&$type, DbObjectError> {
                if let DbType::$variant(x) = self {
                    return Ok(x);
                }

                Err(DbObjectError::InvalidType(0))
            }
        }
    };
}

impl DbType {
    db_type_getter!(list, List, DbObject);
    db_type_getter!(object, Object, DbObject);
    db_type_getter!(boolean, Boolean, bool);
    db_type_getter!(str, String, String);
    db_type_getter!(int, Int, u32);
    db_type_getter!(byte_array, ByteArray, Vec<u8>);
}

pub fn read_db_object(
    buf: &mut NativeReader,
) -> Result<Option<(String, Rc<DbType>)>, DbObjectError> {
    let tmp = buf.get_u8();
    let obj_type = tmp & 0x1F;
    if obj_type == DB_TYPE_INVALID {
        return Ok(None);
    }

    let name = if tmp & 0x80 == 0 {
        buf.get_null_terminated_str()
    } else {
        String::new()
    };

    let result = match obj_type {
        DB_TYPE_LIST => {
            let size = buf.get_7bit_encoded_long();
            let offset = buf.pos();

            let mut values: Vec<Rc<DbType>> = Vec::new();
            while buf.pos() - offset < size.try_into().unwrap() {
                let object = read_db_object(buf)?;
                if object.is_none() {
                    break;
                }

                let object = object.unwrap();
                values.push(object.1);
            }

            DbType::List(DbObject::List(values))
        }
        DB_TYPE_OBJECT => {
            let size = buf.get_7bit_encoded_long();
            let offset = buf.pos();

            let mut values: HashMap<String, Rc<DbType>> = HashMap::new();
            while buf.pos() - offset < size.try_into().unwrap() {
                let object = read_db_object(buf)?;
                if object.is_none() {
                    break;
                }

                let object = object.unwrap();
                values.insert(object.0, object.1);
            }

            DbType::Object(DbObject::Hash(values))
        }
        DB_TYPE_BOOLEAN => DbType::Boolean(buf.get_u8() == 1),
        DB_TYPE_STRING => {
            let size = buf.get_7bit_encoded_int();
            DbType::String(buf.get_sized_str(size))
        }
        DB_TYPE_INT => DbType::Int(buf.get_u32()),
        DB_TYPE_LONG => DbType::Long(buf.get_i64()),
        DB_TYPE_FLOAT => DbType::Float(buf.get_f32()),
        DB_TYPE_DOUBLE => DbType::Double(buf.get_f64()),
        DB_TYPE_GUID => DbType::Guid(buf.get_guid()),
        DB_TYPE_SHA1 => DbType::Sha1(buf.get_sha1()),
        DB_TYPE_BYTE_ARRAY => {
            let size = buf.get_7bit_encoded_long();
            let offset = buf.pos();

            let mut values: Vec<u8> = Vec::new();
            while buf.pos() - offset < size.try_into().unwrap() {
                values.push(buf.get_u8());
            }

            DbType::ByteArray(values)
        }
        _ => {
            println!("Unknown type {obj_type}");
            return Ok(None);
        }
    };

    Ok(Some((name, Rc::new(result))))
}
