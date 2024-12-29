use glacier_reflect::type_registry::TypeRegistry;
use glacier_reflect_swbf2::register_mod_types;

pub fn main() {
    let mut registry = TypeRegistry::default();
    register_mod_types(&mut registry);

    let bp_type = registry.type_by_name("Blueprint").unwrap();
    println!("{:?}", bp_type);
}
