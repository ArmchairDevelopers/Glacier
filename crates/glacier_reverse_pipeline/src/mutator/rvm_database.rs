use std::{
    collections::{HashMap, HashSet},
    io::Cursor,
};

use crate::pipeline::PipelineStorage;
use glacier_reflect::type_info::TypeObject;
use glacier_resource::rvm::pipeline::RvmShaderSandboxData;
use glacier_resource::rvm::{
    reader::{RvmDatabaseReader, RvmDependencyTraverser},
    types::{RvmSerializedDb_ns_SurfaceShader, RvmType},
};
use glacier_store::domain::DomainStore;
use typemap_rev::TypeMapKey;

use super::{PipelineMutationError, PipelineResourceMutator, UnbuildResult};

struct RvmDatabaseByteCount;

impl TypeMapKey for RvmDatabaseByteCount {
    type Value = usize;
}

struct RvmDatabaseUniqueDependencyCount;

impl TypeMapKey for RvmDatabaseUniqueDependencyCount {
    type Value = HashMap<u64, HashSet<usize>>;
}

pub struct Dx11RvmDatabaseResourceMutator;

#[async_trait::async_trait]
impl PipelineResourceMutator for Dx11RvmDatabaseResourceMutator {
    fn max_concurrent_jobs(&self) -> usize {
        1
    }

    fn resource_type_name(&self) -> &'static str {
        "Dx11RvmDatabase"
    }

    async fn pre_mutation(
        &self,
        domain: &DomainStore,
        storage: &PipelineStorage,
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        let mut storage = storage.write().await;

        storage.insert::<RvmDatabaseByteCount>(0);
        storage.insert::<RvmDatabaseUniqueDependencyCount>(HashMap::new());

        Ok(())
    }

    async fn mutate(
        &self,
        domain: &DomainStore,
        storage: PipelineStorage,
        res_name: &str,
        res_id: u64,
        res_meta: &[u8],
        res_data: &[u8],
        result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        println!("Reading Dx11RvmDatabase {}: {}", res_name, res_data.len());

        let mut data = Vec::with_capacity(res_meta.len() + res_data.len());
        data.extend_from_slice(res_meta);
        data.extend_from_slice(res_data);

        {
            let mut storage = storage.write().await;
            let count = storage.get_mut::<RvmDatabaseByteCount>().unwrap();
            *count += data.len();
        }

        let reader = RvmDatabaseReader::new(Cursor::new(data));
        let db = reader.read().await.unwrap();

        println!("Read database, resolving dependencies...");

        let traverser = RvmDependencyTraverser::new(&db);
        let shaders = traverser.traverse(RvmSerializedDb_ns_SurfaceShader::hash());

        println!("Resolved dependencies, found {} shaders", shaders.len());

        let mut asset_index = domain.index_write().await;

        let mut storage = storage.write().await;
        let unique_dependencies = storage
            .get_mut::<RvmDatabaseUniqueDependencyCount>()
            .unwrap();

        for (shader_block_hash, dependencies) in shaders {
            let shader_block = db
                .get_block(shader_block_hash)
                .expect("Shader block not found")
                .first()
                .expect("Shader block is empty");

            if let RvmType::SurfaceShader(shader) = shader_block {
                unique_dependencies
                    .entry(shader.name_hash as u64)
                    .or_default()
                    .insert(dependencies.len());

                //println!("Shader: {}", shader.name_hash);

                let asset = asset_index.get_by_name_hash(shader.name_hash);
                let mut asset = if let Some(asset) = asset {
                    // println!(
                    //     "RVM contains shader: {} ({} dependencies)",
                    //     asset.name,
                    //     dependencies.len()
                    // );

                    asset.clone()
                } else {
                    //println!("Asset not found");
                    continue;
                };

                let mut rvm_sandbox = RvmShaderSandboxData::new();

                let mut sandbox = domain.load_sandbox(&asset.name).await?;
                if let Some(data) = sandbox.read_file("rvmBlocks").await {
                    rvm_sandbox = serde_json::from_slice(&data).unwrap();
                }

                rvm_sandbox.add_block(shader_block_hash, vec![shader_block.clone()]);

                for dependency in &dependencies {
                    if rvm_sandbox.has_block(*dependency) {
                        continue;
                    }

                    let block = db
                        .get_block(*dependency)
                        .expect("Dependency block not found");
                    rvm_sandbox.add_block(*dependency, block.clone());

                    if let RvmType::ShaderStreamableTexture(texture) = block.first().unwrap() {
                        let texture_asset = asset_index
                            .get_by_name_hash(texture.name_hash)
                            .expect("Texture not found");
                        asset.res_imports.insert(texture_asset.partition);
                    }
                }

                // let res_elements = res_name.split('/').collect::<Vec<&str>>();
                //
                // // Second to last element
                // let rvm_name = res_elements.iter().rev().skip(1).next().unwrap();

                sandbox.write_file(
                    "rvmBlocks",
                    serde_json::to_string(&rvm_sandbox)
                        .unwrap()
                        .as_bytes()
                        .to_vec(),
                );
                sandbox.commit().await;

                asset_index.upsert_entry(asset);
            } else {
                panic!("Unexpected shader type");
            }

            // let mut bytecode_count = 0;

            // for dependency in dependencies {
            //     let block = db
            //         .get_block(dependency)
            //         .expect("Dependency block not found")
            //         .first()
            //         .expect("Dependency block is empty");
            //
            //     if let RvmType::ShaderStreamableTexture(texture) = block {
            //         let asset = asset_index
            //             .get_by_name_hash(texture.name_hash)
            //             .expect("Texture not found");
            //         println!(" - Depends on texture '{}'", asset.name);
            //     } else if let RvmType::Dx11ByteCodeElement(bytecode) = block {
            //         bytecode_count += 1;
            //     }
            // }

            //println!(" - {} bytecodes", bytecode_count);
        }

        Ok(())
    }

    async fn post_mutation(
        &self,
        _domain: &DomainStore,
        storage: &PipelineStorage,
        _result: &mut UnbuildResult,
    ) -> Result<(), PipelineMutationError> {
        let storage = storage.read().await;
        let count = storage.get::<RvmDatabaseByteCount>().unwrap();

        println!("Total database size: {}", count);

        let unique_dependencies = storage.get::<RvmDatabaseUniqueDependencyCount>().unwrap();

        let mut total_duplicates = 0;
        for (block_hash, dependencies) in unique_dependencies {
            if dependencies.len() == 1 {
                continue;
            }

            // println!(
            //     "Block {} has {} dependency sets",
            //     block_hash,
            //     dependencies.len()
            // );

            // for (i, deps) in dependencies.iter().enumerate() {
            //     println!("  Set {}: {:?}", i, deps);
            // }

            total_duplicates += 1;
        }

        println!(
            "{} total shaders, {} duplicates",
            unique_dependencies.len(),
            total_duplicates
        );

        Ok(())
    }
}
