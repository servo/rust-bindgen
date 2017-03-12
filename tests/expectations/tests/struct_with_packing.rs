/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C, packed)]
    #[derive(Debug, Default, Copy)]
    pub struct a {
        pub b: ::std::os::raw::c_char,
        pub c: ::std::os::raw::c_short,
    }
    #[test]
    fn bindgen_test_layout_a() {
        assert_eq!(::std::mem::size_of::<a>() , 3usize , concat ! (
                   "Size of: " , stringify ! ( a ) ));
        assert_eq! (::std::mem::align_of::<a>() , 1usize , concat ! (
                    "Alignment of " , stringify ! ( a ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const a ) ) . b as * const _ as usize } ,
                    0usize , concat ! (
                    "Alignment of field: " , stringify ! ( a ) , "::" ,
                    stringify ! ( b ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const a ) ) . c as * const _ as usize } ,
                    1usize , concat ! (
                    "Alignment of field: " , stringify ! ( a ) , "::" ,
                    stringify ! ( c ) ));
    }
    impl Clone for a {
        fn clone(&self) -> Self { *self }
    }
}
