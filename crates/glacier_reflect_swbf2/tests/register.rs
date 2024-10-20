use glacier_reflect::type_registry::TypeRegistry;
use glacier_reflect_swbf2::register_types;

pub fn main() {
    let mut registry = TypeRegistry::new();
    register_types(&mut registry);
}
