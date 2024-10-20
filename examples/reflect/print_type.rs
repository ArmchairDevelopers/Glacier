use glacier_sdk::{
    reflect::{type_info::TypeInfoData, type_registry::TypeRegistry},
    reflect_swbf2::{core::AssetTrait, entity::Blueprint, register_mod_types},
};

#[tokio::main]
async fn main() {
    let mut registry = TypeRegistry::new();
    register_mod_types(&mut registry);

    let bp_type = registry.get_type("Blueprint").unwrap();
    if let TypeInfoData::Class(class_info) = &bp_type.data {
        let value = (class_info.functions.create)();
        println!("{:#?}", value);

        let test = value.lock().await;
        let bp = test.as_any().downcast_ref::<Blueprint>().unwrap();

        let offset = class_info.field("Flags").unwrap().rust_offset;
        println!("Offset: {}", offset);

        println!("Name: '{}'", bp.name());
    }
}
