use std::collections::HashMap;

use crate::type_info::TypeInfo;

pub struct TypeRegistry {
    pub types_by_name: HashMap<&'static str, &'static TypeInfo>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            types_by_name: HashMap::new(),
        }
    }

    pub fn register_type(&mut self, type_info: &'static TypeInfo) {
        self.types_by_name.insert(type_info.name, type_info);
    }

    pub fn get_type(&self, name: &str) -> Option<&'static TypeInfo> {
        self.types_by_name.get(name).copied()
    }
}
