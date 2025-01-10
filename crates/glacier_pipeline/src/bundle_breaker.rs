use std::collections::{HashMap, HashSet};

use glacier_fs::{
    db::partition::DatabasePartition, dbx::reader::DbxPartitionImportLoader, fb::FrostbiteGameData,
};
use glacier_reflect::data_container::DataContainerCore;
use glacier_reflect_custom::{ResourceBundleAsset, RESOURCEBUNDLEASSET_TYPE_INFO};
use glacier_reflect_swbf2::core::{Asset, DataContainer};
use glacier_store::domain::{DomainStore, StubDbxPartitionImportLoader};
use glacier_util::{guid::Guid, hash::QuickHashExt};
use tracing::{debug, info};

struct SharedBundleNameResolver {
    bundles: HashMap<u32, &'static str>,
}

impl SharedBundleNameResolver {
    pub fn new() -> Self {
        Self::load(&[
            "Gameplay/Bundles/SharedBundles/Frontend+MP/Abilities/SharedBundleAbilities_Frontend+MP",
            "Gameplay/Bundles/SharedBundles/Common/Animation/SharedBundleAnimation_common",
            "Gameplay/Bundles/SharedBundles/Frontend+MP/Characters/SharedBundleCharacters_Frontend+PM",
            "Gameplay/Bundles/SharedBundles/Common/Vehicles/SharedBundleVehiclesCockpits",
            "Gameplay/Bundles/SharedBundles/Common/Characters/SharedBundleCharacters1p",
            "UI/Frontend/WebBrowser/WebBrowserResourceBundle",
            "Systems/FrostbiteStartupData",
            "Gameplay/WRGameConfiguration",
            "default_settings",
            "S1/Gameplay/Bundles/Sharedbundleseason1",
            "Gameplay/Bundles/SP/Vehicle/SharedBundle_SP_Vehicle",
            "Gameplay/Bundles/SP/SharedBundle_SP",
            "Gameplay/Bundles/SP/Player/SharedBundle_SP_Player",
            "Gameplay/Bundles/SP/Droid/SharedBundle_SP_Droid",
            "Gameplay/Bundles/SP/Buddy/SharedBundle_SP_Buddy",
            "Gameplay/Bundles/SharedBundles/SP/Vehicles/SharedBundleVehicles_SP",
            "Gameplay/Bundles/SharedBundles/SP/Abilities/SharedBundleAbilities_SP",
            "A3/Gameplay/Bundles/SP/Vehicle/SharedBundle_SP_Vehicle_A3",
            "A3/Gameplay/Bundles/SP/SharedBundle_SP_A3",
            "A3/Gameplay/Bundles/SP/Player/SharedBundle_SP_Player_A3",
            "A3/Gameplay/Bundles/SP/Buddy/SharedBundle_SP_Buddy_A3",
            "UI/Static",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_WorstCase",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_TraditionalChinese",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_SpanishMex",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_Spanish",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_SimplifiedChinese",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_Russian",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_Polish",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_Korean",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_Japanese",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_Italian",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_German",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_French",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_English",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_BrazilianPortuguese",
            "ui/resources/fonts/WSUIIMFontConfiguration_LanguageFormat_ArabicSA",
            "Sound/SP/Music/Screens/SW02_SP_Music_Loading_BundleAsset:8f175ba0-cac3-44bd-87d1-710706c09278",
            "Sound/Music/Loading/SW02_Music_Loading_BundleAsset_initialexperience:c2d5acdb-7a2f-4742-9499-2cd74fefec4c",
            "Sound/Music/Loading/SW02_Music_Loading_BundleAsset:db988158-79d3-489b-96d8-970deca60c67",
            "LoadingScreens_bundle",
            "Gameplay/Bundles/SharedBundles/Common/Weapons/SharedBundleWeapons_common",
            "Persistence/WSMPPersistence"
        ])
    }

    fn load(bundles: &[&'static str]) -> Self {
        let mut map: HashMap<u32, &'static str> = HashMap::new();

        for bundle in bundles {
            map.insert(
                format!("win32/{}", bundle.to_lowercase()).hash_quick(),
                bundle,
            );
        }

        Self { bundles: map }
    }

    pub fn get(&self, hash: u32) -> Option<&'static str> {
        self.bundles.get(&hash).copied()
    }
}

pub struct BundleBreaker {
    resolver: SharedBundleNameResolver,
}

impl BundleBreaker {
    pub fn new() -> Self {
        Self {
            resolver: SharedBundleNameResolver::new(),
        }
    }

    pub async fn execute(&self, domain: &DomainStore, data: &FrostbiteGameData) {
        let mut stub_loader = StubDbxPartitionImportLoader {
            index: domain.index().clone(),
        };

        for bundle in &data.bundles {
            let bundle_name = match self.resolver.get(bundle.hash) {
                Some(name) => name,
                None => continue,
            };

            let mut reference_set = HashSet::new();
    
            {
                let asset_index = domain.index_read().await;
                for entry in asset_index.iter() {
                    if !entry.bundles.contains(&bundle.hash) {
                        continue;
                    }

                    for import in &entry.imports {
                        reference_set.insert(*import);
                    }

                    for import in &entry.res_imports {
                        reference_set.insert(*import);
                    }
                }
            }

            debug!("Breaking down bundle '{}'", bundle_name);

            let mut root_assets = Vec::new();

            {
                let asset_index = domain.index_read().await;
                for entry in &bundle.ebx_entries {
                    if let Some(data) = asset_index.get_by_name(&entry.name) {
                        if !reference_set.contains(&data.partition) {
                            root_assets.push((data.partition, data.instances[0].guid));
                        }
                    }
                }
            }

            if root_assets.is_empty() {
                continue;
            }

            info!(
                "Bundle '{}' has {} root assets",
                bundle_name,
                root_assets.len()
            );

            let mut partition = DatabasePartition::new_empty(bundle_name.to_owned(), Guid::random());
            let instance = partition
                .create_instance(RESOURCEBUNDLEASSET_TYPE_INFO)
                .await
                .expect("Failed to create instance");

            {
                let mut instance = instance.lock().await;
                let resource = instance
                    .as_any_mut()
                    .downcast_mut::<ResourceBundleAsset>()
                    .unwrap();
                resource._glacier_base.name = bundle_name.to_owned();

                for asset in root_assets {
                    let instance = stub_loader
                        .load_partition(asset.0, asset.1)
                        .await
                        .expect("Failed to load partition");
                    resource.assets.push(Some(instance));
                }
            }

            if let Err(err) = domain.save_asset(&partition).await {
                tracing::error!("Failed to save asset: {}", err);
            }

            // let resource_bundle = ResourceBundleAsset {
            //     _glacier_base: Asset {
            //         _glacier_base: DataContainer {
            //             _glacier_dc_core: DataContainerCore {
            //                 id: String,
            //                 partition_guid: Guid,
            //                 instance_guid: Guid,
            //                 exported: bool,
            //             }
            //         },
            //         name: todo!(),
            //     },
            //     assets: vec![]
            // };
        }
    }
}
