/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WhitelistMe<T> {
    pub foo: ::std::os::raw::c_int,
    pub bar: WhitelistMe_Inner<T>,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WhitelistMe_Inner<T> {
    pub bar: T,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
impl <T> Default for WhitelistMe_Inner<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl <T> Default for WhitelistMe<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
