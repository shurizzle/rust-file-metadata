use core_foundation_sys::base::{CFTypeID, CFTypeRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use libc::c_void;

#[repr(C)]
pub struct __MDItemRef(c_void);
pub type MDItemRef = *const __MDItemRef;

extern "C" {
    fn MDItemCopyAttribute(item: MDItemRef, name: CFStringRef) -> CFTypeRef;
    fn MDItemCopyAttributeNames(item: MDItemRef) -> CFArrayRef;
    fn MDItemGetTypeID() -> CFTypeID;
}

pub trait Attribute {
    type Type: TCFType;

    fn key(&self) -> CFStringRef;

    fn convert(value: <<Self as Attribute>::Type as TCFType>::Ref) -> Self::Type {
        unsafe {
            Self::Type::wrap_under_create_rule(value)
        }
    }
}

extern crate proc_macro;
use self::proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Ident, Token, Type};

struct Attr {
    name: Ident,
    ty: Type,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let ty: Type = input.parse()?;

        Ok(Attr {
            name,
            ty,
        })
    }
}

#[proc_macro]
pub fn attribute(input: TokenStream) -> TokenStream {
    let Attr {
        name,
        ty,
    } = parse_macro_input!(input as Attr);
    let konst = format!("kMDItem{}", name);
    let tmp_type = format!("__ATTRIBUTE__{}__TYPE__", name);

    let expanded = quote! {
        extern "C" {
            pub static #konst: core_foundation::string::CFStringRef;
        }

        #[allow(non_camel_case_types)]
        pub struct #tmp_type;

        impl file_metadata::mditem::Attribute for #tmp_type {
            type Type = #ty;

            fn key(&self) -> core_foundation::string::CFStringRef {
                unsafe { #konst }
            }
        }

        const #name: file_metadata::mditem::Attribute = #tmp_type { };
    };

    TokenStream::from(expanded)
}

// extern "C" {
//     pub static kMDItemURL: CFStringRef;
// }
//
// #[allow(non_camel_case_types)]
// pub struct __ATTRIBUTE__URL__TYPE__;
//
// impl Attribute for __ATTRIBUTE__URL__TYPE__ {
//     type Type = CFString;
//
//     fn key(&self) -> CFStringRef {
//         unsafe { kMDItemURL }
//     }
// }

attribute!(URL, CFString);

declare_TCFType!{
    MDItem, MDItemRef
}
impl_TCFType!(MDItem, MDItemRef, MDItemGetTypeID);

impl MDItem {
    pub fn attributes(&self) -> CFArray<CFString> {
        unsafe {
            TCFType::wrap_under_create_rule(MDItemCopyAttributeNames(self.0))
        }
    }
}
