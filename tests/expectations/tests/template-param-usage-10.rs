/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoublyIndirectUsage<T, U> {
    pub doubly_indirect: DoublyIndirectUsage_IndirectUsage<T, U>,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
pub type DoublyIndirectUsage_Aliased<T> = T;
pub type DoublyIndirectUsage_Typedefed<U> = U;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DoublyIndirectUsage_IndirectUsage<T, U> {
    pub member: DoublyIndirectUsage_Aliased<T>,
    pub another: DoublyIndirectUsage_Typedefed<U>,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<U>>,
}
impl <T, U> Default for DoublyIndirectUsage_IndirectUsage<T, U> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
impl <T, U> Default for DoublyIndirectUsage<T, U> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
