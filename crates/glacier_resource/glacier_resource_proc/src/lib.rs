use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(RvmTypeInfo, attributes(hash))]
pub fn derive_rvm_type_ino(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let mut hash = 0;

    let hash_attr = input.attrs.iter().find(|attr| attr.path.is_ident("hash"));
    if let Some(attr) = hash_attr {
        let meta = attr.parse_meta();
        if let Ok(syn::Meta::List(list)) = meta {
            if let Some(syn::NestedMeta::Lit(syn::Lit::Int(hash_value))) = list.nested.first() {
                hash = hash_value.base10_parse::<u32>().unwrap();
            } else {
                return syn::Error::new_spanned(
                    list,
                    "Expected #[hash(<integer>)] with exactly one value",
                )
                .to_compile_error()
                .into();
            }
        } else {
            return syn::Error::new_spanned(attr, "Expected #[hash(<integer>)]")
                .to_compile_error()
                .into();
        }
    } else {
        return syn::Error::new_spanned(input, "Expected #[hash(<integer>)]").to_compile_error().into();
    }

    TokenStream::from(quote! {
        impl #name {
            pub fn data_size() -> usize {
                std::mem::size_of::<Self>()
            }

            pub fn hash() -> u32 {
                #hash
            }

            pub fn self_hash(&self) -> u32 {
                #hash
            }

            pub fn from_slice(data: &[u8]) -> Self {
                unsafe { std::ptr::read(data.as_ptr() as *const Self) }
            }

            pub fn to_slice(&self) -> &[u8] {
                unsafe { std::slice::from_raw_parts(self as *const Self as *const u8, Self::data_size()) }
            }
        }
    })
}

#[proc_macro_derive(RvmTypeEnum)]
pub fn derive_rvm_type_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => {
            return syn::Error::new_spanned(input, "RvmTypeEnum can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let mut hash_match_arms = Vec::new();
    let mut hash_to_size_match_arms = Vec::new();
    let mut hash_from_slice_match_arms = Vec::new();
    let mut to_slice_match_arms = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        
        let variant_name_str = variant_name.to_string();
        let variant_name_ident = format_ident!("{}", variant_name_str);

        match &variant.fields {
            Fields::Unnamed(fields) => {
                let inner = &fields.unnamed[0];
                let inner_ty = &inner.ty;
                let stream = inner_ty.to_token_stream();
                let inner_ident = format_ident!("{}", stream.to_string());

                hash_match_arms.push(quote! {
                    #name::#variant_name_ident(inner) => inner.self_hash(),
                });

                hash_to_size_match_arms.push(quote! {
                    if hash == #inner_ident::hash() {
                        return Some(#inner_ident::data_size());
                    }
                });

                hash_from_slice_match_arms.push(quote! {
                    if hash == #inner_ident::hash() {
                        return Some(#name::#variant_name_ident(#inner_ident::from_slice(data)));
                    }
                });

                to_slice_match_arms.push(quote! {
                    #name::#variant_name_ident(inner) => {
                        inner.to_slice()
                    }
                });
            }
            Fields::Unit => {
                return syn::Error::new_spanned(
                    variant,
                    "RvmTypeEnum does not support unit variants in enums",
                )
                .to_compile_error()
                .into();
            }
            Fields::Named(_) => {
                return syn::Error::new_spanned(
                    variant,
                    "RvmTypeEnum does not support named fields in enum variants",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn hash(&self) -> u32 {
                match self {
                    #(#hash_match_arms)*
                }
            }

            pub fn hash_to_size(hash: u32) -> Option<usize> {
                #(#hash_to_size_match_arms)*

                None
            }

            pub fn from_slice(hash: u32, data: &[u8]) -> Option<Self> {
                assert_eq!(Self::hash_to_size(hash), Some(data.len()), "Invalid data size for hash: 0x{:X}", hash);

                #(#hash_from_slice_match_arms)*

                None
            }

            pub fn to_slice(&self) -> &[u8] {
                match self {
                    #(#to_slice_match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
