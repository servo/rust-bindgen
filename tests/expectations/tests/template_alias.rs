/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    pub type JS_detail_Wrapped<T> = T;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct JS_Rooted<T> {
        pub ptr: JS_detail_Wrapped<T>,
    }
    impl <T> Default for JS_Rooted<T> {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
}
