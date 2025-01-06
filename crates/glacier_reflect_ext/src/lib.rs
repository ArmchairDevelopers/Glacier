#![allow(dead_code)]
#![allow(unused_imports)]

use glacier_reflect_swbf2::{core::{Asset, DataContainer, DataContainerTrait, ASSET_TYPE_INFO}, register_mod_types};
use glacier_util::hash::hash_quick_str_const;
use std::{any::Any, mem::offset_of, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        BoxedTypeObject, ClassInfoData, FieldInfoData, LockedTypeObject, TypeFunctions, TypeInfo,
        TypeInfoData, TypeObject, ValueTypeInfoData,
    },
    type_registry::TypeRegistry,
};

pub mod util;
