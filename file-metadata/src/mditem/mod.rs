extern crate file_metadata_mditem_macros;

use std::marker::PhantomData;

use core_foundation_sys::base::{CFAllocatorRef, kCFAllocatorDefault};
use core_foundation_sys::base::{TCFTypeRef, CFTypeID};
use core_foundation::url::{CFURL, CFURLRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use libc::c_void;

#[repr(C)]
pub struct __MDItemRef(c_void);
pub type MDItemRef = *const __MDItemRef;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn MDItemCreate(allocator: CFAllocatorRef, path: CFStringRef) -> MDItemRef;
    fn MDItemCreateWithURL(allocator: CFAllocatorRef, url: CFURLRef) -> MDItemRef;
    fn MDItemCopyAttribute(item: MDItemRef, name: CFStringRef) -> *const c_void;
    fn MDItemCopyAttributeNames(item: MDItemRef) -> CFArrayRef;
    fn MDItemGetTypeID() -> CFTypeID;
}

pub struct Attribute<T> {
    pub key_getter: fn() -> CFStringRef,
    pub phantom: PhantomData<T>
}

impl<T: TCFType> Attribute<T> {
    pub fn key(&self) -> CFStringRef {
        (self.key_getter)()
    }
}

pub mod attributes;

declare_TCFType!{
    MDItem, MDItemRef
}
impl_TCFType!(MDItem, MDItemRef, MDItemGetTypeID);

impl MDItem {
    pub fn from_string(path: CFString) -> Option<MDItem> {
        let ptr = unsafe { MDItemCreate(kCFAllocatorDefault, path.as_concrete_TypeRef()) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { TCFType::wrap_under_create_rule(ptr) })
        }
    }

    pub fn from_url(url: CFURL) -> Option<MDItem> {
        let ptr = unsafe { MDItemCreateWithURL(kCFAllocatorDefault, url.as_concrete_TypeRef()) };

        if ptr.is_null() {
            None
        } else {
            Some(unsafe { TCFType::wrap_under_create_rule(ptr) })
        }
    }

    pub fn attributes(&self) -> CFArray<CFString> {
        unsafe {
            TCFType::wrap_under_create_rule(MDItemCopyAttributeNames(self.0))
        }
    }

    #[inline]
    pub fn get<T>(&self, attr: Attribute<T>) -> Option<T> where T: TCFType {
        let value = unsafe { MDItemCopyAttribute(self.0, attr.key()) };

        if value.is_null() {
            None
        } else {
            Some(unsafe { T::wrap_under_create_rule(T::Ref::from_void_ptr(value)) })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MDItem, attributes::{CFBundleIdentifier, DisplayName}};
    use core_foundation::url::CFURL;
    use core_foundation::string::CFString;
    use std::path::Path;

    #[test]
    fn get_safari_bundle_id_and_display_name() {
        let safari_url = CFURL::from_path(Path::new("/Applications/Safari.app"), true).unwrap();
        let safari_item = MDItem::from_url(safari_url).unwrap();
        assert_eq!(safari_item.get(CFBundleIdentifier), Some(CFString::new("com.apple.Safari")));
        assert_eq!(safari_item.get(DisplayName), Some(CFString::new("Safari")));
    }
}
