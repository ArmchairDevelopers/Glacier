use std::{mem::offset_of, any::Any, option::Option, sync::Arc};
use tokio::sync::Mutex;

use glacier_reflect::{
    member::MemberInfoFlags,
    type_info::{
        ClassInfoData, ValueTypeInfoData, FieldInfoData, TypeInfo, TypeInfoData, TypeObject, TypeFunctions,
    }, type_registry::TypeRegistry,
};

pub(crate) fn register_u_i_incubator_shared_types(registry: &mut TypeRegistry) {
    registry.register_type(UIMASKINGWIDGETENTITYDATA_TYPE_INFO);
    registry.register_type(UIMASKINGWIDGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGSWITCHCASEENTITYDATA_TYPE_INFO);
    registry.register_type(STRINGSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGSWITCHCOMPARETYPE_TYPE_INFO);
    registry.register_type(STRINGSWITCHCOMPARETYPE_ARRAY_TYPE_INFO);
    registry.register_type(INTEGERSWITCHCASEENTITYDATA_TYPE_INFO);
    registry.register_type(INTEGERSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(BASESWITCHCASEENTITYDATA_TYPE_INFO);
    registry.register_type(BASESWITCHCASEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATENODEENTITYDATA_TYPE_INFO);
    registry.register_type(STATENODEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STATENAVEVENTINFO_TYPE_INFO);
    registry.register_type(STATENAVEVENTINFO_ARRAY_TYPE_INFO);
    registry.register_type(STATENODEENTITYBASEDATA_TYPE_INFO);
    registry.register_type(STATENODEENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(SELECTOBJECTENTITYDATA_TYPE_INFO);
    registry.register_type(SELECTOBJECTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(QUITGAMEENTITYDATA_TYPE_INFO);
    registry.register_type(QUITGAMEENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(TRANSFORMINTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(TRANSFORMINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC4INTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(VEC4INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3INTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(VEC3INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC2INTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(VEC2INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATINTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(FLOATINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONMODE_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONMODE_ARRAY_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONTYPE_TYPE_INFO);
    registry.register_type(PROPERTYINTERPOLATIONTYPE_ARRAY_TYPE_INFO);
    registry.register_type(OBJECTHUBENTITYDATA_TYPE_INFO);
    registry.register_type(OBJECTHUBENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(MATHINTOPENTITYDATA_TYPE_INFO);
    registry.register_type(MATHINTOPENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTMATHOP_TYPE_INFO);
    registry.register_type(INTMATHOP_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENTITYBASEDATA_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGARGUMENTTYPE_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGID_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGID_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRING_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRING_ARRAY_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENCODING_TYPE_INFO);
    registry.register_type(LOCALIZEDSTRINGENCODING_ARRAY_TYPE_INFO);
    registry.register_type(TEXTURESWITCHENTITYDATA_TYPE_INFO);
    registry.register_type(TEXTURESWITCHENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUILISTITEMWIDGETENTITYDATA_TYPE_INFO);
    registry.register_type(FBUILISTITEMWIDGETENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUILISTELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUILISTELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUISLICEDTEXTUREELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUISLICEDTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUIMOVIEELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUIMOVIEELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUIDYNAMICTEXTUREELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUIDYNAMICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUISTATICTEXTUREELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUISTATICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(FBUILABELELEMENTENTITYDATA_TYPE_INFO);
    registry.register_type(FBUILABELELEMENTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_TYPE_INFO);
    registry.register_type(UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_ARRAY_TYPE_INFO);
    registry.register_type(DYNAMICCASTENTITYDATA_TYPE_INFO);
    registry.register_type(DYNAMICCASTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGENTITYDATA_TYPE_INFO);
    registry.register_type(CONFIGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGLISTASSET_TYPE_INFO);
    registry.register_type(STRINGLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(INTLISTASSET_TYPE_INFO);
    registry.register_type(INTLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(FLOATLISTASSET_TYPE_INFO);
    registry.register_type(FLOATLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(COLORLISTASSET_TYPE_INFO);
    registry.register_type(COLORLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(VEC3LISTASSET_TYPE_INFO);
    registry.register_type(VEC3LISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGLISTASSET_TYPE_INFO);
    registry.register_type(CONFIGLISTASSET_ARRAY_TYPE_INFO);
    registry.register_type(CONFIGENTITYASSETDATA_TYPE_INFO);
    registry.register_type(CONFIGENTITYASSETDATA_ARRAY_TYPE_INFO);
    registry.register_type(STRINGDATA_TYPE_INFO);
    registry.register_type(STRINGDATA_ARRAY_TYPE_INFO);
    registry.register_type(INTDATA_TYPE_INFO);
    registry.register_type(INTDATA_ARRAY_TYPE_INFO);
    registry.register_type(FLOATDATA_TYPE_INFO);
    registry.register_type(FLOATDATA_ARRAY_TYPE_INFO);
    registry.register_type(COLORDATA_TYPE_INFO);
    registry.register_type(COLORDATA_ARRAY_TYPE_INFO);
    registry.register_type(VEC3DATA_TYPE_INFO);
    registry.register_type(VEC3DATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALSTRINGENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALSTRINGENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALTRANSFORMENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALTRANSFORMENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC4ENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALVEC4ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC3ENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALVEC3ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALVEC2ENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALVEC2ENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALFLOATENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALFLOATENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALINTENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALINTENTITYDATA_ARRAY_TYPE_INFO);
    registry.register_type(CONDITIONALSTATEENTITYDATA_TYPE_INFO);
    registry.register_type(CONDITIONALSTATEENTITYDATA_ARRAY_TYPE_INFO);
}

#[derive(Clone, Debug, Default)]
pub struct UIMaskingWidgetEntityData {
    pub _glacier_base: super::game_shared_u_i::UIWidgetEntityData,
    pub mask_texture_id: String,
    pub mask_alpha: f32,
    pub invert_mask: bool,
}

pub trait UIMaskingWidgetEntityDataTrait: super::game_shared_u_i::UIWidgetEntityDataTrait {
    fn mask_texture_id(&self) -> &String;
    fn mask_texture_id_mut(&mut self) -> &mut String;
    fn mask_alpha(&self) -> &f32;
    fn mask_alpha_mut(&mut self) -> &mut f32;
    fn invert_mask(&self) -> &bool;
    fn invert_mask_mut(&mut self) -> &mut bool;
}

impl UIMaskingWidgetEntityDataTrait for UIMaskingWidgetEntityData {
    fn mask_texture_id(&self) -> &String {
        &self.mask_texture_id
    }
    fn mask_texture_id_mut(&mut self) -> &mut String {
        &mut self.mask_texture_id
    }
    fn mask_alpha(&self) -> &f32 {
        &self.mask_alpha
    }
    fn mask_alpha_mut(&mut self) -> &mut f32 {
        &mut self.mask_alpha
    }
    fn invert_mask(&self) -> &bool {
        &self.invert_mask
    }
    fn invert_mask_mut(&mut self) -> &mut bool {
        &mut self.invert_mask
    }
}

impl super::game_shared_u_i::UIWidgetEntityDataTrait for UIMaskingWidgetEntityData {
    fn size(&self) -> &super::game_shared_u_i::UIElementSize {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::game_shared_u_i::UIElementSize {
        self._glacier_base.size_mut()
    }
    fn layers(&self) -> &Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementLayerEntityDataTrait>>>> {
        self._glacier_base.layers()
    }
    fn layers_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementLayerEntityDataTrait>>>> {
        self._glacier_base.layers_mut()
    }
    fn texture_mappings(&self) -> &Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UITextureMappingAssetTrait>>>> {
        self._glacier_base.texture_mappings()
    }
    fn texture_mappings_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UITextureMappingAssetTrait>>>> {
        self._glacier_base.texture_mappings_mut()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
}

impl super::entity::EntityDataTrait for UIMaskingWidgetEntityData {
}

impl super::entity::GameObjectDataTrait for UIMaskingWidgetEntityData {
}

impl super::core::DataBusPeerTrait for UIMaskingWidgetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for UIMaskingWidgetEntityData {
}

impl super::core::DataContainerTrait for UIMaskingWidgetEntityData {
}

pub static UIMASKINGWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingWidgetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIWIDGETENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIMaskingWidgetEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "MaskTextureId",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(UIMaskingWidgetEntityData, mask_texture_id),
            },
            FieldInfoData {
                name: "MaskAlpha",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(UIMaskingWidgetEntityData, mask_alpha),
            },
            FieldInfoData {
                name: "InvertMask",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIMaskingWidgetEntityData, invert_mask),
            },
        ],
    }),
    array_type: Some(UIMASKINGWIDGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for UIMaskingWidgetEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        UIMASKINGWIDGETENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static UIMASKINGWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIMaskingWidgetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("UIMaskingWidgetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StringSwitchCaseEntityData {
    pub _glacier_base: BaseSwitchCaseEntityData,
    pub cases: Vec<String>,
    pub in_value: String,
    pub case_sensitive_compare: bool,
    pub compare_type: StringSwitchCompareType,
}

pub trait StringSwitchCaseEntityDataTrait: BaseSwitchCaseEntityDataTrait {
    fn cases(&self) -> &Vec<String>;
    fn cases_mut(&mut self) -> &mut Vec<String>;
    fn in_value(&self) -> &String;
    fn in_value_mut(&mut self) -> &mut String;
    fn case_sensitive_compare(&self) -> &bool;
    fn case_sensitive_compare_mut(&mut self) -> &mut bool;
    fn compare_type(&self) -> &StringSwitchCompareType;
    fn compare_type_mut(&mut self) -> &mut StringSwitchCompareType;
}

impl StringSwitchCaseEntityDataTrait for StringSwitchCaseEntityData {
    fn cases(&self) -> &Vec<String> {
        &self.cases
    }
    fn cases_mut(&mut self) -> &mut Vec<String> {
        &mut self.cases
    }
    fn in_value(&self) -> &String {
        &self.in_value
    }
    fn in_value_mut(&mut self) -> &mut String {
        &mut self.in_value
    }
    fn case_sensitive_compare(&self) -> &bool {
        &self.case_sensitive_compare
    }
    fn case_sensitive_compare_mut(&mut self) -> &mut bool {
        &mut self.case_sensitive_compare
    }
    fn compare_type(&self) -> &StringSwitchCompareType {
        &self.compare_type
    }
    fn compare_type_mut(&mut self) -> &mut StringSwitchCompareType {
        &mut self.compare_type
    }
}

impl BaseSwitchCaseEntityDataTrait for StringSwitchCaseEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn trigger_on_property_change(&self) -> &bool {
        self._glacier_base.trigger_on_property_change()
    }
    fn trigger_on_property_change_mut(&mut self) -> &mut bool {
        self._glacier_base.trigger_on_property_change_mut()
    }
    fn trigger_on_start(&self) -> &bool {
        self._glacier_base.trigger_on_start()
    }
    fn trigger_on_start_mut(&mut self) -> &mut bool {
        self._glacier_base.trigger_on_start_mut()
    }
}

impl super::entity::EntityDataTrait for StringSwitchCaseEntityData {
}

impl super::entity::GameObjectDataTrait for StringSwitchCaseEntityData {
}

impl super::core::DataBusPeerTrait for StringSwitchCaseEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StringSwitchCaseEntityData {
}

impl super::core::DataContainerTrait for StringSwitchCaseEntityData {
}

pub static STRINGSWITCHCASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESWITCHCASEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StringSwitchCaseEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Cases",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(StringSwitchCaseEntityData, cases),
            },
            FieldInfoData {
                name: "InValue",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StringSwitchCaseEntityData, in_value),
            },
            FieldInfoData {
                name: "CaseSensitiveCompare",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(StringSwitchCaseEntityData, case_sensitive_compare),
            },
            FieldInfoData {
                name: "CompareType",
                flags: MemberInfoFlags::new(0),
                field_type: "StringSwitchCompareType",
                rust_offset: offset_of!(StringSwitchCaseEntityData, compare_type),
            },
        ],
    }),
    array_type: Some(STRINGSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringSwitchCaseEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STRINGSWITCHCASEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STRINGSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringSwitchCaseEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum StringSwitchCompareType {
    #[default]
    StringSwitchCompareType_Equals = 0,
    StringSwitchCompareType_Contains = 1,
    StringSwitchCompareType_StartsWith = 2,
    StringSwitchCompareType_EndsWith = 3,
}

pub static STRINGSWITCHCOMPARETYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCompareType",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(STRINGSWITCHCOMPARETYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for StringSwitchCompareType {
    fn type_info(&self) -> &'static TypeInfo {
        STRINGSWITCHCOMPARETYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static STRINGSWITCHCOMPARETYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringSwitchCompareType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringSwitchCompareType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntegerSwitchCaseEntityData {
    pub _glacier_base: BaseSwitchCaseEntityData,
    pub cases: Vec<i32>,
    pub in_value: i32,
}

pub trait IntegerSwitchCaseEntityDataTrait: BaseSwitchCaseEntityDataTrait {
    fn cases(&self) -> &Vec<i32>;
    fn cases_mut(&mut self) -> &mut Vec<i32>;
    fn in_value(&self) -> &i32;
    fn in_value_mut(&mut self) -> &mut i32;
}

impl IntegerSwitchCaseEntityDataTrait for IntegerSwitchCaseEntityData {
    fn cases(&self) -> &Vec<i32> {
        &self.cases
    }
    fn cases_mut(&mut self) -> &mut Vec<i32> {
        &mut self.cases
    }
    fn in_value(&self) -> &i32 {
        &self.in_value
    }
    fn in_value_mut(&mut self) -> &mut i32 {
        &mut self.in_value
    }
}

impl BaseSwitchCaseEntityDataTrait for IntegerSwitchCaseEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn trigger_on_property_change(&self) -> &bool {
        self._glacier_base.trigger_on_property_change()
    }
    fn trigger_on_property_change_mut(&mut self) -> &mut bool {
        self._glacier_base.trigger_on_property_change_mut()
    }
    fn trigger_on_start(&self) -> &bool {
        self._glacier_base.trigger_on_start()
    }
    fn trigger_on_start_mut(&mut self) -> &mut bool {
        self._glacier_base.trigger_on_start_mut()
    }
}

impl super::entity::EntityDataTrait for IntegerSwitchCaseEntityData {
}

impl super::entity::GameObjectDataTrait for IntegerSwitchCaseEntityData {
}

impl super::core::DataBusPeerTrait for IntegerSwitchCaseEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for IntegerSwitchCaseEntityData {
}

impl super::core::DataContainerTrait for IntegerSwitchCaseEntityData {
}

pub static INTEGERSWITCHCASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(BASESWITCHCASEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntegerSwitchCaseEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Cases",
                flags: MemberInfoFlags::new(144),
                field_type: "Int32-Array",
                rust_offset: offset_of!(IntegerSwitchCaseEntityData, cases),
            },
            FieldInfoData {
                name: "InValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IntegerSwitchCaseEntityData, in_value),
            },
        ],
    }),
    array_type: Some(INTEGERSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntegerSwitchCaseEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        INTEGERSWITCHCASEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INTEGERSWITCHCASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntegerSwitchCaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntegerSwitchCaseEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct BaseSwitchCaseEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub trigger_on_property_change: bool,
    pub trigger_on_start: bool,
}

pub trait BaseSwitchCaseEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn trigger_on_property_change(&self) -> &bool;
    fn trigger_on_property_change_mut(&mut self) -> &mut bool;
    fn trigger_on_start(&self) -> &bool;
    fn trigger_on_start_mut(&mut self) -> &mut bool;
}

impl BaseSwitchCaseEntityDataTrait for BaseSwitchCaseEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn trigger_on_property_change(&self) -> &bool {
        &self.trigger_on_property_change
    }
    fn trigger_on_property_change_mut(&mut self) -> &mut bool {
        &mut self.trigger_on_property_change
    }
    fn trigger_on_start(&self) -> &bool {
        &self.trigger_on_start
    }
    fn trigger_on_start_mut(&mut self) -> &mut bool {
        &mut self.trigger_on_start
    }
}

impl super::entity::EntityDataTrait for BaseSwitchCaseEntityData {
}

impl super::entity::GameObjectDataTrait for BaseSwitchCaseEntityData {
}

impl super::core::DataBusPeerTrait for BaseSwitchCaseEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for BaseSwitchCaseEntityData {
}

impl super::core::DataContainerTrait for BaseSwitchCaseEntityData {
}

pub static BASESWITCHCASEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseSwitchCaseEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<BaseSwitchCaseEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(BaseSwitchCaseEntityData, realm),
            },
            FieldInfoData {
                name: "TriggerOnPropertyChange",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BaseSwitchCaseEntityData, trigger_on_property_change),
            },
            FieldInfoData {
                name: "TriggerOnStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(BaseSwitchCaseEntityData, trigger_on_start),
            },
        ],
    }),
    array_type: Some(BASESWITCHCASEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for BaseSwitchCaseEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        BASESWITCHCASEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static BASESWITCHCASEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "BaseSwitchCaseEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("BaseSwitchCaseEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StateNodeEntityData {
    pub _glacier_base: StateNodeEntityBaseData,
    pub event_triggers_info: Vec<StateNavEventInfo>,
    pub consumed_events_info: Vec<StateNavEventInfo>,
}

pub trait StateNodeEntityDataTrait: StateNodeEntityBaseDataTrait {
    fn event_triggers_info(&self) -> &Vec<StateNavEventInfo>;
    fn event_triggers_info_mut(&mut self) -> &mut Vec<StateNavEventInfo>;
    fn consumed_events_info(&self) -> &Vec<StateNavEventInfo>;
    fn consumed_events_info_mut(&mut self) -> &mut Vec<StateNavEventInfo>;
}

impl StateNodeEntityDataTrait for StateNodeEntityData {
    fn event_triggers_info(&self) -> &Vec<StateNavEventInfo> {
        &self.event_triggers_info
    }
    fn event_triggers_info_mut(&mut self) -> &mut Vec<StateNavEventInfo> {
        &mut self.event_triggers_info
    }
    fn consumed_events_info(&self) -> &Vec<StateNavEventInfo> {
        &self.consumed_events_info
    }
    fn consumed_events_info_mut(&mut self) -> &mut Vec<StateNavEventInfo> {
        &mut self.consumed_events_info
    }
}

impl StateNodeEntityBaseDataTrait for StateNodeEntityData {
    fn state_name(&self) -> &String {
        self._glacier_base.state_name()
    }
    fn state_name_mut(&mut self) -> &mut String {
        self._glacier_base.state_name_mut()
    }
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
}

impl super::entity::EntityDataTrait for StateNodeEntityData {
}

impl super::entity::GameObjectDataTrait for StateNodeEntityData {
}

impl super::core::DataBusPeerTrait for StateNodeEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StateNodeEntityData {
}

impl super::core::DataContainerTrait for StateNodeEntityData {
}

pub static STATENODEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(STATENODEENTITYBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StateNodeEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "EventTriggersInfo",
                flags: MemberInfoFlags::new(144),
                field_type: "StateNavEventInfo-Array",
                rust_offset: offset_of!(StateNodeEntityData, event_triggers_info),
            },
            FieldInfoData {
                name: "ConsumedEventsInfo",
                flags: MemberInfoFlags::new(144),
                field_type: "StateNavEventInfo-Array",
                rust_offset: offset_of!(StateNodeEntityData, consumed_events_info),
            },
        ],
    }),
    array_type: Some(STATENODEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StateNodeEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        STATENODEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STATENODEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StateNodeEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StateNavEventInfo {
    pub trigger_event_hash: i32,
    pub on_event_hash: i32,
}

pub trait StateNavEventInfoTrait: TypeObject {
    fn trigger_event_hash(&self) -> &i32;
    fn trigger_event_hash_mut(&mut self) -> &mut i32;
    fn on_event_hash(&self) -> &i32;
    fn on_event_hash_mut(&mut self) -> &mut i32;
}

impl StateNavEventInfoTrait for StateNavEventInfo {
    fn trigger_event_hash(&self) -> &i32 {
        &self.trigger_event_hash
    }
    fn trigger_event_hash_mut(&mut self) -> &mut i32 {
        &mut self.trigger_event_hash
    }
    fn on_event_hash(&self) -> &i32 {
        &self.on_event_hash
    }
    fn on_event_hash_mut(&mut self) -> &mut i32 {
        &mut self.on_event_hash
    }
}

pub static STATENAVEVENTINFO_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNavEventInfo",
    flags: MemberInfoFlags::new(36937),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StateNavEventInfo as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "TriggerEventHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StateNavEventInfo, trigger_event_hash),
            },
            FieldInfoData {
                name: "OnEventHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(StateNavEventInfo, on_event_hash),
            },
        ],
    }),
    array_type: Some(STATENAVEVENTINFO_ARRAY_TYPE_INFO),
    alignment: 4,
};

impl TypeObject for StateNavEventInfo {
    fn type_info(&self) -> &'static TypeInfo {
        STATENAVEVENTINFO_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static STATENAVEVENTINFO_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNavEventInfo-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StateNavEventInfo"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StateNodeEntityBaseData {
    pub _glacier_base: super::entity::EntityData,
    pub state_name: String,
    pub realm: super::core::Realm,
}

pub trait StateNodeEntityBaseDataTrait: super::entity::EntityDataTrait {
    fn state_name(&self) -> &String;
    fn state_name_mut(&mut self) -> &mut String;
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
}

impl StateNodeEntityBaseDataTrait for StateNodeEntityBaseData {
    fn state_name(&self) -> &String {
        &self.state_name
    }
    fn state_name_mut(&mut self) -> &mut String {
        &mut self.state_name
    }
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
}

impl super::entity::EntityDataTrait for StateNodeEntityBaseData {
}

impl super::entity::GameObjectDataTrait for StateNodeEntityBaseData {
}

impl super::core::DataBusPeerTrait for StateNodeEntityBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for StateNodeEntityBaseData {
}

impl super::core::DataContainerTrait for StateNodeEntityBaseData {
}

pub static STATENODEENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StateNodeEntityBaseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StateName",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StateNodeEntityBaseData, state_name),
            },
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(StateNodeEntityBaseData, realm),
            },
        ],
    }),
    array_type: Some(STATENODEENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for StateNodeEntityBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        STATENODEENTITYBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STATENODEENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StateNodeEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StateNodeEntityBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct SelectObjectEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub inputs: Vec<String>,
    pub input_select: i32,
    pub dynamic_input_data_type: u32,
}

pub trait SelectObjectEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn inputs(&self) -> &Vec<String>;
    fn inputs_mut(&mut self) -> &mut Vec<String>;
    fn input_select(&self) -> &i32;
    fn input_select_mut(&mut self) -> &mut i32;
    fn dynamic_input_data_type(&self) -> &u32;
    fn dynamic_input_data_type_mut(&mut self) -> &mut u32;
}

impl SelectObjectEntityDataTrait for SelectObjectEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn inputs(&self) -> &Vec<String> {
        &self.inputs
    }
    fn inputs_mut(&mut self) -> &mut Vec<String> {
        &mut self.inputs
    }
    fn input_select(&self) -> &i32 {
        &self.input_select
    }
    fn input_select_mut(&mut self) -> &mut i32 {
        &mut self.input_select
    }
    fn dynamic_input_data_type(&self) -> &u32 {
        &self.dynamic_input_data_type
    }
    fn dynamic_input_data_type_mut(&mut self) -> &mut u32 {
        &mut self.dynamic_input_data_type
    }
}

impl super::entity::EntityDataTrait for SelectObjectEntityData {
}

impl super::entity::GameObjectDataTrait for SelectObjectEntityData {
}

impl super::core::DataBusPeerTrait for SelectObjectEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for SelectObjectEntityData {
}

impl super::core::DataContainerTrait for SelectObjectEntityData {
}

pub static SELECTOBJECTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<SelectObjectEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(SelectObjectEntityData, realm),
            },
            FieldInfoData {
                name: "Inputs",
                flags: MemberInfoFlags::new(144),
                field_type: "CString-Array",
                rust_offset: offset_of!(SelectObjectEntityData, inputs),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(SelectObjectEntityData, input_select),
            },
            FieldInfoData {
                name: "DynamicInputDataType",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(SelectObjectEntityData, dynamic_input_data_type),
            },
        ],
    }),
    array_type: Some(SELECTOBJECTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for SelectObjectEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        SELECTOBJECTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static SELECTOBJECTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "SelectObjectEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("SelectObjectEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct QuitGameEntityData {
    pub _glacier_base: super::entity::EntityData,
}

pub trait QuitGameEntityDataTrait: super::entity::EntityDataTrait {
}

impl QuitGameEntityDataTrait for QuitGameEntityData {
}

impl super::entity::EntityDataTrait for QuitGameEntityData {
}

impl super::entity::GameObjectDataTrait for QuitGameEntityData {
}

impl super::core::DataBusPeerTrait for QuitGameEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for QuitGameEntityData {
}

impl super::core::DataContainerTrait for QuitGameEntityData {
}

pub static QUITGAMEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuitGameEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<QuitGameEntityData as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(QUITGAMEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for QuitGameEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        QUITGAMEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static QUITGAMEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "QuitGameEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("QuitGameEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TransformInterpolatorEntityData {
    pub _glacier_base: PropertyInterpolatorEntityData,
    pub r#in: super::core::LinearTransform,
    pub default_value: super::core::LinearTransform,
}

pub trait TransformInterpolatorEntityDataTrait: PropertyInterpolatorEntityDataTrait {
    fn r#in(&self) -> &super::core::LinearTransform;
    fn r#in_mut(&mut self) -> &mut super::core::LinearTransform;
    fn default_value(&self) -> &super::core::LinearTransform;
    fn default_value_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl TransformInterpolatorEntityDataTrait for TransformInterpolatorEntityData {
    fn r#in(&self) -> &super::core::LinearTransform {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.r#in
    }
    fn default_value(&self) -> &super::core::LinearTransform {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.default_value
    }
}

impl PropertyInterpolatorEntityDataTrait for TransformInterpolatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn interpolation_type(&self) -> &PropertyInterpolationType {
        self._glacier_base.interpolation_type()
    }
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType {
        self._glacier_base.interpolation_type_mut()
    }
    fn interpolation_mode(&self) -> &PropertyInterpolationMode {
        self._glacier_base.interpolation_mode()
    }
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode {
        self._glacier_base.interpolation_mode_mut()
    }
    fn duration(&self) -> &f32 {
        self._glacier_base.duration()
    }
    fn duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.duration_mut()
    }
    fn use_real_time_clock(&self) -> &bool {
        self._glacier_base.use_real_time_clock()
    }
    fn use_real_time_clock_mut(&mut self) -> &mut bool {
        self._glacier_base.use_real_time_clock_mut()
    }
    fn force_frame_correct_output(&self) -> &bool {
        self._glacier_base.force_frame_correct_output()
    }
    fn force_frame_correct_output_mut(&mut self) -> &mut bool {
        self._glacier_base.force_frame_correct_output_mut()
    }
}

impl super::entity::EntityDataTrait for TransformInterpolatorEntityData {
}

impl super::entity::GameObjectDataTrait for TransformInterpolatorEntityData {
}

impl super::core::DataBusPeerTrait for TransformInterpolatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TransformInterpolatorEntityData {
}

impl super::core::DataContainerTrait for TransformInterpolatorEntityData {
}

pub static TRANSFORMINTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TransformInterpolatorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(TransformInterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(TransformInterpolatorEntityData, default_value),
            },
        ],
    }),
    array_type: Some(TRANSFORMINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for TransformInterpolatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TRANSFORMINTERPOLATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TRANSFORMINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TransformInterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("TransformInterpolatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec4InterpolatorEntityData {
    pub _glacier_base: PropertyInterpolatorEntityData,
    pub r#in: super::core::Vec4,
    pub default_value: super::core::Vec4,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub trait Vec4InterpolatorEntityDataTrait: PropertyInterpolatorEntityDataTrait {
    fn r#in(&self) -> &super::core::Vec4;
    fn r#in_mut(&mut self) -> &mut super::core::Vec4;
    fn default_value(&self) -> &super::core::Vec4;
    fn default_value_mut(&mut self) -> &mut super::core::Vec4;
    fn use_velocity(&self) -> &bool;
    fn use_velocity_mut(&mut self) -> &mut bool;
    fn velocity(&self) -> &f32;
    fn velocity_mut(&mut self) -> &mut f32;
}

impl Vec4InterpolatorEntityDataTrait for Vec4InterpolatorEntityData {
    fn r#in(&self) -> &super::core::Vec4 {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.r#in
    }
    fn default_value(&self) -> &super::core::Vec4 {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.default_value
    }
    fn use_velocity(&self) -> &bool {
        &self.use_velocity
    }
    fn use_velocity_mut(&mut self) -> &mut bool {
        &mut self.use_velocity
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
    fn velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity
    }
}

impl PropertyInterpolatorEntityDataTrait for Vec4InterpolatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn interpolation_type(&self) -> &PropertyInterpolationType {
        self._glacier_base.interpolation_type()
    }
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType {
        self._glacier_base.interpolation_type_mut()
    }
    fn interpolation_mode(&self) -> &PropertyInterpolationMode {
        self._glacier_base.interpolation_mode()
    }
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode {
        self._glacier_base.interpolation_mode_mut()
    }
    fn duration(&self) -> &f32 {
        self._glacier_base.duration()
    }
    fn duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.duration_mut()
    }
    fn use_real_time_clock(&self) -> &bool {
        self._glacier_base.use_real_time_clock()
    }
    fn use_real_time_clock_mut(&mut self) -> &mut bool {
        self._glacier_base.use_real_time_clock_mut()
    }
    fn force_frame_correct_output(&self) -> &bool {
        self._glacier_base.force_frame_correct_output()
    }
    fn force_frame_correct_output_mut(&mut self) -> &mut bool {
        self._glacier_base.force_frame_correct_output_mut()
    }
}

impl super::entity::EntityDataTrait for Vec4InterpolatorEntityData {
}

impl super::entity::GameObjectDataTrait for Vec4InterpolatorEntityData {
}

impl super::core::DataBusPeerTrait for Vec4InterpolatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for Vec4InterpolatorEntityData {
}

impl super::core::DataContainerTrait for Vec4InterpolatorEntityData {
}

pub static VEC4INTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec4InterpolatorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(Vec4InterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(Vec4InterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Vec4InterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec4InterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(VEC4INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec4InterpolatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEC4INTERPOLATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VEC4INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec4InterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec4InterpolatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3InterpolatorEntityData {
    pub _glacier_base: PropertyInterpolatorEntityData,
    pub r#in: super::core::Vec3,
    pub default_value: super::core::Vec3,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub trait Vec3InterpolatorEntityDataTrait: PropertyInterpolatorEntityDataTrait {
    fn r#in(&self) -> &super::core::Vec3;
    fn r#in_mut(&mut self) -> &mut super::core::Vec3;
    fn default_value(&self) -> &super::core::Vec3;
    fn default_value_mut(&mut self) -> &mut super::core::Vec3;
    fn use_velocity(&self) -> &bool;
    fn use_velocity_mut(&mut self) -> &mut bool;
    fn velocity(&self) -> &f32;
    fn velocity_mut(&mut self) -> &mut f32;
}

impl Vec3InterpolatorEntityDataTrait for Vec3InterpolatorEntityData {
    fn r#in(&self) -> &super::core::Vec3 {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.r#in
    }
    fn default_value(&self) -> &super::core::Vec3 {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.default_value
    }
    fn use_velocity(&self) -> &bool {
        &self.use_velocity
    }
    fn use_velocity_mut(&mut self) -> &mut bool {
        &mut self.use_velocity
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
    fn velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity
    }
}

impl PropertyInterpolatorEntityDataTrait for Vec3InterpolatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn interpolation_type(&self) -> &PropertyInterpolationType {
        self._glacier_base.interpolation_type()
    }
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType {
        self._glacier_base.interpolation_type_mut()
    }
    fn interpolation_mode(&self) -> &PropertyInterpolationMode {
        self._glacier_base.interpolation_mode()
    }
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode {
        self._glacier_base.interpolation_mode_mut()
    }
    fn duration(&self) -> &f32 {
        self._glacier_base.duration()
    }
    fn duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.duration_mut()
    }
    fn use_real_time_clock(&self) -> &bool {
        self._glacier_base.use_real_time_clock()
    }
    fn use_real_time_clock_mut(&mut self) -> &mut bool {
        self._glacier_base.use_real_time_clock_mut()
    }
    fn force_frame_correct_output(&self) -> &bool {
        self._glacier_base.force_frame_correct_output()
    }
    fn force_frame_correct_output_mut(&mut self) -> &mut bool {
        self._glacier_base.force_frame_correct_output_mut()
    }
}

impl super::entity::EntityDataTrait for Vec3InterpolatorEntityData {
}

impl super::entity::GameObjectDataTrait for Vec3InterpolatorEntityData {
}

impl super::core::DataBusPeerTrait for Vec3InterpolatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for Vec3InterpolatorEntityData {
}

impl super::core::DataContainerTrait for Vec3InterpolatorEntityData {
}

pub static VEC3INTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3InterpolatorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Vec3InterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Vec3InterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Vec3InterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec3InterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(VEC3INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3InterpolatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3INTERPOLATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VEC3INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3InterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec3InterpolatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec2InterpolatorEntityData {
    pub _glacier_base: PropertyInterpolatorEntityData,
    pub r#in: super::core::Vec2,
    pub default_value: super::core::Vec2,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub trait Vec2InterpolatorEntityDataTrait: PropertyInterpolatorEntityDataTrait {
    fn r#in(&self) -> &super::core::Vec2;
    fn r#in_mut(&mut self) -> &mut super::core::Vec2;
    fn default_value(&self) -> &super::core::Vec2;
    fn default_value_mut(&mut self) -> &mut super::core::Vec2;
    fn use_velocity(&self) -> &bool;
    fn use_velocity_mut(&mut self) -> &mut bool;
    fn velocity(&self) -> &f32;
    fn velocity_mut(&mut self) -> &mut f32;
}

impl Vec2InterpolatorEntityDataTrait for Vec2InterpolatorEntityData {
    fn r#in(&self) -> &super::core::Vec2 {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.r#in
    }
    fn default_value(&self) -> &super::core::Vec2 {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.default_value
    }
    fn use_velocity(&self) -> &bool {
        &self.use_velocity
    }
    fn use_velocity_mut(&mut self) -> &mut bool {
        &mut self.use_velocity
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
    fn velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity
    }
}

impl PropertyInterpolatorEntityDataTrait for Vec2InterpolatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn interpolation_type(&self) -> &PropertyInterpolationType {
        self._glacier_base.interpolation_type()
    }
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType {
        self._glacier_base.interpolation_type_mut()
    }
    fn interpolation_mode(&self) -> &PropertyInterpolationMode {
        self._glacier_base.interpolation_mode()
    }
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode {
        self._glacier_base.interpolation_mode_mut()
    }
    fn duration(&self) -> &f32 {
        self._glacier_base.duration()
    }
    fn duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.duration_mut()
    }
    fn use_real_time_clock(&self) -> &bool {
        self._glacier_base.use_real_time_clock()
    }
    fn use_real_time_clock_mut(&mut self) -> &mut bool {
        self._glacier_base.use_real_time_clock_mut()
    }
    fn force_frame_correct_output(&self) -> &bool {
        self._glacier_base.force_frame_correct_output()
    }
    fn force_frame_correct_output_mut(&mut self) -> &mut bool {
        self._glacier_base.force_frame_correct_output_mut()
    }
}

impl super::entity::EntityDataTrait for Vec2InterpolatorEntityData {
}

impl super::entity::GameObjectDataTrait for Vec2InterpolatorEntityData {
}

impl super::core::DataBusPeerTrait for Vec2InterpolatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for Vec2InterpolatorEntityData {
}

impl super::core::DataContainerTrait for Vec2InterpolatorEntityData {
}

pub static VEC2INTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec2InterpolatorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(Vec2InterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(Vec2InterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(Vec2InterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(Vec2InterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(VEC2INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec2InterpolatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        VEC2INTERPOLATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VEC2INTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec2InterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec2InterpolatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatInterpolatorEntityData {
    pub _glacier_base: PropertyInterpolatorEntityData,
    pub r#in: f32,
    pub default_value: f32,
    pub use_velocity: bool,
    pub velocity: f32,
}

pub trait FloatInterpolatorEntityDataTrait: PropertyInterpolatorEntityDataTrait {
    fn r#in(&self) -> &f32;
    fn r#in_mut(&mut self) -> &mut f32;
    fn default_value(&self) -> &f32;
    fn default_value_mut(&mut self) -> &mut f32;
    fn use_velocity(&self) -> &bool;
    fn use_velocity_mut(&mut self) -> &mut bool;
    fn velocity(&self) -> &f32;
    fn velocity_mut(&mut self) -> &mut f32;
}

impl FloatInterpolatorEntityDataTrait for FloatInterpolatorEntityData {
    fn r#in(&self) -> &f32 {
        &self.r#in
    }
    fn r#in_mut(&mut self) -> &mut f32 {
        &mut self.r#in
    }
    fn default_value(&self) -> &f32 {
        &self.default_value
    }
    fn default_value_mut(&mut self) -> &mut f32 {
        &mut self.default_value
    }
    fn use_velocity(&self) -> &bool {
        &self.use_velocity
    }
    fn use_velocity_mut(&mut self) -> &mut bool {
        &mut self.use_velocity
    }
    fn velocity(&self) -> &f32 {
        &self.velocity
    }
    fn velocity_mut(&mut self) -> &mut f32 {
        &mut self.velocity
    }
}

impl PropertyInterpolatorEntityDataTrait for FloatInterpolatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn interpolation_type(&self) -> &PropertyInterpolationType {
        self._glacier_base.interpolation_type()
    }
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType {
        self._glacier_base.interpolation_type_mut()
    }
    fn interpolation_mode(&self) -> &PropertyInterpolationMode {
        self._glacier_base.interpolation_mode()
    }
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode {
        self._glacier_base.interpolation_mode_mut()
    }
    fn duration(&self) -> &f32 {
        self._glacier_base.duration()
    }
    fn duration_mut(&mut self) -> &mut f32 {
        self._glacier_base.duration_mut()
    }
    fn use_real_time_clock(&self) -> &bool {
        self._glacier_base.use_real_time_clock()
    }
    fn use_real_time_clock_mut(&mut self) -> &mut bool {
        self._glacier_base.use_real_time_clock_mut()
    }
    fn force_frame_correct_output(&self) -> &bool {
        self._glacier_base.force_frame_correct_output()
    }
    fn force_frame_correct_output_mut(&mut self) -> &mut bool {
        self._glacier_base.force_frame_correct_output_mut()
    }
}

impl super::entity::EntityDataTrait for FloatInterpolatorEntityData {
}

impl super::entity::GameObjectDataTrait for FloatInterpolatorEntityData {
}

impl super::core::DataBusPeerTrait for FloatInterpolatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FloatInterpolatorEntityData {
}

impl super::core::DataContainerTrait for FloatInterpolatorEntityData {
}

pub static FLOATINTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatInterpolatorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "In",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatInterpolatorEntityData, r#in),
            },
            FieldInfoData {
                name: "DefaultValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatInterpolatorEntityData, default_value),
            },
            FieldInfoData {
                name: "UseVelocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FloatInterpolatorEntityData, use_velocity),
            },
            FieldInfoData {
                name: "Velocity",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatInterpolatorEntityData, velocity),
            },
        ],
    }),
    array_type: Some(FLOATINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatInterpolatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATINTERPOLATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FLOATINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatInterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FloatInterpolatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct PropertyInterpolatorEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub interpolation_type: PropertyInterpolationType,
    pub interpolation_mode: PropertyInterpolationMode,
    pub duration: f32,
    pub use_real_time_clock: bool,
    pub force_frame_correct_output: bool,
}

pub trait PropertyInterpolatorEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn interpolation_type(&self) -> &PropertyInterpolationType;
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType;
    fn interpolation_mode(&self) -> &PropertyInterpolationMode;
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode;
    fn duration(&self) -> &f32;
    fn duration_mut(&mut self) -> &mut f32;
    fn use_real_time_clock(&self) -> &bool;
    fn use_real_time_clock_mut(&mut self) -> &mut bool;
    fn force_frame_correct_output(&self) -> &bool;
    fn force_frame_correct_output_mut(&mut self) -> &mut bool;
}

impl PropertyInterpolatorEntityDataTrait for PropertyInterpolatorEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn interpolation_type(&self) -> &PropertyInterpolationType {
        &self.interpolation_type
    }
    fn interpolation_type_mut(&mut self) -> &mut PropertyInterpolationType {
        &mut self.interpolation_type
    }
    fn interpolation_mode(&self) -> &PropertyInterpolationMode {
        &self.interpolation_mode
    }
    fn interpolation_mode_mut(&mut self) -> &mut PropertyInterpolationMode {
        &mut self.interpolation_mode
    }
    fn duration(&self) -> &f32 {
        &self.duration
    }
    fn duration_mut(&mut self) -> &mut f32 {
        &mut self.duration
    }
    fn use_real_time_clock(&self) -> &bool {
        &self.use_real_time_clock
    }
    fn use_real_time_clock_mut(&mut self) -> &mut bool {
        &mut self.use_real_time_clock
    }
    fn force_frame_correct_output(&self) -> &bool {
        &self.force_frame_correct_output
    }
    fn force_frame_correct_output_mut(&mut self) -> &mut bool {
        &mut self.force_frame_correct_output
    }
}

impl super::entity::EntityDataTrait for PropertyInterpolatorEntityData {
}

impl super::entity::GameObjectDataTrait for PropertyInterpolatorEntityData {
}

impl super::core::DataBusPeerTrait for PropertyInterpolatorEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for PropertyInterpolatorEntityData {
}

impl super::core::DataContainerTrait for PropertyInterpolatorEntityData {
}

pub static PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolatorEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<PropertyInterpolatorEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(PropertyInterpolatorEntityData, realm),
            },
            FieldInfoData {
                name: "InterpolationType",
                flags: MemberInfoFlags::new(0),
                field_type: "PropertyInterpolationType",
                rust_offset: offset_of!(PropertyInterpolatorEntityData, interpolation_type),
            },
            FieldInfoData {
                name: "InterpolationMode",
                flags: MemberInfoFlags::new(0),
                field_type: "PropertyInterpolationMode",
                rust_offset: offset_of!(PropertyInterpolatorEntityData, interpolation_mode),
            },
            FieldInfoData {
                name: "Duration",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(PropertyInterpolatorEntityData, duration),
            },
            FieldInfoData {
                name: "UseRealTimeClock",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropertyInterpolatorEntityData, use_real_time_clock),
            },
            FieldInfoData {
                name: "ForceFrameCorrectOutput",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(PropertyInterpolatorEntityData, force_frame_correct_output),
            },
        ],
    }),
    array_type: Some(PROPERTYINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for PropertyInterpolatorEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYINTERPOLATORENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static PROPERTYINTERPOLATORENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolatorEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("PropertyInterpolatorEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PropertyInterpolationMode {
    #[default]
    PropertyInterpolationMode_EaseIn = 0,
    PropertyInterpolationMode_EaseOut = 1,
    PropertyInterpolationMode_EaseInOut = 2,
    PropertyInterpolationMode_EaseOutIn = 3,
    PropertyInterpolationMode_Count = 4,
}

pub static PROPERTYINTERPOLATIONMODE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationMode",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(PROPERTYINTERPOLATIONMODE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropertyInterpolationMode {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYINTERPOLATIONMODE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PROPERTYINTERPOLATIONMODE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationMode-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("PropertyInterpolationMode"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum PropertyInterpolationType {
    #[default]
    PropertyInterpolationType_Linear = 0,
    PropertyInterpolationType_Quad = 1,
    PropertyInterpolationType_Cubic = 2,
    PropertyInterpolationType_Quart = 3,
    PropertyInterpolationType_Quint = 4,
    PropertyInterpolationType_Expo = 5,
    PropertyInterpolationType_Sine = 6,
    PropertyInterpolationType_Circ = 7,
    PropertyInterpolationType_Back = 8,
    PropertyInterpolationType_Elastic = 9,
    PropertyInterpolationType_Bounce = 10,
    PropertyInterpolationType_Count = 11,
}

pub static PROPERTYINTERPOLATIONTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationType",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(PROPERTYINTERPOLATIONTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for PropertyInterpolationType {
    fn type_info(&self) -> &'static TypeInfo {
        PROPERTYINTERPOLATIONTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static PROPERTYINTERPOLATIONTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "PropertyInterpolationType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("PropertyInterpolationType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ObjectHubEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub input_select: i32,
    pub dynamic_input_data_type: u32,
}

pub trait ObjectHubEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn input_select(&self) -> &i32;
    fn input_select_mut(&mut self) -> &mut i32;
    fn dynamic_input_data_type(&self) -> &u32;
    fn dynamic_input_data_type_mut(&mut self) -> &mut u32;
}

impl ObjectHubEntityDataTrait for ObjectHubEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn input_select(&self) -> &i32 {
        &self.input_select
    }
    fn input_select_mut(&mut self) -> &mut i32 {
        &mut self.input_select
    }
    fn dynamic_input_data_type(&self) -> &u32 {
        &self.dynamic_input_data_type
    }
    fn dynamic_input_data_type_mut(&mut self) -> &mut u32 {
        &mut self.dynamic_input_data_type
    }
}

impl super::entity::EntityDataTrait for ObjectHubEntityData {
}

impl super::entity::GameObjectDataTrait for ObjectHubEntityData {
}

impl super::core::DataBusPeerTrait for ObjectHubEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ObjectHubEntityData {
}

impl super::core::DataContainerTrait for ObjectHubEntityData {
}

pub static OBJECTHUBENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ObjectHubEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ObjectHubEntityData, realm),
            },
            FieldInfoData {
                name: "InputSelect",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ObjectHubEntityData, input_select),
            },
            FieldInfoData {
                name: "DynamicInputDataType",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(ObjectHubEntityData, dynamic_input_data_type),
            },
        ],
    }),
    array_type: Some(OBJECTHUBENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ObjectHubEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        OBJECTHUBENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static OBJECTHUBENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ObjectHubEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ObjectHubEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct MathIntOpEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub operators: Vec<IntMathOp>,
}

pub trait MathIntOpEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn operators(&self) -> &Vec<IntMathOp>;
    fn operators_mut(&mut self) -> &mut Vec<IntMathOp>;
}

impl MathIntOpEntityDataTrait for MathIntOpEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn operators(&self) -> &Vec<IntMathOp> {
        &self.operators
    }
    fn operators_mut(&mut self) -> &mut Vec<IntMathOp> {
        &mut self.operators
    }
}

impl super::entity::EntityDataTrait for MathIntOpEntityData {
}

impl super::entity::GameObjectDataTrait for MathIntOpEntityData {
}

impl super::core::DataBusPeerTrait for MathIntOpEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for MathIntOpEntityData {
}

impl super::core::DataContainerTrait for MathIntOpEntityData {
}

pub static MATHINTOPENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<MathIntOpEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(MathIntOpEntityData, realm),
            },
            FieldInfoData {
                name: "Operators",
                flags: MemberInfoFlags::new(144),
                field_type: "IntMathOp-Array",
                rust_offset: offset_of!(MathIntOpEntityData, operators),
            },
        ],
    }),
    array_type: Some(MATHINTOPENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for MathIntOpEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        MATHINTOPENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static MATHINTOPENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "MathIntOpEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("MathIntOpEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum IntMathOp {
    #[default]
    IntMathOp_Add = 0,
    IntMathOp_Subtract = 1,
    IntMathOp_Multiply = 2,
    IntMathOp_Divide = 3,
    IntMathOp_Min = 4,
    IntMathOp_Max = 5,
    IntMathOp_Modulo = 6,
    IntMathOp_Exponent = 7,
}

pub static INTMATHOP_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntMathOp",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(INTMATHOP_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for IntMathOp {
    fn type_info(&self) -> &'static TypeInfo {
        INTMATHOP_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INTMATHOP_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntMathOp-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntMathOp"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedStringEntityData {
    pub _glacier_base: LocalizedStringEntityBaseData,
    pub sid: String,
}

pub trait LocalizedStringEntityDataTrait: LocalizedStringEntityBaseDataTrait {
    fn sid(&self) -> &String;
    fn sid_mut(&mut self) -> &mut String;
}

impl LocalizedStringEntityDataTrait for LocalizedStringEntityData {
    fn sid(&self) -> &String {
        &self.sid
    }
    fn sid_mut(&mut self) -> &mut String {
        &mut self.sid
    }
}

impl LocalizedStringEntityBaseDataTrait for LocalizedStringEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn arguments(&self) -> &Vec<LocalizedStringArgumentType> {
        self._glacier_base.arguments()
    }
    fn arguments_mut(&mut self) -> &mut Vec<LocalizedStringArgumentType> {
        self._glacier_base.arguments_mut()
    }
    fn argument_hashes(&self) -> &Vec<u32> {
        self._glacier_base.argument_hashes()
    }
    fn argument_hashes_mut(&mut self) -> &mut Vec<u32> {
        self._glacier_base.argument_hashes_mut()
    }
}

impl super::entity::EntityDataTrait for LocalizedStringEntityData {
}

impl super::entity::GameObjectDataTrait for LocalizedStringEntityData {
}

impl super::core::DataBusPeerTrait for LocalizedStringEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocalizedStringEntityData {
}

impl super::core::DataContainerTrait for LocalizedStringEntityData {
}

pub static LOCALIZEDSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Sid",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LocalizedStringEntityData, sid),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALIZEDSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedStringEntityBaseData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub arguments: Vec<LocalizedStringArgumentType>,
    pub argument_hashes: Vec<u32>,
}

pub trait LocalizedStringEntityBaseDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn arguments(&self) -> &Vec<LocalizedStringArgumentType>;
    fn arguments_mut(&mut self) -> &mut Vec<LocalizedStringArgumentType>;
    fn argument_hashes(&self) -> &Vec<u32>;
    fn argument_hashes_mut(&mut self) -> &mut Vec<u32>;
}

impl LocalizedStringEntityBaseDataTrait for LocalizedStringEntityBaseData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn arguments(&self) -> &Vec<LocalizedStringArgumentType> {
        &self.arguments
    }
    fn arguments_mut(&mut self) -> &mut Vec<LocalizedStringArgumentType> {
        &mut self.arguments
    }
    fn argument_hashes(&self) -> &Vec<u32> {
        &self.argument_hashes
    }
    fn argument_hashes_mut(&mut self) -> &mut Vec<u32> {
        &mut self.argument_hashes
    }
}

impl super::entity::EntityDataTrait for LocalizedStringEntityBaseData {
}

impl super::entity::GameObjectDataTrait for LocalizedStringEntityBaseData {
}

impl super::core::DataBusPeerTrait for LocalizedStringEntityBaseData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for LocalizedStringEntityBaseData {
}

impl super::core::DataContainerTrait for LocalizedStringEntityBaseData {
}

pub static LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBaseData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringEntityBaseData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(LocalizedStringEntityBaseData, realm),
            },
            FieldInfoData {
                name: "Arguments",
                flags: MemberInfoFlags::new(144),
                field_type: "LocalizedStringArgumentType-Array",
                rust_offset: offset_of!(LocalizedStringEntityBaseData, arguments),
            },
            FieldInfoData {
                name: "ArgumentHashes",
                flags: MemberInfoFlags::new(144),
                field_type: "Uint32-Array",
                rust_offset: offset_of!(LocalizedStringEntityBaseData, argument_hashes),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGENTITYBASEDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for LocalizedStringEntityBaseData {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGENTITYBASEDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALIZEDSTRINGENTITYBASEDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEntityBaseData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringEntityBaseData"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocalizedStringArgumentType {
    #[default]
    LocalizedStringArgumentType_Integer = 0,
    LocalizedStringArgumentType_Long = 1,
    LocalizedStringArgumentType_Float = 2,
    LocalizedStringArgumentType_String = 3,
    LocalizedStringArgumentType_LocalizedString = 4,
    LocalizedStringArgumentType_Date = 5,
    LocalizedStringArgumentType_Time = 6,
}

pub static LOCALIZEDSTRINGARGUMENTTYPE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringArgumentType",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalizedStringArgumentType {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGARGUMENTTYPE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LOCALIZEDSTRINGARGUMENTTYPE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringArgumentType-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringArgumentType"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedStringId {
    pub _glacier_base: super::core::DataContainer,
    pub string_hash: i32,
}

pub trait LocalizedStringIdTrait: super::core::DataContainerTrait {
    fn string_hash(&self) -> &i32;
    fn string_hash_mut(&mut self) -> &mut i32;
}

impl LocalizedStringIdTrait for LocalizedStringId {
    fn string_hash(&self) -> &i32 {
        &self.string_hash
    }
    fn string_hash_mut(&mut self) -> &mut i32 {
        &mut self.string_hash
    }
}

impl super::core::DataContainerTrait for LocalizedStringId {
}

pub static LOCALIZEDSTRINGID_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringId",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedStringId as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StringHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(LocalizedStringId, string_hash),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRINGID_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedStringId {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGID_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALIZEDSTRINGID_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringId-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringId"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct LocalizedString {
    pub _glacier_base: super::core::DataContainer,
    pub string: String,
    pub encoding: LocalizedStringEncoding,
}

pub trait LocalizedStringTrait: super::core::DataContainerTrait {
    fn string(&self) -> &String;
    fn string_mut(&mut self) -> &mut String;
    fn encoding(&self) -> &LocalizedStringEncoding;
    fn encoding_mut(&mut self) -> &mut LocalizedStringEncoding;
}

impl LocalizedStringTrait for LocalizedString {
    fn string(&self) -> &String {
        &self.string
    }
    fn string_mut(&mut self) -> &mut String {
        &mut self.string
    }
    fn encoding(&self) -> &LocalizedStringEncoding {
        &self.encoding
    }
    fn encoding_mut(&mut self) -> &mut LocalizedStringEncoding {
        &mut self.encoding
    }
}

impl super::core::DataContainerTrait for LocalizedString {
}

pub static LOCALIZEDSTRING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedString",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINER_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<LocalizedString as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "String",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(LocalizedString, string),
            },
            FieldInfoData {
                name: "Encoding",
                flags: MemberInfoFlags::new(0),
                field_type: "LocalizedStringEncoding",
                rust_offset: offset_of!(LocalizedString, encoding),
            },
        ],
    }),
    array_type: Some(LOCALIZEDSTRING_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for LocalizedString {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static LOCALIZEDSTRING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedString-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedString"),
    array_type: None,
    alignment: 8,
};


#[derive(Hash, Clone, Copy, PartialEq, Default, Debug)]
#[repr(i64)]
#[allow(non_camel_case_types)]
pub enum LocalizedStringEncoding {
    #[default]
    LocalizedStringEncoding_Unknown = 0,
    LocalizedStringEncoding_Packed = 1,
    LocalizedStringEncoding_UTF8 = 2,
}

pub static LOCALIZEDSTRINGENCODING_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEncoding",
    flags: MemberInfoFlags::new(49429),
    module: "UIIncubatorShared",
    data: TypeInfoData::Enum,
    array_type: Some(LOCALIZEDSTRINGENCODING_ARRAY_TYPE_INFO),
    alignment: 1,
};

impl TypeObject for LocalizedStringEncoding {
    fn type_info(&self) -> &'static TypeInfo {
        LOCALIZEDSTRINGENCODING_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static LOCALIZEDSTRINGENCODING_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "LocalizedStringEncoding-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("LocalizedStringEncoding"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct TextureSwitchEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub textures: Vec<Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>>,
}

pub trait TextureSwitchEntityDataTrait: super::entity::EntityDataTrait {
    fn textures(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>>;
    fn textures_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>>;
}

impl TextureSwitchEntityDataTrait for TextureSwitchEntityData {
    fn textures(&self) -> &Vec<Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>> {
        &self.textures
    }
    fn textures_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::render::TextureAssetTrait>>>> {
        &mut self.textures
    }
}

impl super::entity::EntityDataTrait for TextureSwitchEntityData {
}

impl super::entity::GameObjectDataTrait for TextureSwitchEntityData {
}

impl super::core::DataBusPeerTrait for TextureSwitchEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for TextureSwitchEntityData {
}

impl super::core::DataContainerTrait for TextureSwitchEntityData {
}

pub static TEXTURESWITCHENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<TextureSwitchEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Textures",
                flags: MemberInfoFlags::new(144),
                field_type: "TextureAsset-Array",
                rust_offset: offset_of!(TextureSwitchEntityData, textures),
            },
        ],
    }),
    array_type: Some(TEXTURESWITCHENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for TextureSwitchEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        TEXTURESWITCHENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static TEXTURESWITCHENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "TextureSwitchEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("TextureSwitchEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIListItemWidgetEntityData {
    pub _glacier_base: super::game_shared_u_i::UIWidgetEntityData,
    pub out_type_hash: i32,
}

pub trait FBUIListItemWidgetEntityDataTrait: super::game_shared_u_i::UIWidgetEntityDataTrait {
    fn out_type_hash(&self) -> &i32;
    fn out_type_hash_mut(&mut self) -> &mut i32;
}

impl FBUIListItemWidgetEntityDataTrait for FBUIListItemWidgetEntityData {
    fn out_type_hash(&self) -> &i32 {
        &self.out_type_hash
    }
    fn out_type_hash_mut(&mut self) -> &mut i32 {
        &mut self.out_type_hash
    }
}

impl super::game_shared_u_i::UIWidgetEntityDataTrait for FBUIListItemWidgetEntityData {
    fn size(&self) -> &super::game_shared_u_i::UIElementSize {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::game_shared_u_i::UIElementSize {
        self._glacier_base.size_mut()
    }
    fn layers(&self) -> &Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementLayerEntityDataTrait>>>> {
        self._glacier_base.layers()
    }
    fn layers_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementLayerEntityDataTrait>>>> {
        self._glacier_base.layers_mut()
    }
    fn texture_mappings(&self) -> &Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UITextureMappingAssetTrait>>>> {
        self._glacier_base.texture_mappings()
    }
    fn texture_mappings_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::game_shared_u_i::UITextureMappingAssetTrait>>>> {
        self._glacier_base.texture_mappings_mut()
    }
    fn components(&self) -> &Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components()
    }
    fn components_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn super::entity::GameObjectDataTrait>>>> {
        self._glacier_base.components_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
}

impl super::entity::EntityDataTrait for FBUIListItemWidgetEntityData {
}

impl super::entity::GameObjectDataTrait for FBUIListItemWidgetEntityData {
}

impl super::core::DataBusPeerTrait for FBUIListItemWidgetEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUIListItemWidgetEntityData {
}

impl super::core::DataContainerTrait for FBUIListItemWidgetEntityData {
}

pub static FBUILISTITEMWIDGETENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIWIDGETENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIListItemWidgetEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "OutTypeHash",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUIListItemWidgetEntityData, out_type_hash),
            },
        ],
    }),
    array_type: Some(FBUILISTITEMWIDGETENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FBUIListItemWidgetEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUILISTITEMWIDGETENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUILISTITEMWIDGETENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListItemWidgetEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIListItemWidgetEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIListElementEntityData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub in_array: Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>,
    pub row_template: Option<Arc<Mutex<dyn super::game_shared_u_i::UIWidgetBlueprintTrait>>>,
    pub row_height: u32,
}

pub trait FBUIListElementEntityDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn in_array(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>;
    fn in_array_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>;
    fn row_template(&self) -> &Option<Arc<Mutex<dyn super::game_shared_u_i::UIWidgetBlueprintTrait>>>;
    fn row_template_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared_u_i::UIWidgetBlueprintTrait>>>;
    fn row_height(&self) -> &u32;
    fn row_height_mut(&mut self) -> &mut u32;
}

impl FBUIListElementEntityDataTrait for FBUIListElementEntityData {
    fn in_array(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>> {
        &self.in_array
    }
    fn in_array_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::DataContainerTrait>>> {
        &mut self.in_array
    }
    fn row_template(&self) -> &Option<Arc<Mutex<dyn super::game_shared_u_i::UIWidgetBlueprintTrait>>> {
        &self.row_template
    }
    fn row_template_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared_u_i::UIWidgetBlueprintTrait>>> {
        &mut self.row_template
    }
    fn row_height(&self) -> &u32 {
        &self.row_height
    }
    fn row_height_mut(&mut self) -> &mut u32 {
        &mut self.row_height
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for FBUIListElementEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for FBUIListElementEntityData {
}

impl super::entity::GameObjectDataTrait for FBUIListElementEntityData {
}

impl super::core::DataBusPeerTrait for FBUIListElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUIListElementEntityData {
}

impl super::core::DataContainerTrait for FBUIListElementEntityData {
}

pub static FBUILISTELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIListElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "InArray",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(FBUIListElementEntityData, in_array),
            },
            FieldInfoData {
                name: "RowTemplate",
                flags: MemberInfoFlags::new(0),
                field_type: "UIWidgetBlueprint",
                rust_offset: offset_of!(FBUIListElementEntityData, row_template),
            },
            FieldInfoData {
                name: "RowHeight",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(FBUIListElementEntityData, row_height),
            },
        ],
    }),
    array_type: Some(FBUILISTELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIListElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUILISTELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUILISTELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIListElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIListElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUISlicedTextureElementEntityData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub texture: super::game_shared_u_i::UIAutoMappedTexture,
    pub slice_left: i32,
    pub slice_top: i32,
    pub slice_right: i32,
    pub slice_bottom: i32,
    pub padding_left: i32,
    pub padding_top: i32,
    pub padding_right: i32,
    pub padding_bottom: i32,
    pub fill_center: bool,
}

pub trait FBUISlicedTextureElementEntityDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn texture(&self) -> &super::game_shared_u_i::UIAutoMappedTexture;
    fn texture_mut(&mut self) -> &mut super::game_shared_u_i::UIAutoMappedTexture;
    fn slice_left(&self) -> &i32;
    fn slice_left_mut(&mut self) -> &mut i32;
    fn slice_top(&self) -> &i32;
    fn slice_top_mut(&mut self) -> &mut i32;
    fn slice_right(&self) -> &i32;
    fn slice_right_mut(&mut self) -> &mut i32;
    fn slice_bottom(&self) -> &i32;
    fn slice_bottom_mut(&mut self) -> &mut i32;
    fn padding_left(&self) -> &i32;
    fn padding_left_mut(&mut self) -> &mut i32;
    fn padding_top(&self) -> &i32;
    fn padding_top_mut(&mut self) -> &mut i32;
    fn padding_right(&self) -> &i32;
    fn padding_right_mut(&mut self) -> &mut i32;
    fn padding_bottom(&self) -> &i32;
    fn padding_bottom_mut(&mut self) -> &mut i32;
    fn fill_center(&self) -> &bool;
    fn fill_center_mut(&mut self) -> &mut bool;
}

impl FBUISlicedTextureElementEntityDataTrait for FBUISlicedTextureElementEntityData {
    fn texture(&self) -> &super::game_shared_u_i::UIAutoMappedTexture {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::game_shared_u_i::UIAutoMappedTexture {
        &mut self.texture
    }
    fn slice_left(&self) -> &i32 {
        &self.slice_left
    }
    fn slice_left_mut(&mut self) -> &mut i32 {
        &mut self.slice_left
    }
    fn slice_top(&self) -> &i32 {
        &self.slice_top
    }
    fn slice_top_mut(&mut self) -> &mut i32 {
        &mut self.slice_top
    }
    fn slice_right(&self) -> &i32 {
        &self.slice_right
    }
    fn slice_right_mut(&mut self) -> &mut i32 {
        &mut self.slice_right
    }
    fn slice_bottom(&self) -> &i32 {
        &self.slice_bottom
    }
    fn slice_bottom_mut(&mut self) -> &mut i32 {
        &mut self.slice_bottom
    }
    fn padding_left(&self) -> &i32 {
        &self.padding_left
    }
    fn padding_left_mut(&mut self) -> &mut i32 {
        &mut self.padding_left
    }
    fn padding_top(&self) -> &i32 {
        &self.padding_top
    }
    fn padding_top_mut(&mut self) -> &mut i32 {
        &mut self.padding_top
    }
    fn padding_right(&self) -> &i32 {
        &self.padding_right
    }
    fn padding_right_mut(&mut self) -> &mut i32 {
        &mut self.padding_right
    }
    fn padding_bottom(&self) -> &i32 {
        &self.padding_bottom
    }
    fn padding_bottom_mut(&mut self) -> &mut i32 {
        &mut self.padding_bottom
    }
    fn fill_center(&self) -> &bool {
        &self.fill_center
    }
    fn fill_center_mut(&mut self) -> &mut bool {
        &mut self.fill_center
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for FBUISlicedTextureElementEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for FBUISlicedTextureElementEntityData {
}

impl super::entity::GameObjectDataTrait for FBUISlicedTextureElementEntityData {
}

impl super::core::DataBusPeerTrait for FBUISlicedTextureElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUISlicedTextureElementEntityData {
}

impl super::core::DataContainerTrait for FBUISlicedTextureElementEntityData {
}

pub static FBUISLICEDTEXTUREELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUISlicedTextureElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "UIAutoMappedTexture",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, texture),
            },
            FieldInfoData {
                name: "SliceLeft",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_left),
            },
            FieldInfoData {
                name: "SliceTop",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_top),
            },
            FieldInfoData {
                name: "SliceRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_right),
            },
            FieldInfoData {
                name: "SliceBottom",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, slice_bottom),
            },
            FieldInfoData {
                name: "PaddingLeft",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_left),
            },
            FieldInfoData {
                name: "PaddingTop",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_top),
            },
            FieldInfoData {
                name: "PaddingRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_right),
            },
            FieldInfoData {
                name: "PaddingBottom",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, padding_bottom),
            },
            FieldInfoData {
                name: "FillCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FBUISlicedTextureElementEntityData, fill_center),
            },
        ],
    }),
    array_type: Some(FBUISLICEDTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUISlicedTextureElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUISLICEDTEXTUREELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUISLICEDTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUISlicedTextureElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUISlicedTextureElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIMovieElementEntityData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub movie: Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>,
    pub r#loop: bool,
    pub auto_start: bool,
    pub preload: bool,
    pub fullscreen: bool,
    pub volume: f32,
}

pub trait FBUIMovieElementEntityDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn movie(&self) -> &Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>;
    fn movie_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>>;
    fn r#loop(&self) -> &bool;
    fn r#loop_mut(&mut self) -> &mut bool;
    fn auto_start(&self) -> &bool;
    fn auto_start_mut(&mut self) -> &mut bool;
    fn preload(&self) -> &bool;
    fn preload_mut(&mut self) -> &mut bool;
    fn fullscreen(&self) -> &bool;
    fn fullscreen_mut(&mut self) -> &mut bool;
    fn volume(&self) -> &f32;
    fn volume_mut(&mut self) -> &mut f32;
}

impl FBUIMovieElementEntityDataTrait for FBUIMovieElementEntityData {
    fn movie(&self) -> &Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>> {
        &self.movie
    }
    fn movie_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::movie_base::MovieTextureBaseAssetTrait>>> {
        &mut self.movie
    }
    fn r#loop(&self) -> &bool {
        &self.r#loop
    }
    fn r#loop_mut(&mut self) -> &mut bool {
        &mut self.r#loop
    }
    fn auto_start(&self) -> &bool {
        &self.auto_start
    }
    fn auto_start_mut(&mut self) -> &mut bool {
        &mut self.auto_start
    }
    fn preload(&self) -> &bool {
        &self.preload
    }
    fn preload_mut(&mut self) -> &mut bool {
        &mut self.preload
    }
    fn fullscreen(&self) -> &bool {
        &self.fullscreen
    }
    fn fullscreen_mut(&mut self) -> &mut bool {
        &mut self.fullscreen
    }
    fn volume(&self) -> &f32 {
        &self.volume
    }
    fn volume_mut(&mut self) -> &mut f32 {
        &mut self.volume
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for FBUIMovieElementEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for FBUIMovieElementEntityData {
}

impl super::entity::GameObjectDataTrait for FBUIMovieElementEntityData {
}

impl super::core::DataBusPeerTrait for FBUIMovieElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUIMovieElementEntityData {
}

impl super::core::DataContainerTrait for FBUIMovieElementEntityData {
}

pub static FBUIMOVIEELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIMovieElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Movie",
                flags: MemberInfoFlags::new(0),
                field_type: "MovieTextureBaseAsset",
                rust_offset: offset_of!(FBUIMovieElementEntityData, movie),
            },
            FieldInfoData {
                name: "Loop",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FBUIMovieElementEntityData, r#loop),
            },
            FieldInfoData {
                name: "AutoStart",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FBUIMovieElementEntityData, auto_start),
            },
            FieldInfoData {
                name: "Preload",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FBUIMovieElementEntityData, preload),
            },
            FieldInfoData {
                name: "Fullscreen",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FBUIMovieElementEntityData, fullscreen),
            },
            FieldInfoData {
                name: "Volume",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FBUIMovieElementEntityData, volume),
            },
        ],
    }),
    array_type: Some(FBUIMOVIEELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIMovieElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUIMOVIEELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUIMOVIEELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIMovieElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIMovieElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIDynamicTextureElementEntityData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub uv_rect: super::core::Vec4,
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub texture: Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>,
}

pub trait FBUIDynamicTextureElementEntityDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn address_u(&self) -> &super::render_base::TextureAddress;
    fn address_u_mut(&mut self) -> &mut super::render_base::TextureAddress;
    fn address_v(&self) -> &super::render_base::TextureAddress;
    fn address_v_mut(&mut self) -> &mut super::render_base::TextureAddress;
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>>;
}

impl FBUIDynamicTextureElementEntityDataTrait for FBUIDynamicTextureElementEntityData {
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
    fn address_u(&self) -> &super::render_base::TextureAddress {
        &self.address_u
    }
    fn address_u_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address_u
    }
    fn address_v(&self) -> &super::render_base::TextureAddress {
        &self.address_v
    }
    fn address_v_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address_v
    }
    fn texture(&self) -> &Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::render_base::TextureBaseAssetTrait>>> {
        &mut self.texture
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for FBUIDynamicTextureElementEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for FBUIDynamicTextureElementEntityData {
}

impl super::entity::GameObjectDataTrait for FBUIDynamicTextureElementEntityData {
}

impl super::core::DataBusPeerTrait for FBUIDynamicTextureElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUIDynamicTextureElementEntityData {
}

impl super::core::DataContainerTrait for FBUIDynamicTextureElementEntityData {
}

pub static FBUIDYNAMICTEXTUREELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIDynamicTextureElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, uv_rect),
            },
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, address_v),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureBaseAsset",
                rust_offset: offset_of!(FBUIDynamicTextureElementEntityData, texture),
            },
        ],
    }),
    array_type: Some(FBUIDYNAMICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIDynamicTextureElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUIDYNAMICTEXTUREELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUIDYNAMICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIDynamicTextureElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIDynamicTextureElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUIStaticTextureElementEntityData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub uv_rect: super::core::Vec4,
    pub address_u: super::render_base::TextureAddress,
    pub address_v: super::render_base::TextureAddress,
    pub texture: super::game_shared_u_i::UIAutoMappedTexture,
}

pub trait FBUIStaticTextureElementEntityDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn address_u(&self) -> &super::render_base::TextureAddress;
    fn address_u_mut(&mut self) -> &mut super::render_base::TextureAddress;
    fn address_v(&self) -> &super::render_base::TextureAddress;
    fn address_v_mut(&mut self) -> &mut super::render_base::TextureAddress;
    fn texture(&self) -> &super::game_shared_u_i::UIAutoMappedTexture;
    fn texture_mut(&mut self) -> &mut super::game_shared_u_i::UIAutoMappedTexture;
}

impl FBUIStaticTextureElementEntityDataTrait for FBUIStaticTextureElementEntityData {
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
    fn address_u(&self) -> &super::render_base::TextureAddress {
        &self.address_u
    }
    fn address_u_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address_u
    }
    fn address_v(&self) -> &super::render_base::TextureAddress {
        &self.address_v
    }
    fn address_v_mut(&mut self) -> &mut super::render_base::TextureAddress {
        &mut self.address_v
    }
    fn texture(&self) -> &super::game_shared_u_i::UIAutoMappedTexture {
        &self.texture
    }
    fn texture_mut(&mut self) -> &mut super::game_shared_u_i::UIAutoMappedTexture {
        &mut self.texture
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for FBUIStaticTextureElementEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for FBUIStaticTextureElementEntityData {
}

impl super::entity::GameObjectDataTrait for FBUIStaticTextureElementEntityData {
}

impl super::core::DataBusPeerTrait for FBUIStaticTextureElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUIStaticTextureElementEntityData {
}

impl super::core::DataContainerTrait for FBUIStaticTextureElementEntityData {
}

pub static FBUISTATICTEXTUREELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUIStaticTextureElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, uv_rect),
            },
            FieldInfoData {
                name: "AddressU",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, address_u),
            },
            FieldInfoData {
                name: "AddressV",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureAddress",
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, address_v),
            },
            FieldInfoData {
                name: "Texture",
                flags: MemberInfoFlags::new(0),
                field_type: "UIAutoMappedTexture",
                rust_offset: offset_of!(FBUIStaticTextureElementEntityData, texture),
            },
        ],
    }),
    array_type: Some(FBUISTATICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUIStaticTextureElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUISTATICTEXTUREELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUISTATICTEXTUREELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUIStaticTextureElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUIStaticTextureElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FBUILabelElementEntityData {
    pub _glacier_base: super::game_shared_u_i::UIElementEntityData,
    pub text: String,
    pub font_style: Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontStyleTrait>>>,
    pub word_wrap: bool,
    pub font_effect: Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontEffectTrait>>>,
    pub horizontal_alignment: super::game_base::UIElementAlignment,
    pub vertical_alignment: super::game_base::UIElementAlignment,
}

pub trait FBUILabelElementEntityDataTrait: super::game_shared_u_i::UIElementEntityDataTrait {
    fn text(&self) -> &String;
    fn text_mut(&mut self) -> &mut String;
    fn font_style(&self) -> &Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontStyleTrait>>>;
    fn font_style_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontStyleTrait>>>;
    fn word_wrap(&self) -> &bool;
    fn word_wrap_mut(&mut self) -> &mut bool;
    fn font_effect(&self) -> &Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontEffectTrait>>>;
    fn font_effect_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontEffectTrait>>>;
    fn horizontal_alignment(&self) -> &super::game_base::UIElementAlignment;
    fn horizontal_alignment_mut(&mut self) -> &mut super::game_base::UIElementAlignment;
    fn vertical_alignment(&self) -> &super::game_base::UIElementAlignment;
    fn vertical_alignment_mut(&mut self) -> &mut super::game_base::UIElementAlignment;
}

impl FBUILabelElementEntityDataTrait for FBUILabelElementEntityData {
    fn text(&self) -> &String {
        &self.text
    }
    fn text_mut(&mut self) -> &mut String {
        &mut self.text
    }
    fn font_style(&self) -> &Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontStyleTrait>>> {
        &self.font_style
    }
    fn font_style_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontStyleTrait>>> {
        &mut self.font_style
    }
    fn word_wrap(&self) -> &bool {
        &self.word_wrap
    }
    fn word_wrap_mut(&mut self) -> &mut bool {
        &mut self.word_wrap
    }
    fn font_effect(&self) -> &Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontEffectTrait>>> {
        &self.font_effect
    }
    fn font_effect_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::game_shared_u_i::UIElementFontEffectTrait>>> {
        &mut self.font_effect
    }
    fn horizontal_alignment(&self) -> &super::game_base::UIElementAlignment {
        &self.horizontal_alignment
    }
    fn horizontal_alignment_mut(&mut self) -> &mut super::game_base::UIElementAlignment {
        &mut self.horizontal_alignment
    }
    fn vertical_alignment(&self) -> &super::game_base::UIElementAlignment {
        &self.vertical_alignment
    }
    fn vertical_alignment_mut(&mut self) -> &mut super::game_base::UIElementAlignment {
        &mut self.vertical_alignment
    }
}

impl super::game_shared_u_i::UIElementEntityDataTrait for FBUILabelElementEntityData {
    fn instance_name(&self) -> &String {
        self._glacier_base.instance_name()
    }
    fn instance_name_mut(&mut self) -> &mut String {
        self._glacier_base.instance_name_mut()
    }
    fn instance_name_hash(&self) -> &u32 {
        self._glacier_base.instance_name_hash()
    }
    fn instance_name_hash_mut(&mut self) -> &mut u32 {
        self._glacier_base.instance_name_hash_mut()
    }
    fn transform_pivot(&self) -> &super::core::Vec3 {
        self._glacier_base.transform_pivot()
    }
    fn transform_pivot_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.transform_pivot_mut()
    }
    fn size(&self) -> &super::core::Vec2 {
        self._glacier_base.size()
    }
    fn size_mut(&mut self) -> &mut super::core::Vec2 {
        self._glacier_base.size_mut()
    }
    fn layout_mode(&self) -> &super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode()
    }
    fn layout_mode_mut(&mut self) -> &mut super::game_shared_u_i::UILayoutMode {
        self._glacier_base.layout_mode_mut()
    }
    fn offset(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset()
    }
    fn offset_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.offset_mut()
    }
    fn anchor(&self) -> &super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor()
    }
    fn anchor_mut(&mut self) -> &mut super::game_shared_u_i::UIElementAnchor {
        self._glacier_base.anchor_mut()
    }
    fn position(&self) -> &super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position()
    }
    fn position_mut(&mut self) -> &mut super::game_shared_u_i::UIElementOffset {
        self._glacier_base.position_mut()
    }
    fn expansion(&self) -> &super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion()
    }
    fn expansion_mut(&mut self) -> &mut super::game_shared_u_i::UIElementRectExpansion {
        self._glacier_base.expansion_mut()
    }
    fn visible(&self) -> &bool {
        self._glacier_base.visible()
    }
    fn visible_mut(&mut self) -> &mut bool {
        self._glacier_base.visible_mut()
    }
    fn color(&self) -> &super::core::Vec3 {
        self._glacier_base.color()
    }
    fn color_mut(&mut self) -> &mut super::core::Vec3 {
        self._glacier_base.color_mut()
    }
    fn alpha(&self) -> &f32 {
        self._glacier_base.alpha()
    }
    fn alpha_mut(&mut self) -> &mut f32 {
        self._glacier_base.alpha_mut()
    }
    fn transform(&self) -> &super::core::LinearTransform {
        self._glacier_base.transform()
    }
    fn transform_mut(&mut self) -> &mut super::core::LinearTransform {
        self._glacier_base.transform_mut()
    }
}

impl super::entity::EntityDataTrait for FBUILabelElementEntityData {
}

impl super::entity::GameObjectDataTrait for FBUILabelElementEntityData {
}

impl super::core::DataBusPeerTrait for FBUILabelElementEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for FBUILabelElementEntityData {
}

impl super::core::DataContainerTrait for FBUILabelElementEntityData {
}

pub static FBUILABELELEMENTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::game_shared_u_i::UIELEMENTENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FBUILabelElementEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Text",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(FBUILabelElementEntityData, text),
            },
            FieldInfoData {
                name: "FontStyle",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontStyle",
                rust_offset: offset_of!(FBUILabelElementEntityData, font_style),
            },
            FieldInfoData {
                name: "WordWrap",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(FBUILabelElementEntityData, word_wrap),
            },
            FieldInfoData {
                name: "FontEffect",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementFontEffect",
                rust_offset: offset_of!(FBUILabelElementEntityData, font_effect),
            },
            FieldInfoData {
                name: "HorizontalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAlignment",
                rust_offset: offset_of!(FBUILabelElementEntityData, horizontal_alignment),
            },
            FieldInfoData {
                name: "VerticalAlignment",
                flags: MemberInfoFlags::new(0),
                field_type: "UIElementAlignment",
                rust_offset: offset_of!(FBUILabelElementEntityData, vertical_alignment),
            },
        ],
    }),
    array_type: Some(FBUILABELELEMENTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for FBUILabelElementEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        FBUILABELELEMENTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FBUILABELELEMENTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FBUILabelElementEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FBUILabelElementEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawSlicedTextureCommandDynamicState {
    pub rect: super::core::Vec4,
    pub field_flag_changed0: u8,
}

pub trait UIDrawSlicedTextureCommandDynamicStateTrait: TypeObject {
    fn rect(&self) -> &super::core::Vec4;
    fn rect_mut(&mut self) -> &mut super::core::Vec4;
    fn field_flag_changed0(&self) -> &u8;
    fn field_flag_changed0_mut(&mut self) -> &mut u8;
}

impl UIDrawSlicedTextureCommandDynamicStateTrait for UIDrawSlicedTextureCommandDynamicState {
    fn rect(&self) -> &super::core::Vec4 {
        &self.rect
    }
    fn rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.rect
    }
    fn field_flag_changed0(&self) -> &u8 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u8 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandDynamicState",
    flags: MemberInfoFlags::new(36937),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawSlicedTextureCommandDynamicState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Rect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandDynamicState, rect),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint8",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandDynamicState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawSlicedTextureCommandDynamicState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static UIDRAWSLICEDTEXTURECOMMANDDYNAMICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandDynamicState-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("UIDrawSlicedTextureCommandDynamicState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct UIDrawSlicedTextureCommandStaticState {
    pub slice_left: i32,
    pub slice_top: i32,
    pub slice_right: i32,
    pub slice_bottom: i32,
    pub padding_left: i32,
    pub padding_top: i32,
    pub padding_right: i32,
    pub padding_bottom: i32,
    pub fill_center: bool,
    pub uv_rect: super::core::Vec4,
    pub texture_handle: super::render_base::TextureResourceHandle,
    pub field_flag_changed0: u16,
}

pub trait UIDrawSlicedTextureCommandStaticStateTrait: TypeObject {
    fn slice_left(&self) -> &i32;
    fn slice_left_mut(&mut self) -> &mut i32;
    fn slice_top(&self) -> &i32;
    fn slice_top_mut(&mut self) -> &mut i32;
    fn slice_right(&self) -> &i32;
    fn slice_right_mut(&mut self) -> &mut i32;
    fn slice_bottom(&self) -> &i32;
    fn slice_bottom_mut(&mut self) -> &mut i32;
    fn padding_left(&self) -> &i32;
    fn padding_left_mut(&mut self) -> &mut i32;
    fn padding_top(&self) -> &i32;
    fn padding_top_mut(&mut self) -> &mut i32;
    fn padding_right(&self) -> &i32;
    fn padding_right_mut(&mut self) -> &mut i32;
    fn padding_bottom(&self) -> &i32;
    fn padding_bottom_mut(&mut self) -> &mut i32;
    fn fill_center(&self) -> &bool;
    fn fill_center_mut(&mut self) -> &mut bool;
    fn uv_rect(&self) -> &super::core::Vec4;
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4;
    fn texture_handle(&self) -> &super::render_base::TextureResourceHandle;
    fn texture_handle_mut(&mut self) -> &mut super::render_base::TextureResourceHandle;
    fn field_flag_changed0(&self) -> &u16;
    fn field_flag_changed0_mut(&mut self) -> &mut u16;
}

impl UIDrawSlicedTextureCommandStaticStateTrait for UIDrawSlicedTextureCommandStaticState {
    fn slice_left(&self) -> &i32 {
        &self.slice_left
    }
    fn slice_left_mut(&mut self) -> &mut i32 {
        &mut self.slice_left
    }
    fn slice_top(&self) -> &i32 {
        &self.slice_top
    }
    fn slice_top_mut(&mut self) -> &mut i32 {
        &mut self.slice_top
    }
    fn slice_right(&self) -> &i32 {
        &self.slice_right
    }
    fn slice_right_mut(&mut self) -> &mut i32 {
        &mut self.slice_right
    }
    fn slice_bottom(&self) -> &i32 {
        &self.slice_bottom
    }
    fn slice_bottom_mut(&mut self) -> &mut i32 {
        &mut self.slice_bottom
    }
    fn padding_left(&self) -> &i32 {
        &self.padding_left
    }
    fn padding_left_mut(&mut self) -> &mut i32 {
        &mut self.padding_left
    }
    fn padding_top(&self) -> &i32 {
        &self.padding_top
    }
    fn padding_top_mut(&mut self) -> &mut i32 {
        &mut self.padding_top
    }
    fn padding_right(&self) -> &i32 {
        &self.padding_right
    }
    fn padding_right_mut(&mut self) -> &mut i32 {
        &mut self.padding_right
    }
    fn padding_bottom(&self) -> &i32 {
        &self.padding_bottom
    }
    fn padding_bottom_mut(&mut self) -> &mut i32 {
        &mut self.padding_bottom
    }
    fn fill_center(&self) -> &bool {
        &self.fill_center
    }
    fn fill_center_mut(&mut self) -> &mut bool {
        &mut self.fill_center
    }
    fn uv_rect(&self) -> &super::core::Vec4 {
        &self.uv_rect
    }
    fn uv_rect_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.uv_rect
    }
    fn texture_handle(&self) -> &super::render_base::TextureResourceHandle {
        &self.texture_handle
    }
    fn texture_handle_mut(&mut self) -> &mut super::render_base::TextureResourceHandle {
        &mut self.texture_handle
    }
    fn field_flag_changed0(&self) -> &u16 {
        &self.field_flag_changed0
    }
    fn field_flag_changed0_mut(&mut self) -> &mut u16 {
        &mut self.field_flag_changed0
    }
}

pub static UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandStaticState",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<UIDrawSlicedTextureCommandStaticState as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "SliceLeft",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_left),
            },
            FieldInfoData {
                name: "SliceTop",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_top),
            },
            FieldInfoData {
                name: "SliceRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_right),
            },
            FieldInfoData {
                name: "SliceBottom",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, slice_bottom),
            },
            FieldInfoData {
                name: "PaddingLeft",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_left),
            },
            FieldInfoData {
                name: "PaddingTop",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_top),
            },
            FieldInfoData {
                name: "PaddingRight",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_right),
            },
            FieldInfoData {
                name: "PaddingBottom",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, padding_bottom),
            },
            FieldInfoData {
                name: "FillCenter",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, fill_center),
            },
            FieldInfoData {
                name: "UvRect",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, uv_rect),
            },
            FieldInfoData {
                name: "TextureHandle",
                flags: MemberInfoFlags::new(0),
                field_type: "TextureResourceHandle",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, texture_handle),
            },
            FieldInfoData {
                name: "FieldFlagChanged0",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint16",
                rust_offset: offset_of!(UIDrawSlicedTextureCommandStaticState, field_flag_changed0),
            },
        ],
    }),
    array_type: Some(UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for UIDrawSlicedTextureCommandStaticState {
    fn type_info(&self) -> &'static TypeInfo {
        UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static UIDRAWSLICEDTEXTURECOMMANDSTATICSTATE_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "UIDrawSlicedTextureCommandStaticState-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("UIDrawSlicedTextureCommandStaticState"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct DynamicCastEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub in_data: Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>,
    pub dynamic_output_data_type: u32,
}

pub trait DynamicCastEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn in_data(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>;
    fn in_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::DataContainerTrait>>>;
    fn dynamic_output_data_type(&self) -> &u32;
    fn dynamic_output_data_type_mut(&mut self) -> &mut u32;
}

impl DynamicCastEntityDataTrait for DynamicCastEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn in_data(&self) -> &Option<Arc<Mutex<dyn super::core::DataContainerTrait>>> {
        &self.in_data
    }
    fn in_data_mut(&mut self) -> &mut Option<Arc<Mutex<dyn super::core::DataContainerTrait>>> {
        &mut self.in_data
    }
    fn dynamic_output_data_type(&self) -> &u32 {
        &self.dynamic_output_data_type
    }
    fn dynamic_output_data_type_mut(&mut self) -> &mut u32 {
        &mut self.dynamic_output_data_type
    }
}

impl super::entity::EntityDataTrait for DynamicCastEntityData {
}

impl super::entity::GameObjectDataTrait for DynamicCastEntityData {
}

impl super::core::DataBusPeerTrait for DynamicCastEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for DynamicCastEntityData {
}

impl super::core::DataContainerTrait for DynamicCastEntityData {
}

pub static DYNAMICCASTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<DynamicCastEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(DynamicCastEntityData, realm),
            },
            FieldInfoData {
                name: "InData",
                flags: MemberInfoFlags::new(0),
                field_type: "DataContainer",
                rust_offset: offset_of!(DynamicCastEntityData, in_data),
            },
            FieldInfoData {
                name: "DynamicOutputDataType",
                flags: MemberInfoFlags::new(0),
                field_type: "Uint32",
                rust_offset: offset_of!(DynamicCastEntityData, dynamic_output_data_type),
            },
        ],
    }),
    array_type: Some(DYNAMICCASTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for DynamicCastEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        DYNAMICCASTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static DYNAMICCASTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "DynamicCastEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("DynamicCastEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConfigEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub asset_to_output: Option<Arc<Mutex<dyn ConfigEntityAssetDataTrait>>>,
    pub needed_to_create_ouputs: f32,
}

pub trait ConfigEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn asset_to_output(&self) -> &Option<Arc<Mutex<dyn ConfigEntityAssetDataTrait>>>;
    fn asset_to_output_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ConfigEntityAssetDataTrait>>>;
    fn needed_to_create_ouputs(&self) -> &f32;
    fn needed_to_create_ouputs_mut(&mut self) -> &mut f32;
}

impl ConfigEntityDataTrait for ConfigEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn asset_to_output(&self) -> &Option<Arc<Mutex<dyn ConfigEntityAssetDataTrait>>> {
        &self.asset_to_output
    }
    fn asset_to_output_mut(&mut self) -> &mut Option<Arc<Mutex<dyn ConfigEntityAssetDataTrait>>> {
        &mut self.asset_to_output
    }
    fn needed_to_create_ouputs(&self) -> &f32 {
        &self.needed_to_create_ouputs
    }
    fn needed_to_create_ouputs_mut(&mut self) -> &mut f32 {
        &mut self.needed_to_create_ouputs
    }
}

impl super::entity::EntityDataTrait for ConfigEntityData {
}

impl super::entity::GameObjectDataTrait for ConfigEntityData {
}

impl super::core::DataBusPeerTrait for ConfigEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConfigEntityData {
}

impl super::core::DataContainerTrait for ConfigEntityData {
}

pub static CONFIGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConfigEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ConfigEntityData, realm),
            },
            FieldInfoData {
                name: "AssetToOutput",
                flags: MemberInfoFlags::new(0),
                field_type: "ConfigEntityAssetData",
                rust_offset: offset_of!(ConfigEntityData, asset_to_output),
            },
            FieldInfoData {
                name: "neededToCreateOuputs",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConfigEntityData, needed_to_create_ouputs),
            },
        ],
    }),
    array_type: Some(CONFIGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONFIGENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONFIGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConfigEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StringListAsset {
    pub _glacier_base: ConfigListAsset,
    pub string_types: Vec<StringData>,
}

pub trait StringListAssetTrait: ConfigListAssetTrait {
    fn string_types(&self) -> &Vec<StringData>;
    fn string_types_mut(&mut self) -> &mut Vec<StringData>;
}

impl StringListAssetTrait for StringListAsset {
    fn string_types(&self) -> &Vec<StringData> {
        &self.string_types
    }
    fn string_types_mut(&mut self) -> &mut Vec<StringData> {
        &mut self.string_types
    }
}

impl ConfigListAssetTrait for StringListAsset {
}

impl super::core::AssetTrait for StringListAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for StringListAsset {
}

pub static STRINGLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StringListAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "StringTypes",
                flags: MemberInfoFlags::new(144),
                field_type: "StringData-Array",
                rust_offset: offset_of!(StringListAsset, string_types),
            },
        ],
    }),
    array_type: Some(STRINGLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringListAsset {
    fn type_info(&self) -> &'static TypeInfo {
        STRINGLISTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static STRINGLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringListAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntListAsset {
    pub _glacier_base: ConfigListAsset,
    pub int_types: Vec<IntData>,
}

pub trait IntListAssetTrait: ConfigListAssetTrait {
    fn int_types(&self) -> &Vec<IntData>;
    fn int_types_mut(&mut self) -> &mut Vec<IntData>;
}

impl IntListAssetTrait for IntListAsset {
    fn int_types(&self) -> &Vec<IntData> {
        &self.int_types
    }
    fn int_types_mut(&mut self) -> &mut Vec<IntData> {
        &mut self.int_types
    }
}

impl ConfigListAssetTrait for IntListAsset {
}

impl super::core::AssetTrait for IntListAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for IntListAsset {
}

pub static INTLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntListAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "IntTypes",
                flags: MemberInfoFlags::new(144),
                field_type: "IntData-Array",
                rust_offset: offset_of!(IntListAsset, int_types),
            },
        ],
    }),
    array_type: Some(INTLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntListAsset {
    fn type_info(&self) -> &'static TypeInfo {
        INTLISTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static INTLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntListAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatListAsset {
    pub _glacier_base: ConfigListAsset,
    pub float_types: Vec<FloatData>,
}

pub trait FloatListAssetTrait: ConfigListAssetTrait {
    fn float_types(&self) -> &Vec<FloatData>;
    fn float_types_mut(&mut self) -> &mut Vec<FloatData>;
}

impl FloatListAssetTrait for FloatListAsset {
    fn float_types(&self) -> &Vec<FloatData> {
        &self.float_types
    }
    fn float_types_mut(&mut self) -> &mut Vec<FloatData> {
        &mut self.float_types
    }
}

impl ConfigListAssetTrait for FloatListAsset {
}

impl super::core::AssetTrait for FloatListAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for FloatListAsset {
}

pub static FLOATLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatListAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "FloatTypes",
                flags: MemberInfoFlags::new(144),
                field_type: "FloatData-Array",
                rust_offset: offset_of!(FloatListAsset, float_types),
            },
        ],
    }),
    array_type: Some(FLOATLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatListAsset {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATLISTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static FLOATLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FloatListAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorListAsset {
    pub _glacier_base: ConfigListAsset,
    pub color_types: Vec<ColorData>,
}

pub trait ColorListAssetTrait: ConfigListAssetTrait {
    fn color_types(&self) -> &Vec<ColorData>;
    fn color_types_mut(&mut self) -> &mut Vec<ColorData>;
}

impl ColorListAssetTrait for ColorListAsset {
    fn color_types(&self) -> &Vec<ColorData> {
        &self.color_types
    }
    fn color_types_mut(&mut self) -> &mut Vec<ColorData> {
        &mut self.color_types
    }
}

impl ConfigListAssetTrait for ColorListAsset {
}

impl super::core::AssetTrait for ColorListAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ColorListAsset {
}

pub static COLORLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorListAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ColorTypes",
                flags: MemberInfoFlags::new(144),
                field_type: "ColorData-Array",
                rust_offset: offset_of!(ColorListAsset, color_types),
            },
        ],
    }),
    array_type: Some(COLORLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ColorListAsset {
    fn type_info(&self) -> &'static TypeInfo {
        COLORLISTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static COLORLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ColorListAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3ListAsset {
    pub _glacier_base: ConfigListAsset,
    pub vec3_types: Vec<Vec3Data>,
}

pub trait Vec3ListAssetTrait: ConfigListAssetTrait {
    fn vec3_types(&self) -> &Vec<Vec3Data>;
    fn vec3_types_mut(&mut self) -> &mut Vec<Vec3Data>;
}

impl Vec3ListAssetTrait for Vec3ListAsset {
    fn vec3_types(&self) -> &Vec<Vec3Data> {
        &self.vec3_types
    }
    fn vec3_types_mut(&mut self) -> &mut Vec<Vec3Data> {
        &mut self.vec3_types
    }
}

impl ConfigListAssetTrait for Vec3ListAsset {
}

impl super::core::AssetTrait for Vec3ListAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for Vec3ListAsset {
}

pub static VEC3LISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONFIGLISTASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3ListAsset as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Vec3Types",
                flags: MemberInfoFlags::new(144),
                field_type: "Vec3Data-Array",
                rust_offset: offset_of!(Vec3ListAsset, vec3_types),
            },
        ],
    }),
    array_type: Some(VEC3LISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for Vec3ListAsset {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3LISTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static VEC3LISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3ListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec3ListAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConfigListAsset {
    pub _glacier_base: super::core::Asset,
}

pub trait ConfigListAssetTrait: super::core::AssetTrait {
}

impl ConfigListAssetTrait for ConfigListAsset {
}

impl super::core::AssetTrait for ConfigListAsset {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ConfigListAsset {
}

pub static CONFIGLISTASSET_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigListAsset",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::ASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConfigListAsset as Default>::default())),
        },
        fields: &[
        ],
    }),
    array_type: Some(CONFIGLISTASSET_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigListAsset {
    fn type_info(&self) -> &'static TypeInfo {
        CONFIGLISTASSET_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONFIGLISTASSET_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigListAsset-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConfigListAsset"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConfigEntityAssetData {
    pub _glacier_base: super::core::DataContainerPolicyAsset,
    pub data_lists: Vec<Option<Arc<Mutex<dyn ConfigListAssetTrait>>>>,
}

pub trait ConfigEntityAssetDataTrait: super::core::DataContainerPolicyAssetTrait {
    fn data_lists(&self) -> &Vec<Option<Arc<Mutex<dyn ConfigListAssetTrait>>>>;
    fn data_lists_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ConfigListAssetTrait>>>>;
}

impl ConfigEntityAssetDataTrait for ConfigEntityAssetData {
    fn data_lists(&self) -> &Vec<Option<Arc<Mutex<dyn ConfigListAssetTrait>>>> {
        &self.data_lists
    }
    fn data_lists_mut(&mut self) -> &mut Vec<Option<Arc<Mutex<dyn ConfigListAssetTrait>>>> {
        &mut self.data_lists
    }
}

impl super::core::DataContainerPolicyAssetTrait for ConfigEntityAssetData {
}

impl super::core::AssetTrait for ConfigEntityAssetData {
    fn name(&self) -> &String {
        self._glacier_base.name()
    }
    fn name_mut(&mut self) -> &mut String {
        self._glacier_base.name_mut()
    }
}

impl super::core::DataContainerTrait for ConfigEntityAssetData {
}

pub static CONFIGENTITYASSETDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityAssetData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::core::DATACONTAINERPOLICYASSET_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConfigEntityAssetData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "DataLists",
                flags: MemberInfoFlags::new(144),
                field_type: "ConfigListAsset-Array",
                rust_offset: offset_of!(ConfigEntityAssetData, data_lists),
            },
        ],
    }),
    array_type: Some(CONFIGENTITYASSETDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConfigEntityAssetData {
    fn type_info(&self) -> &'static TypeInfo {
        CONFIGENTITYASSETDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONFIGENTITYASSETDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConfigEntityAssetData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConfigEntityAssetData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct StringData {
    pub name: String,
    pub string_value: String,
}

pub trait StringDataTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn string_value(&self) -> &String;
    fn string_value_mut(&mut self) -> &mut String;
}

impl StringDataTrait for StringData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn string_value(&self) -> &String {
        &self.string_value
    }
    fn string_value_mut(&mut self) -> &mut String {
        &mut self.string_value
    }
}

pub static STRINGDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<StringData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StringData, name),
            },
            FieldInfoData {
                name: "StringValue",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(StringData, string_value),
            },
        ],
    }),
    array_type: Some(STRINGDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for StringData {
    fn type_info(&self) -> &'static TypeInfo {
        STRINGDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static STRINGDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "StringData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("StringData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct IntData {
    pub name: String,
    pub int_value: i32,
}

pub trait IntDataTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn int_value(&self) -> &i32;
    fn int_value_mut(&mut self) -> &mut i32;
}

impl IntDataTrait for IntData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn int_value(&self) -> &i32 {
        &self.int_value
    }
    fn int_value_mut(&mut self) -> &mut i32 {
        &mut self.int_value
    }
}

pub static INTDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<IntData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(IntData, name),
            },
            FieldInfoData {
                name: "IntValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(IntData, int_value),
            },
        ],
    }),
    array_type: Some(INTDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for IntData {
    fn type_info(&self) -> &'static TypeInfo {
        INTDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static INTDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "IntData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("IntData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct FloatData {
    pub name: String,
    pub float_value: f32,
}

pub trait FloatDataTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn float_value(&self) -> &f32;
    fn float_value_mut(&mut self) -> &mut f32;
}

impl FloatDataTrait for FloatData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn float_value(&self) -> &f32 {
        &self.float_value
    }
    fn float_value_mut(&mut self) -> &mut f32 {
        &mut self.float_value
    }
}

pub static FLOATDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<FloatData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(FloatData, name),
            },
            FieldInfoData {
                name: "FloatValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(FloatData, float_value),
            },
        ],
    }),
    array_type: Some(FLOATDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for FloatData {
    fn type_info(&self) -> &'static TypeInfo {
        FLOATDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static FLOATDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "FloatData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("FloatData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ColorData {
    pub name: String,
    pub color_value: super::core::Vec3,
}

pub trait ColorDataTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn color_value(&self) -> &super::core::Vec3;
    fn color_value_mut(&mut self) -> &mut super::core::Vec3;
}

impl ColorDataTrait for ColorData {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn color_value(&self) -> &super::core::Vec3 {
        &self.color_value
    }
    fn color_value_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.color_value
    }
}

pub static COLORDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorData",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ColorData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ColorData, name),
            },
            FieldInfoData {
                name: "ColorValue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ColorData, color_value),
            },
        ],
    }),
    array_type: Some(COLORDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ColorData {
    fn type_info(&self) -> &'static TypeInfo {
        COLORDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static COLORDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ColorData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ColorData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct Vec3Data {
    pub name: String,
    pub vec3_value: super::core::Vec3,
}

pub trait Vec3DataTrait: TypeObject {
    fn name(&self) -> &String;
    fn name_mut(&mut self) -> &mut String;
    fn vec3_value(&self) -> &super::core::Vec3;
    fn vec3_value_mut(&mut self) -> &mut super::core::Vec3;
}

impl Vec3DataTrait for Vec3Data {
    fn name(&self) -> &String {
        &self.name
    }
    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    fn vec3_value(&self) -> &super::core::Vec3 {
        &self.vec3_value
    }
    fn vec3_value_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.vec3_value
    }
}

pub static VEC3DATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Data",
    flags: MemberInfoFlags::new(73),
    module: "UIIncubatorShared",
    data: TypeInfoData::ValueType(ValueTypeInfoData {
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<Vec3Data as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Name",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(Vec3Data, name),
            },
            FieldInfoData {
                name: "Vec3Value",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(Vec3Data, vec3_value),
            },
        ],
    }),
    array_type: Some(VEC3DATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for Vec3Data {
    fn type_info(&self) -> &'static TypeInfo {
        VEC3DATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        None
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        None
    }
}


pub static VEC3DATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "Vec3Data-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("Vec3Data"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalStringEntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: String,
    pub value_if_false: String,
}

pub trait ConditionalStringEntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &String;
    fn value_if_true_mut(&mut self) -> &mut String;
    fn value_if_false(&self) -> &String;
    fn value_if_false_mut(&mut self) -> &mut String;
}

impl ConditionalStringEntityDataTrait for ConditionalStringEntityData {
    fn value_if_true(&self) -> &String {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut String {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &String {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut String {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalStringEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalStringEntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalStringEntityData {
}

impl super::core::DataBusPeerTrait for ConditionalStringEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalStringEntityData {
}

impl super::core::DataContainerTrait for ConditionalStringEntityData {
}

pub static CONDITIONALSTRINGENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalStringEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ConditionalStringEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "CString",
                rust_offset: offset_of!(ConditionalStringEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALSTRINGENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalStringEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALSTRINGENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALSTRINGENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStringEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalStringEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalTransformEntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: super::core::LinearTransform,
    pub value_if_false: super::core::LinearTransform,
}

pub trait ConditionalTransformEntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &super::core::LinearTransform;
    fn value_if_true_mut(&mut self) -> &mut super::core::LinearTransform;
    fn value_if_false(&self) -> &super::core::LinearTransform;
    fn value_if_false_mut(&mut self) -> &mut super::core::LinearTransform;
}

impl ConditionalTransformEntityDataTrait for ConditionalTransformEntityData {
    fn value_if_true(&self) -> &super::core::LinearTransform {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &super::core::LinearTransform {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut super::core::LinearTransform {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalTransformEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalTransformEntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalTransformEntityData {
}

impl super::core::DataBusPeerTrait for ConditionalTransformEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalTransformEntityData {
}

impl super::core::DataContainerTrait for ConditionalTransformEntityData {
}

pub static CONDITIONALTRANSFORMENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalTransformEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ConditionalTransformEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "LinearTransform",
                rust_offset: offset_of!(ConditionalTransformEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALTRANSFORMENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConditionalTransformEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALTRANSFORMENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALTRANSFORMENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalTransformEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalTransformEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalVec4EntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: super::core::Vec4,
    pub value_if_false: super::core::Vec4,
}

pub trait ConditionalVec4EntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &super::core::Vec4;
    fn value_if_true_mut(&mut self) -> &mut super::core::Vec4;
    fn value_if_false(&self) -> &super::core::Vec4;
    fn value_if_false_mut(&mut self) -> &mut super::core::Vec4;
}

impl ConditionalVec4EntityDataTrait for ConditionalVec4EntityData {
    fn value_if_true(&self) -> &super::core::Vec4 {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &super::core::Vec4 {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut super::core::Vec4 {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalVec4EntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalVec4EntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalVec4EntityData {
}

impl super::core::DataBusPeerTrait for ConditionalVec4EntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalVec4EntityData {
}

impl super::core::DataContainerTrait for ConditionalVec4EntityData {
}

pub static CONDITIONALVEC4ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4EntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalVec4EntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ConditionalVec4EntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec4",
                rust_offset: offset_of!(ConditionalVec4EntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALVEC4ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConditionalVec4EntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALVEC4ENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALVEC4ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec4EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalVec4EntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalVec3EntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: super::core::Vec3,
    pub value_if_false: super::core::Vec3,
}

pub trait ConditionalVec3EntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &super::core::Vec3;
    fn value_if_true_mut(&mut self) -> &mut super::core::Vec3;
    fn value_if_false(&self) -> &super::core::Vec3;
    fn value_if_false_mut(&mut self) -> &mut super::core::Vec3;
}

impl ConditionalVec3EntityDataTrait for ConditionalVec3EntityData {
    fn value_if_true(&self) -> &super::core::Vec3 {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &super::core::Vec3 {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut super::core::Vec3 {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalVec3EntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalVec3EntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalVec3EntityData {
}

impl super::core::DataBusPeerTrait for ConditionalVec3EntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalVec3EntityData {
}

impl super::core::DataContainerTrait for ConditionalVec3EntityData {
}

pub static CONDITIONALVEC3ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3EntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalVec3EntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ConditionalVec3EntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec3",
                rust_offset: offset_of!(ConditionalVec3EntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALVEC3ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 16,
};

impl TypeObject for ConditionalVec3EntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALVEC3ENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALVEC3ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec3EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalVec3EntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalVec2EntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: super::core::Vec2,
    pub value_if_false: super::core::Vec2,
}

pub trait ConditionalVec2EntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &super::core::Vec2;
    fn value_if_true_mut(&mut self) -> &mut super::core::Vec2;
    fn value_if_false(&self) -> &super::core::Vec2;
    fn value_if_false_mut(&mut self) -> &mut super::core::Vec2;
}

impl ConditionalVec2EntityDataTrait for ConditionalVec2EntityData {
    fn value_if_true(&self) -> &super::core::Vec2 {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &super::core::Vec2 {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut super::core::Vec2 {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalVec2EntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalVec2EntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalVec2EntityData {
}

impl super::core::DataBusPeerTrait for ConditionalVec2EntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalVec2EntityData {
}

impl super::core::DataContainerTrait for ConditionalVec2EntityData {
}

pub static CONDITIONALVEC2ENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2EntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalVec2EntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(ConditionalVec2EntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "Vec2",
                rust_offset: offset_of!(ConditionalVec2EntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALVEC2ENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalVec2EntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALVEC2ENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALVEC2ENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalVec2EntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalVec2EntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalFloatEntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: f32,
    pub value_if_false: f32,
}

pub trait ConditionalFloatEntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &f32;
    fn value_if_true_mut(&mut self) -> &mut f32;
    fn value_if_false(&self) -> &f32;
    fn value_if_false_mut(&mut self) -> &mut f32;
}

impl ConditionalFloatEntityDataTrait for ConditionalFloatEntityData {
    fn value_if_true(&self) -> &f32 {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut f32 {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &f32 {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut f32 {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalFloatEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalFloatEntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalFloatEntityData {
}

impl super::core::DataBusPeerTrait for ConditionalFloatEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalFloatEntityData {
}

impl super::core::DataContainerTrait for ConditionalFloatEntityData {
}

pub static CONDITIONALFLOATENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalFloatEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConditionalFloatEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "Float32",
                rust_offset: offset_of!(ConditionalFloatEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALFLOATENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalFloatEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALFLOATENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALFLOATENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalFloatEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalFloatEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalIntEntityData {
    pub _glacier_base: ConditionalStateEntityData,
    pub value_if_true: i32,
    pub value_if_false: i32,
}

pub trait ConditionalIntEntityDataTrait: ConditionalStateEntityDataTrait {
    fn value_if_true(&self) -> &i32;
    fn value_if_true_mut(&mut self) -> &mut i32;
    fn value_if_false(&self) -> &i32;
    fn value_if_false_mut(&mut self) -> &mut i32;
}

impl ConditionalIntEntityDataTrait for ConditionalIntEntityData {
    fn value_if_true(&self) -> &i32 {
        &self.value_if_true
    }
    fn value_if_true_mut(&mut self) -> &mut i32 {
        &mut self.value_if_true
    }
    fn value_if_false(&self) -> &i32 {
        &self.value_if_false
    }
    fn value_if_false_mut(&mut self) -> &mut i32 {
        &mut self.value_if_false
    }
}

impl ConditionalStateEntityDataTrait for ConditionalIntEntityData {
    fn realm(&self) -> &super::core::Realm {
        self._glacier_base.realm()
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        self._glacier_base.realm_mut()
    }
    fn condition(&self) -> &bool {
        self._glacier_base.condition()
    }
    fn condition_mut(&mut self) -> &mut bool {
        self._glacier_base.condition_mut()
    }
}

impl super::entity::EntityDataTrait for ConditionalIntEntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalIntEntityData {
}

impl super::core::DataBusPeerTrait for ConditionalIntEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalIntEntityData {
}

impl super::core::DataContainerTrait for ConditionalIntEntityData {
}

pub static CONDITIONALINTENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(CONDITIONALSTATEENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalIntEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "ValueIfTrue",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ConditionalIntEntityData, value_if_true),
            },
            FieldInfoData {
                name: "ValueIfFalse",
                flags: MemberInfoFlags::new(0),
                field_type: "Int32",
                rust_offset: offset_of!(ConditionalIntEntityData, value_if_false),
            },
        ],
    }),
    array_type: Some(CONDITIONALINTENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 8,
};

impl TypeObject for ConditionalIntEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALINTENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALINTENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalIntEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalIntEntityData"),
    array_type: None,
    alignment: 8,
};


#[derive(Clone, Debug, Default)]
pub struct ConditionalStateEntityData {
    pub _glacier_base: super::entity::EntityData,
    pub realm: super::core::Realm,
    pub condition: bool,
}

pub trait ConditionalStateEntityDataTrait: super::entity::EntityDataTrait {
    fn realm(&self) -> &super::core::Realm;
    fn realm_mut(&mut self) -> &mut super::core::Realm;
    fn condition(&self) -> &bool;
    fn condition_mut(&mut self) -> &mut bool;
}

impl ConditionalStateEntityDataTrait for ConditionalStateEntityData {
    fn realm(&self) -> &super::core::Realm {
        &self.realm
    }
    fn realm_mut(&mut self) -> &mut super::core::Realm {
        &mut self.realm
    }
    fn condition(&self) -> &bool {
        &self.condition
    }
    fn condition_mut(&mut self) -> &mut bool {
        &mut self.condition
    }
}

impl super::entity::EntityDataTrait for ConditionalStateEntityData {
}

impl super::entity::GameObjectDataTrait for ConditionalStateEntityData {
}

impl super::core::DataBusPeerTrait for ConditionalStateEntityData {
    fn flags(&self) -> &u32 {
        self._glacier_base.flags()
    }
    fn flags_mut(&mut self) -> &mut u32 {
        self._glacier_base.flags_mut()
    }
}

impl super::core::GameDataContainerTrait for ConditionalStateEntityData {
}

impl super::core::DataContainerTrait for ConditionalStateEntityData {
}

pub static CONDITIONALSTATEENTITYDATA_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStateEntityData",
    flags: MemberInfoFlags::new(101),
    module: "UIIncubatorShared",
    data: TypeInfoData::Class(ClassInfoData {
        super_class: Some(super::entity::ENTITYDATA_TYPE_INFO),
        functions: TypeFunctions {
            create: || Arc::new(Mutex::new(<ConditionalStateEntityData as Default>::default())),
        },
        fields: &[
            FieldInfoData {
                name: "Realm",
                flags: MemberInfoFlags::new(0),
                field_type: "Realm",
                rust_offset: offset_of!(ConditionalStateEntityData, realm),
            },
            FieldInfoData {
                name: "Condition",
                flags: MemberInfoFlags::new(0),
                field_type: "Boolean",
                rust_offset: offset_of!(ConditionalStateEntityData, condition),
            },
        ],
    }),
    array_type: Some(CONDITIONALSTATEENTITYDATA_ARRAY_TYPE_INFO),
    alignment: 0,
};

impl TypeObject for ConditionalStateEntityData {
    fn type_info(&self) -> &'static TypeInfo {
        CONDITIONALSTATEENTITYDATA_TYPE_INFO
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn data_container_core(&self) -> Option<&glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core()
    }
    fn data_container_core_mut(&mut self) -> Option<&mut glacier_reflect::data_container::DataContainerCore> {
        self._glacier_base.data_container_core_mut()
    }
}


pub static CONDITIONALSTATEENTITYDATA_ARRAY_TYPE_INFO: &'static TypeInfo = &TypeInfo {
    name: "ConditionalStateEntityData-Array",
    flags: MemberInfoFlags::new(145),
    module: "UIIncubatorShared",
    data: TypeInfoData::Array("ConditionalStateEntityData"),
    array_type: None,
    alignment: 8,
};


