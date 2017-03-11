/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct Wrapper {
        pub _address: u8,
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Wrapper_Wrapped<T> {
        pub t: T,
    }
    impl <T> Default for Wrapper_Wrapped<T> {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    pub type Wrapper_Type<T> = Wrapper_Wrapped<T>;
}
