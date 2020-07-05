#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BaseUsesT<T> {
    pub t: *mut T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for BaseUsesT<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CrtpUsesU<U> {
    pub _base: BaseUsesT<CrtpUsesU<U>>,
    pub usage: U,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl<U> Default for CrtpUsesU<U> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
