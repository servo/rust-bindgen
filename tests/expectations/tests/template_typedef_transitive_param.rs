/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Wrapper {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Wrapper_Wrapped<T> {
    pub t: T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for Wrapper_Wrapped<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type Wrapper_Type<T> = Wrapper_Wrapped<T>;
