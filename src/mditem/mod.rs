use core_foundation_sys::base::{TCFTypeRef, CFTypeID};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::array::{CFArray, CFArrayRef};
use core_foundation::base::TCFType;
use libc::c_void;

#[repr(C)]
pub struct __MDItemRef(c_void);
pub type MDItemRef = *const __MDItemRef;

#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    pub static kMDItemURL: CFStringRef;

    fn MDItemCopyAttribute(item: MDItemRef, name: CFStringRef) -> *const c_void;
    fn MDItemCopyAttributeNames(item: MDItemRef) -> CFArrayRef;
    fn MDItemGetTypeID() -> CFTypeID;
}

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

    #[inline]
    pub fn get<T>(&self, key: CFStringRef) -> Option<T> where T: TCFType {
        let value = unsafe { MDItemCopyAttribute(self.0, key) };

        if value.is_null() {
            None
        } else {
            Some(unsafe { T::wrap_under_create_rule(T::Ref::from_void_ptr(value)) })
        }
    }
}
