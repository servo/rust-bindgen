/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct A {
        pub foo: usize,
    }
    #[test]
    fn bindgen_test_layout_A() {
        assert_eq!(::std::mem::size_of::<A>() , 8usize , concat ! (
                   "Size of: " , stringify ! ( A ) ));
        assert_eq! (::std::mem::align_of::<A>() , 8usize , concat ! (
                    "Alignment of " , stringify ! ( A ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const A ) ) . foo as * const _ as usize } ,
                    0usize , concat ! (
                    "Alignment of field: " , stringify ! ( A ) , "::" ,
                    stringify ! ( foo ) ));
    }
    impl Clone for A {
        fn clone(&self) -> Self { *self }
    }
}
