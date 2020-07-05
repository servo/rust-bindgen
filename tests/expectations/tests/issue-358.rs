#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JS_PersistentRooted {
    pub _base: a,
}
impl Default for JS_PersistentRooted {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct a {
    pub b: *mut a,
}
impl Default for a {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
