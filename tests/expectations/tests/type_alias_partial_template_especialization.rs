/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    pub type MaybeWrapped<A> = A;
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Rooted<T> {
        pub ptr: MaybeWrapped<T>,
    }
    impl <T> Default for Rooted<T> {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
}
