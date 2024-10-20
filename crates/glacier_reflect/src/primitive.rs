use crate::{
    member::MemberInfoFlags,
    type_info::{TypeInfo, TypeInfoData},
};

macro_rules! declare_primitive_type {
    ($name: ident) => {
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
        }
    };
}

declare_primitive_type!(UInt8);
declare_primitive_type!(Int8);
declare_primitive_type!(UInt16);
declare_primitive_type!(Int16);
declare_primitive_type!(UInt32);
declare_primitive_type!(Int32);
declare_primitive_type!(UInt64);
declare_primitive_type!(Int64);
