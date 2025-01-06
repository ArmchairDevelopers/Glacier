use std::collections::HashMap;

use crate::{
    builtin::register_builtin_types, primitive::register_primitive_types, type_info::{TypeInfo, TypeInfoData},
};

pub struct TypeRegistry {
    pub types_by_name: HashMap<&'static str, &'static TypeInfo>,
    pub types_by_name_hash: HashMap<u32, &'static TypeInfo>,
    pub types_by_super: HashMap<&'static TypeInfo, Vec<&'static TypeInfo>>,
}

impl TypeRegistry {
    pub fn register_type(&mut self, type_info: &'static TypeInfo) {
        self.types_by_name.insert(type_info.name, type_info);
        self.types_by_name_hash
            .insert(type_info.name_hash, type_info);

        if let TypeInfoData::Class(class_info) = &type_info.data && let Some(super_class) = class_info.super_class {
            self.types_by_super
                .entry(super_class)
                .or_insert_with(Vec::new)
                .push(type_info);
        }
    }

    pub fn type_by_name(&self, name: &str) -> Option<&'static TypeInfo> {
        self.types_by_name.get(name).copied()
    }

    pub fn type_by_hash(&self, name_hash: u32) -> Option<&'static TypeInfo> {
        self.types_by_name_hash.get(&name_hash).copied()
    }

    pub fn types_by_super(&self, super_type: &'static TypeInfo) -> Option<&Vec<&'static TypeInfo>> {
        self.types_by_super.get(super_type)
    }

    /// Exercise caution when using this function, as it can be very slow for large type hierarchies
    pub fn types_by_super_recursive(&self, super_type: &'static TypeInfo) -> Vec<&'static TypeInfo> {
        let mut types = Vec::new();
        let mut to_visit = vec![super_type];

        while let Some(super_type) = to_visit.pop() {
            if let Some(sub_types) = self.types_by_super(super_type) {
                for sub_type in sub_types {
                    if !types.contains(sub_type) {
                        types.push(sub_type);
                        to_visit.push(sub_type);
                    }
                }
            }
        }

        types
    }

    pub fn is_type_descendant(&self, super_type: &'static TypeInfo, sub_type: &'static TypeInfo) -> bool {
        let mut current_type = sub_type;

        while let TypeInfoData::Class(class_info) = &current_type.data {
            if let Some(super_class) = class_info.super_class {
                if super_class == super_type {
                    return true;
                }

                current_type = super_class;
            } else {
                break;
            }
        }

        false
    }

    pub fn types(&self) -> Vec<&'static TypeInfo> {
        self.types_by_name.values().copied().collect()
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        let mut registry = Self {
            types_by_name: HashMap::new(),
            types_by_name_hash: HashMap::new(),
            types_by_super: HashMap::new(),
        };

        register_primitive_types(&mut registry);
        register_builtin_types(&mut registry);

        registry
    }
}
