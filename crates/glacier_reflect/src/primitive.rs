use std::any::Any;

use crate::{
    member::MemberInfoFlags,
    type_info::{TypeInfo, TypeInfoData, TypeObject},
};

macro_rules! declare_primitive_type {
    ($name: ident, $type: ident) => {
        paste::paste! {
            pub const [<PRIMITIVE_ARRAY_TYPE_ $name:upper>]: &'static TypeInfo = &TypeInfo {
                name: concat!(stringify!($name), "-Array"),
                flags: MemberInfoFlags::new(0),
                module: "Intrinsic",
                data: TypeInfoData::Array(stringify!($name)),
                array_type: None,
                alignment: 0,
            };

            pub const [<PRIMITIVE_TYPE_ $name:upper>]: &'static TypeInfo = &TypeInfo {
                name: stringify!($name),
                flags: MemberInfoFlags::new(0),
                module: "Intrinsic",
                data: TypeInfoData::$name,
                array_type: Some([<PRIMITIVE_ARRAY_TYPE_ $name:upper>]),
                alignment: 0,
            };

            impl TypeObject for $type {
                fn type_info(&self) -> &'static TypeInfo {
                    [<PRIMITIVE_TYPE_ $name:upper>]
                }

                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        }
    };
}

declare_primitive_type!(Uint8, u8);
declare_primitive_type!(Int8, i8);
declare_primitive_type!(Uint16, u16);
declare_primitive_type!(Int16, i16);
declare_primitive_type!(Uint32, u32);
declare_primitive_type!(Int32, i32);
declare_primitive_type!(Uint64, u64);
declare_primitive_type!(Int64, i64);
declare_primitive_type!(Float32, f32);
declare_primitive_type!(Float64, f64);
declare_primitive_type!(Boolean, bool);
declare_primitive_type!(CString, String);

#[test]
pub fn test_primitive_type_of() {
    assert_eq!(0u8.type_info(), PRIMITIVE_TYPE_UINT8);
    assert_eq!(0i8.type_info(), PRIMITIVE_TYPE_UINT16);
    assert_eq!(0u16.type_info(), PRIMITIVE_TYPE_UINT16);
    assert_eq!(0i16.type_info(), PRIMITIVE_TYPE_INT16);
    assert_eq!(0u32.type_info(), PRIMITIVE_TYPE_UINT32);
    assert_eq!(0i32.type_info(), PRIMITIVE_TYPE_INT32);
    assert_eq!(0u64.type_info(), PRIMITIVE_TYPE_UINT64);
    assert_eq!(0i64.type_info(), PRIMITIVE_TYPE_INT64);
    assert_eq!(false.type_info(), PRIMITIVE_TYPE_BOOLEAN);
    assert_eq!(String::new().type_info(), PRIMITIVE_TYPE_CSTRING);
}
