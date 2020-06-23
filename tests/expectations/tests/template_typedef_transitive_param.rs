#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Wrapper {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Wrapper_Wrapped<T> {
    pub t: T,
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl<T> Default for Wrapper_Wrapped<T> {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type Wrapper_Type<T> = Wrapper_Wrapped<T>;
