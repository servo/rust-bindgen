/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct BaseUsesT<T> {
        pub t: *mut T,
    }
    impl <T> Default for BaseUsesT<T> {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct CrtpUsesU<U> {
        pub _base: BaseUsesT<CrtpUsesU<U>>,
        pub usage: U,
    }
    impl <U> Default for CrtpUsesU<U> {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
}
