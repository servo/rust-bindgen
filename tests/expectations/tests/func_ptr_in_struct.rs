/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum baz { }
    #[repr(C)]
    #[derive(Debug, Copy)]
    pub struct Foo {
        pub bar: ::std::option::Option<unsafe extern "C" fn(x:
                                                                ::std::os::raw::c_int,
                                                            y:
                                                                ::std::os::raw::c_int)
                                           -> baz>,
    }
    #[test]
    fn bindgen_test_layout_Foo() {
        assert_eq!(::std::mem::size_of::<Foo>() , 8usize , concat ! (
                   "Size of: " , stringify ! ( Foo ) ));
        assert_eq! (::std::mem::align_of::<Foo>() , 8usize , concat ! (
                    "Alignment of " , stringify ! ( Foo ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const Foo ) ) . bar as * const _ as usize }
                    , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( Foo ) , "::" ,
                    stringify ! ( bar ) ));
    }
    impl Clone for Foo {
        fn clone(&self) -> Self { *self }
    }
    impl Default for Foo {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
}
