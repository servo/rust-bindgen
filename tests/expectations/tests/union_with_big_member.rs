/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[repr(C)]
    pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
    impl <T> __BindgenUnionField<T> {
        #[inline]
        pub fn new() -> Self {
            __BindgenUnionField(::std::marker::PhantomData)
        }
        #[inline]
        pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
        #[inline]
        pub unsafe fn as_mut(&mut self) -> &mut T {
            ::std::mem::transmute(self)
        }
    }
    impl <T> ::std::default::Default for __BindgenUnionField<T> {
        #[inline]
        fn default() -> Self { Self::new() }
    }
    impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
        #[inline]
        fn clone(&self) -> Self { Self::new() }
    }
    impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
    impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            fmt.write_str("__BindgenUnionField")
        }
    }
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Copy)]
    pub struct WithBigArray {
        pub a: __BindgenUnionField<::std::os::raw::c_int>,
        pub b: __BindgenUnionField<[::std::os::raw::c_int; 33usize]>,
        pub bindgen_union_field: [u32; 33usize],
    }
    #[test]
    fn bindgen_test_layout_WithBigArray() {
        assert_eq!(::std::mem::size_of::<WithBigArray>() , 132usize , concat !
                   ( "Size of: " , stringify ! ( WithBigArray ) ));
        assert_eq! (::std::mem::align_of::<WithBigArray>() , 4usize , concat !
                    ( "Alignment of " , stringify ! ( WithBigArray ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const WithBigArray ) ) . a as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( WithBigArray ) ,
                    "::" , stringify ! ( a ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const WithBigArray ) ) . b as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( WithBigArray ) ,
                    "::" , stringify ! ( b ) ));
    }
    impl Clone for WithBigArray {
        fn clone(&self) -> Self { *self }
    }
    impl Default for WithBigArray {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct WithBigArray2 {
        pub a: __BindgenUnionField<::std::os::raw::c_int>,
        pub b: __BindgenUnionField<[::std::os::raw::c_schar; 33usize]>,
        pub bindgen_union_field: [u32; 9usize],
    }
    #[test]
    fn bindgen_test_layout_WithBigArray2() {
        assert_eq!(::std::mem::size_of::<WithBigArray2>() , 36usize , concat !
                   ( "Size of: " , stringify ! ( WithBigArray2 ) ));
        assert_eq! (::std::mem::align_of::<WithBigArray2>() , 4usize , concat
                    ! ( "Alignment of " , stringify ! ( WithBigArray2 ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const WithBigArray2 ) ) . a as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( WithBigArray2 ) ,
                    "::" , stringify ! ( a ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const WithBigArray2 ) ) . b as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( WithBigArray2 ) ,
                    "::" , stringify ! ( b ) ));
    }
    impl Clone for WithBigArray2 {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Copy)]
    pub struct WithBigMember {
        pub a: __BindgenUnionField<::std::os::raw::c_int>,
        pub b: __BindgenUnionField<WithBigArray>,
        pub bindgen_union_field: [u32; 33usize],
    }
    #[test]
    fn bindgen_test_layout_WithBigMember() {
        assert_eq!(::std::mem::size_of::<WithBigMember>() , 132usize , concat
                   ! ( "Size of: " , stringify ! ( WithBigMember ) ));
        assert_eq! (::std::mem::align_of::<WithBigMember>() , 4usize , concat
                    ! ( "Alignment of " , stringify ! ( WithBigMember ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const WithBigMember ) ) . a as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( WithBigMember ) ,
                    "::" , stringify ! ( a ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const WithBigMember ) ) . b as * const _ as
                    usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( WithBigMember ) ,
                    "::" , stringify ! ( b ) ));
    }
    impl Clone for WithBigMember {
        fn clone(&self) -> Self { *self }
    }
    impl Default for WithBigMember {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
}
