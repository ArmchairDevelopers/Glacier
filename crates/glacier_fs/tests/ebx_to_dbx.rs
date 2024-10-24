use tokio::time::Instant;

use glacier_fs::{dbx::writer::DbxPartitionWriter, ebx::partition::EbxPartitionReader};
use glacier_reflect::type_registry::TypeRegistry;
use glacier_reflect_swbf2::register_mod_types;
use tokio::fs::File;

#[tokio::test]
async fn test_level_list_report() {
    let data = include_bytes!("./data/LevelListReport.bin");

    let mut registry = TypeRegistry::default();
    register_mod_types(&mut registry);

    let start = Instant::now();

    let mut reader = EbxPartitionReader::new("LevelListReport".to_owned(), &registry);
    reader.read(data.to_vec()).await;

    println!("Finished reading EBX");

    let partition = reader.finalize();

    let mut dbx_writer = DbxPartitionWriter::new(&partition, &registry);

    //let mut writer = Vec::new();
    let mut writer = File::create("test.dbx").await.unwrap();
    dbx_writer.write(&mut writer).await.unwrap();

    println!("Finished writing DBX in {:?}", start.elapsed());

    // let writer = String::from_utf8(writer).unwrap();
    // println!("{}\n\ntest", writer);
}
