use std::any::Any;

use glacier_util::guid::Guid;

use crate::{
    member::MemberInfoFlags,
    type_info::{TypeInfo, TypeInfoData, TypeObject},
};

macro_rules! declare_builtin_type {
    ($name: ident, $type: ident) => {
        paste::paste! {
            pub const [<BUILTIN_ARRAY_TYPE_ $name:upper>]: &'static TypeInfo = &TypeInfo {
                name: concat!(stringify!($name), "-Array"),
                flags: MemberInfoFlags::new(0),
                module: "Intrinsic",
                data: TypeInfoData::Array(stringify!($name)),
                array_type: None,
                alignment: 0,
            };

            pub const [<BUILTIN_TYPE_ $name:upper>]: &'static TypeInfo = &TypeInfo {
                name: stringify!($name),
                flags: MemberInfoFlags::new(0),
                module: "Intrinsic",
                data: TypeInfoData::$name,
                array_type: Some([<BUILTIN_ARRAY_TYPE_ $name:upper>]),
                alignment: 0,
            };

            impl TypeObject for $type {
                fn type_info(&self) -> &'static TypeInfo {
                    [<BUILTIN_TYPE_ $name:upper>]
                }

                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        }
    };
}

#[derive(Default, Clone, Debug)]
pub struct ResourceRef {}

declare_builtin_type!(ResourceRef, ResourceRef);

#[derive(Default, Clone, Debug)]
pub struct TypeRef {}

declare_builtin_type!(TypeRef, TypeRef);

#[derive(Default, Clone, Debug)]
pub struct FileRef {}

declare_builtin_type!(FileRef, FileRef);

#[derive(Default, Clone, Debug)]
pub struct BoxedValueRef {}

declare_builtin_type!(BoxedValueRef, BoxedValueRef);

#[derive(Default, Clone, Debug)]
pub struct SHA1 {}

declare_builtin_type!(SHA1, SHA1);

declare_builtin_type!(Guid, Guid);
