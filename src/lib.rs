//! Derive `Serialize` and `Deserialize` that uses short enum representation for C enum.
//!
//! A small variant of the [serde-repr](https://crates.io/crates/serde_repr) (99% of the code is
//! simple a copy/paste)
//!
//! # Examples
//!
//! ```
//! use serde_short::{Serialize_short, Deserialize_short};
//!
//! #[derive(Serialize_short, Deserialize_short, PartialEq, Debug)]
//! #[repr(C)]
//! enum SmallPrime {
//!     Two = 2,
//!     Three = 3,
//!     Five = 5,
//!     Seven = 7,
//! }
//!
//! fn main() -> serde_json::Result<()> {
//!     let j = serde_json::to_string(&SmallPrime::Seven)?;
//!     assert_eq!(j, "7");
//!
//!     let p: SmallPrime = serde_json::from_str("2")?;
//!     assert_eq!(p, SmallPrime::Two);
//!
//!     Ok(())
//! }
//! ```
extern crate proc_macro;

mod parse;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use crate::parse::Input;

#[proc_macro_derive(Serialize_short)]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;
    let repr = input.repr;

    let match_variants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            #ident::#variant => #ident::#variant as #repr,
        }
    });

    TokenStream::from(quote! {
        #[allow(deprecated)]
        impl serde::Serialize for #ident {
            #[allow(clippy::use_self)]
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                let value: #repr = match *self {
                    #(#match_variants)*
                };
                serde::Serialize::serialize(&value, serializer)
            }
        }
    })
}

#[proc_macro_derive(Deserialize_short, attributes(serde))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    let ident = input.ident;
    let repr = input.repr;
    let variants = input.variants.iter().map(|variant| &variant.ident);

    let declare_discriminants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            const #variant: #repr = #ident::#variant as #repr;
        }
    });

    let match_discriminants = input.variants.iter().map(|variant| {
        let variant = &variant.ident;
        quote! {
            discriminant::#variant => ::core::result::Result::Ok(#ident::#variant),
        }
    });

    let error_format = match input.variants.len() {
        1 => "invalid value: {}, expected {}".to_owned(),
        2 => "invalid value: {}, expected {} or {}".to_owned(),
        n => "invalid value: {}, expected one of: {}".to_owned() + &", {}".repeat(n - 1),
    };

    let other_arm = match input.default_variant {
        Some(variant) => {
            let variant = &variant.ident;
            quote! {
                ::core::result::Result::Ok(#ident::#variant)
            }
        }
        None => quote! {
            ::core::result::Result::Err(serde::de::Error::custom(
                format_args!(#error_format, other #(, discriminant::#variants)*)
            ))
        },
    };

    TokenStream::from(quote! {
        #[allow(deprecated)]
        impl<'de> serde::Deserialize<'de> for #ident {
            #[allow(clippy::use_self)]
            fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                struct discriminant;

                #[allow(non_upper_case_globals)]
                impl discriminant {
                    #(#declare_discriminants)*
                }

                match <#repr as serde::Deserialize>::deserialize(deserializer)? {
                    #(#match_discriminants)*
                    other => #other_arm,
                }
            }
        }
    })
}
