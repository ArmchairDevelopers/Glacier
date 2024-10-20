use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_frosted_strings_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(LOCALIZATIONLANGUAGECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(FSUITEXTDATABASE_TYPE_INFO);
    registry.register_type(FSUITEXTDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(FSLOCALIZATIONASSET_TYPE_INFO);
    registry.register_type(FSLOCALIZATIONASSET_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct LocalizationLanguageChangedMessage {
}

pub trait LocalizationLanguageChangedMessageTrait: TypeObject {
}

impl LocalizationLanguageChangedMessageTrait for LocalizationLanguageChangedMessage {
}

pub static LOCALIZATIONLANGUAGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizationLanguageChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "FrostedStringsShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizationLanguageChangedMessage as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LocalizationLanguageChangedMessage {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZATIONLANGUAGECHANGEDMESSAGE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct FsUITextDatabase {
    pub _glacier_base: super::u_i::UITextDatabase,
}

pub trait FsUITextDatabaseTrait: super::u_i::UITextDatabaseTrait {
}

impl FsUITextDatabaseTrait for FsUITextDatabase {
}

impl super::u_i::UITextDatabaseTrait for FsUITextDatabase {
    fn language(&self) -> &super::core::LanguageFormat {
        self._glacier_base.language()
    }
    fn binary_chunk(&self) -> &glacier_util::guid::Guid {
        self._glacier_base.binary_chunk()
    }
    fn binary_chunk_size(&self) -> &u32 {
        self._glacier_base.binary_chunk_size()
    }
    fn histogram_chunk(&self) -> &glacier_util::guid::Guid {
        self._glacier_base.histogram_chunk()
    }
    fn histogram_chunk_size(&self) -> &u32 {
        self._glacier_base.histogram_chunk_size()
    }
}

impl super::core::AssetTrait for FsUITextDatabase {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for FsUITextDatabase {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FSUITEXTDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsUITextDatabase",
    flags: MemberInfoFlags::new(101),
    module: "FrostedStringsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::u_i::UITEXTDATABASE_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FsUITextDatabase as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FSUITEXTDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FsUITextDatabase {
    fn type_info(&self) -> &'static TypeInfo {
        FSUITEXTDATABASE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FSUITEXTDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsUITextDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "FrostedStringsShared",
    data: TypeInfoData::Array("FsUITextDatabase"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FsLocalizationAsset {
    pub _glacier_base: super::u_i::LocalizationAsset,
}

pub trait FsLocalizationAssetTrait: super::u_i::LocalizationAssetTrait {
}

impl FsLocalizationAssetTrait for FsLocalizationAsset {
}

impl super::u_i::LocalizationAssetTrait for FsLocalizationAsset {
    fn localized_texts(&self) -> &Vec<Option<Arc<Mutex<dyn super::u_i::UITextDatabaseTrait>>>> {
        self._glacier_base.localized_texts()
    }
}

impl super::core::AssetTrait for FsLocalizationAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
}

impl super::core::DataContainerTrait for FsLocalizationAsset {
    fn dc_core(&self) -> &glacier_reflect::data_container::DataContainerCore {
        self._glacier_base.dc_core()
    }
}

pub static FSLOCALIZATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsLocalizationAsset",
    flags: MemberInfoFlags::new(101),
    module: "FrostedStringsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::u_i::LOCALIZATIONASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FsLocalizationAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(FSLOCALIZATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FsLocalizationAsset {
    fn type_info(&self) -> &'static TypeInfo {
        FSLOCALIZATIONASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}


pub static FSLOCALIZATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsLocalizationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "FrostedStringsShared",
    data: TypeInfoData::Array("FsLocalizationAsset"),
    array_type: None,
    alignment: 8,
};


