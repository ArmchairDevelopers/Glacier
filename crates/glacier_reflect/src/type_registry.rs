use std::collections::HashMap;

use glacier_util::hash::hash_quick_str;

use crate::{
    builtin::register_builtin_types, primitive::register_primitive_types, type_info::TypeInfo,
};

pub struct TypeRegistry {
    pub types_by_name: HashMap<&'static str, &'static TypeInfo>,
    pub types_by_name_hash: HashMap<u32, &'static TypeInfo>,
}

impl TypeRegistry {
    pub fn register_type(&mut self, type_info: &'static TypeInfo) {
        self.types_by_name.insert(type_info.name, type_info);
        self.types_by_name_hash
            .insert(hash_quick_str(&type_info.name), type_info);
    }

    pub fn type_by_name(&self, name: &str) -> Option<&'static TypeInfo> {
        self.types_by_name.get(name).copied()
    }

    pub fn type_by_hash(&self, name_hash: u32) -> Option<&'static TypeInfo> {
        self.types_by_name_hash.get(&name_hash).copied()
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        let mut registry = Self {
            types_by_name: HashMap::new(),
            types_by_name_hash: HashMap::new(),
        };

        register_primitive_types(&mut registry);
        register_builtin_types(&mut registry);

        registry
    }
}
