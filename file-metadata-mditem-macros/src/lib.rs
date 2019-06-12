extern crate proc_macro;
extern crate syn;
extern crate quote;

use proc_macro::{TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token, Type};
use syn::export::Span;
use std::env;

struct Attribute {
    name: Ident,
    ty: Type,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let ty: Type = input.parse()?;

        Ok(Attribute {
            name,
            ty,
        })
    }
}

#[proc_macro]
pub fn def_attribute(input: TokenStream) -> TokenStream {
    let Attribute {
        name,
        ty,
    } = parse_macro_input!(input as Attribute);
    let konst_name = format!("kMDItem{}", name);
    let konst = Ident::new(&konst_name, Span::call_site());
    let krate = if env::var("CARGO_PKG_NAME").unwrap() == "file-metadata" {
        quote!(crate)
    } else {
        quote!(::file_metadata)
    };

    let expanded = quote! {
        #[link(name = "CoreServices", kind = "framework")]
        extern "C" {
            pub static #konst: ::core_foundation::string::CFStringRef;
        }

        #[allow(non_upper_case_globals)]
        pub const #name: #krate::mditem::Attribute<#ty> = #krate::mditem::Attribute {
            key_getter: || { unsafe { #konst } },
            phantom: ::std::marker::PhantomData
        };
    };

    TokenStream::from(expanded)
}
