use std::collections::HashMap;

use glacier_util::hash::hash_quick_str;

use crate::type_info::TypeInfo;

#[derive(Default)]
pub struct TypeRegistry {
    pub types_by_name: HashMap<&'static str, &'static TypeInfo>,
    pub types_by_name_hash: HashMap<u32, &'static TypeInfo>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            types_by_name: HashMap::new(),
            types_by_name_hash: HashMap::new(),
        }
    }

    pub fn register_type(&mut self, type_info: &'static TypeInfo) {
        self.types_by_name.insert(type_info.name, type_info);
        self.types_by_name_hash.insert(hash_quick_str(&type_info.name), type_info);
    }

    pub fn get_type(&self, name: &str) -> Option<&'static TypeInfo> {
        self.types_by_name.get(name).copied()
    }
    
    pub fn get_type_by_hash(&self, name_hash: u32) -> Option<&'static TypeInfo> {
        self.types_by_name_hash.get(&name_hash).copied()
    }
}
