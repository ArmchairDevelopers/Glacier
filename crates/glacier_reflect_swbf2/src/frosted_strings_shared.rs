use std::mem::offset_of;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_frosted_strings_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(LOCALIZATIONLANGUAGECHANGEDMESSAGE_TYPE_INFO);
    registry.register_type(FSUITEXTDATABASE_TYPE_INFO);
    registry.register_type(FSUITEXTDATABASE_ARRAY_TYPE_INFO);
    registry.register_type(FSLOCALIZATIONASSET_TYPE_INFO);
    registry.register_type(FSLOCALIZATIONASSET_ARRAY_TYPE_INFO);
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocalizationLanguageChangedMessage {
}

pub const LOCALIZATIONLANGUAGECHANGEDMESSAGE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizationLanguageChangedMessage",
    flags: MemberInfoFlags::new(36937),
    module: "FrostedStringsShared",
    data: TypeInfoData::Value(ValueTypeInfoData {
        fields: &[
        ],
    }),
    array_type: None,
    alignment: 8,
};

impl TypeObject for LocalizationLanguageChangedMessage {
    fn type_info() -> &'static TypeInfo {
        LOCALIZATIONLANGUAGECHANGEDMESSAGE_TYPE_INFO
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FsUITextDatabase {
}

pub const FSUITEXTDATABASE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsUITextDatabase",
    flags: MemberInfoFlags::new(101),
    module: "FrostedStringsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(UITEXTDATABASE_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FSUITEXTDATABASE_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FsUITextDatabase {
    fn type_info() -> &'static TypeInfo {
        FSUITEXTDATABASE_TYPE_INFO
    }
}


pub const FSUITEXTDATABASE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsUITextDatabase-Array",
    flags: MemberInfoFlags::new(145),
    module: "FrostedStringsShared",
    data: TypeInfoData::Array("FsUITextDatabase-Array"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FsLocalizationAsset {
}

pub const FSLOCALIZATIONASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsLocalizationAsset",
    flags: MemberInfoFlags::new(101),
    module: "FrostedStringsShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZATIONASSET_TYPE_INFO),
        fields: &[
        ],
    }),
    array_type: Some(FSLOCALIZATIONASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FsLocalizationAsset {
    fn type_info() -> &'static TypeInfo {
        FSLOCALIZATIONASSET_TYPE_INFO
    }
}


pub const FSLOCALIZATIONASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FsLocalizationAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "FrostedStringsShared",
    data: TypeInfoData::Array("FsLocalizationAsset-Array"),
    array_type: None,
    alignment: 8,
};


