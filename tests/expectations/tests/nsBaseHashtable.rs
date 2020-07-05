#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type size_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsBaseHashtableET {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct nsTHashtable {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsBaseHashtable {
    pub _address: u8,
}
pub type nsBaseHashtable_KeyType = [u8; 0usize];
pub type nsBaseHashtable_EntryType = nsBaseHashtableET;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nsBaseHashtable_LookupResult {
    pub mEntry: *mut nsBaseHashtable_EntryType,
    pub mTable: *mut nsBaseHashtable,
}
impl Default for nsBaseHashtable_LookupResult {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug)]
pub struct nsBaseHashtable_EntryPtr {
    pub mEntry: *mut nsBaseHashtable_EntryType,
    pub mExistingEntry: bool,
}
impl Default for nsBaseHashtable_EntryPtr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for nsBaseHashtable {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
