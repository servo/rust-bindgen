/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Debug, Copy)]
    pub struct a {
        pub val_a: *mut b,
    }
    #[test]
    fn bindgen_test_layout_a() {
        assert_eq!(::std::mem::size_of::<a>() , 8usize , concat ! (
                   "Size of: " , stringify ! ( a ) ));
        assert_eq! (::std::mem::align_of::<a>() , 8usize , concat ! (
                    "Alignment of " , stringify ! ( a ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const a ) ) . val_a as * const _ as usize }
                    , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( a ) , "::" ,
                    stringify ! ( val_a ) ));
    }
    impl Clone for a {
        fn clone(&self) -> Self { *self }
    }
    impl Default for a {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct b {
        pub val_b: ::std::os::raw::c_int,
    }
    #[test]
    fn bindgen_test_layout_b() {
        assert_eq!(::std::mem::size_of::<b>() , 4usize , concat ! (
                   "Size of: " , stringify ! ( b ) ));
        assert_eq! (::std::mem::align_of::<b>() , 4usize , concat ! (
                    "Alignment of " , stringify ! ( b ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const b ) ) . val_b as * const _ as usize }
                    , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( b ) , "::" ,
                    stringify ! ( val_b ) ));
    }
    impl Clone for b {
        fn clone(&self) -> Self { *self }
    }
}
