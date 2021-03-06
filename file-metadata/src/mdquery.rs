#![allow(non_upper_case_globals)]

use core_foundation_sys::base::{CFTypeID, CFIndex, CFAllocatorRef, kCFAllocatorDefault};
use core_foundation::base::{TCFType, FromVoid, ItemRef};
use core_foundation::string::{CFString, CFStringRef};
use core_foundation::array::{CFArray, CFArrayRef};
use libc::{c_void, c_ulong};

use crate::mditem::MDItem;

pub type OptionBits = u32;
pub type CFOptionFlags = c_ulong;

#[repr(C)]
pub struct __MDQuery(c_void);
pub type MDQueryRef = *const __MDQuery;

bitflags! {
    pub struct MDQueryOptionFlags: CFOptionFlags {
        const SYNC = 1;
        const WANTS_UPDATES = 4;
        const ALLOW_FS_TRANSLATION = 8;
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct MDQueryBatchingParams {
    first_max_num: usize,
    first_max_ms: usize,
    progress_max_num: usize,
    progress_max_ms: usize,
    update_max_num: usize,
    update_max_ms: usize,
}

// use core_foundation::base::{CFComparisonResult, CFTypeRef};

// type MDQueryCreateValueFunction = extern "C" fn(query: MDQueryRef, attrName: CFStringRef, attrValue: CFTypeRef, context: *const c_void) -> *const c_void;
// type MDQueryCreateResultFunction = extern "C" fn(query: MDQueryRef, item: MDItemRef, context: *const c_void) -> *const c_void;
// type MDQuerySortComparatorFunction = extern "C" fn(attrs1: *const CFTypeRef, attrs2: *const CFTypeRef, context: *const c_void) -> CFComparisonResult;

#[allow(non_snake_case)]
#[link(name = "CoreServices", kind = "framework")]
extern "C" {
    fn MDQueryCreate(allocator: CFAllocatorRef, queryString: CFStringRef, valueListAttrs: CFArrayRef, sortingAttrs: CFArrayRef) -> MDQueryRef;
    // fn MDQueryCreateSubset(allocator: CFAllocatorRef, query: MDQueryRef, queryString: CFStringRef, valueListAttrs: CFArrayRef, sortingAttrs: CFArrayRef) -> MDQueryRef;
    // fn MDQuerySetSearchScope(query: MDQueryRef, scopeDirectories: CFArrayRef, scopeOptions: OptionBits);
    // fn MDQuerySetDispatchQueue(query: MDQueryRef, queue: *const c_void/* TODO */);

    // fn MDQuerySetMaxCount(query: MDQueryRef, size: CFIndex);
    // fn MDQueryGetBatchingParameters(query: MDQueryRef) -> MDQueryBatchingParams;
    // fn MDQuerySetBatchingParameters(query: MDQueryRef, params: MDQueryBatchingParams);
    // fn MDQueryCopyValueListAttributes(query: MDQueryRef) -> CFArrayRef;
    // fn MDQueryCopySortingAttributes(query: MDQueryRef) -> CFArrayRef;
    fn MDQueryCopyQueryString(query: MDQueryRef) -> CFStringRef;


    fn MDQueryExecute(query: MDQueryRef, optionFlags: CFOptionFlags) -> bool;
    fn MDQueryStop(query: MDQueryRef);
    // fn MDQueryDisableUpdates(query: MDQueryRef);
    // fn MDQueryEnableUpdates(query: MDQueryRef);
    // fn MDQueryIsGatheringComplete(query: MDQueryRef) -> bool;

    // fn MDQueryCopyValuesOfAttribute(query: MDQueryRef, name: CFStringRef) -> CFArrayRef;
    // fn MDQueryGetAttributeValueOfResultAtIndex(query: MDQueryRef, name: CFStringRef, idx: CFIndex) -> *const c_void;
    // fn MDQueryGetCountOfResultsWithAttributeValue(query: MDQueryRef, name: CFStringRef, value: CFTypeRef) -> CFIndex;
    // fn MDQueryGetIndexOfResult(query: MDQueryRef, result: *const c_void) -> CFIndex;
    fn MDQueryGetResultAtIndex(query: MDQueryRef, idx: CFIndex) -> *const c_void;
    fn MDQueryGetResultCount(query: MDQueryRef) -> CFIndex;
    // fn MDQuerySetSortComparatorBlock(query: MDQueryRef, comparator: extern "C" fn(attrs1: *const CFTypeRef, attrs2: *const CFTypeRef) -> CFComparisonResult);

    fn MDQueryGetTypeID() -> CFTypeID;
}

pub struct MDQueryIterator<'a> {
    query: &'a MDQuery,
    index: CFIndex,
    len: CFIndex,
}

impl<'a> Iterator for MDQueryIterator<'a> {
    type Item = ItemRef<'a, MDItem>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let value = unsafe { self.query.get_unchecked(self.index) };
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for MDQueryIterator<'a> {
    fn len(&self) -> usize {
        (self.query.len() - self.index) as usize
    }
}

declare_TCFType!{
    MDQuery, MDQueryRef
}
impl_TCFType!(MDQuery, MDQueryRef, MDQueryGetTypeID);

impl MDQuery {
    pub fn new(query_string: CFString, value_list_attrs: Option<CFArray>, sorting_attrs: Option<CFArray>) -> Option<Self> {
        unsafe {
            let query_string = query_string.as_concrete_TypeRef();
            let value_list_attrs = value_list_attrs.map(|v| v.as_concrete_TypeRef()).unwrap_or_else(::std::ptr::null);
            let sorting_attrs = sorting_attrs.map(|v| v.as_concrete_TypeRef()).unwrap_or_else(::std::ptr::null);
            let query_ref = MDQueryCreate(kCFAllocatorDefault, query_string, value_list_attrs, sorting_attrs);

            if query_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(query_ref))
            }
        }
    }

    #[inline]
    pub fn execute(&self, flags: MDQueryOptionFlags) -> bool {
        unsafe {
            MDQueryExecute(self.0, flags.bits())
        }
    }

    #[inline]
    pub fn stop(&self) {
        unsafe {
            MDQueryStop(self.0);
        }
    }

    #[inline]
    pub fn len(&self) -> CFIndex {
        unsafe {
            MDQueryGetResultCount(self.0)
        }
    }

    #[inline]
    pub(crate) unsafe fn get_unchecked<'a>(&'a self, idx: CFIndex) -> ItemRef<'a, MDItem> {
        MDItem::from_void(MDQueryGetResultAtIndex(self.0, idx))
    }

    #[inline]
    pub fn get<'a>(&'a self, idx: CFIndex) -> Option<ItemRef<'a, MDItem>> {
        if idx < self.len() {
            Some(unsafe { self.get_unchecked(idx) })
        } else {
            None
        }
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> MDQueryIterator<'a> {
        MDQueryIterator {
            query: self,
            index: 0,
            len: self.len(),
        }
    }

    #[inline]
    pub fn query_string(&self) -> CFString {
        unsafe {
            TCFType::wrap_under_create_rule(MDQueryCopyQueryString(self.0))
        }
    }
}

impl ::std::fmt::Debug for MDQuery {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "\"{:?}\"", self.query_string())
    }
}

#[cfg(test)]
mod tests {
    use core_foundation::string::CFString;
    use crate::mdquery::*;
    use crate::mditem::attributes::Path;

    #[test]
    fn it_works() {
        let query_string = "kMDItemContentTypeTree == \"com.apple.application\"c";
        let query_cfstring = CFString::new(query_string);
        let query = MDQuery::new(query_cfstring, None, None).unwrap();
        query.execute(MDQueryOptionFlags::SYNC | MDQueryOptionFlags::ALLOW_FS_TRANSLATION);
        println!("{:#?}", query.iter().map(|v| v.get(Path).unwrap()).collect::<Vec<_>>());
    }
}
