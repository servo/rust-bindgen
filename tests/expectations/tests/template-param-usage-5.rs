/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndirectlyUsesTemplateParameter<T> {
    pub aliased: IndirectlyUsesTemplateParameter_Aliased<T>,
    _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
}
pub type IndirectlyUsesTemplateParameter_Aliased<T> = T;
impl <T> Default for IndirectlyUsesTemplateParameter<T> {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
