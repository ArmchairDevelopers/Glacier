use bytes::BytesMut;
use glacier_sdk::{
    fs::ebx::partition::EbxPartitionReader,
    reflect::{type_info::TypeInfoData, type_registry::TypeRegistry},
    reflect_swbf2::{
        core::{AssetTrait, DataContainer, DataContainerTrait},
        entity::Blueprint,
        register_mod_types,
    },
};

#[tokio::main]
async fn main() {
    let data = include_bytes!("../../crates/glacier_fs/tests/data/DefaultSoldierStateMachine.bin");

    let mut registry = TypeRegistry::default();
    register_mod_types(&mut registry);

    let partition = {
        let mut reader = EbxPartitionReader::new("LevelListReport".to_owned(), &registry);
        reader.read(data.to_vec()).await;
        reader.finalize()
    };

    let primary_instance = partition.primary_instance().unwrap();
    //let primary_instance = primary_instance.lock().await;

    //println!("{:?}", primary_instance);
    println!("Done");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
}
