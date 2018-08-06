/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T> {
    _alignment: [T; 0],
    _marker: ::std::marker::PhantomData<T>,
}
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField {
            _marker: ::std::marker::PhantomData,
            _alignment: Default::default(),
        }
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct test {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_test() {
    assert_eq!(
        ::std::mem::size_of::<test>(),
        4usize,
        concat!("Size of: ", stringify!(test))
    );
    assert_eq!(
        ::std::mem::align_of::<test>(),
        4usize,
        concat!("Alignment of ", stringify!(test))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<test>())).a as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(test), "::", stringify!(a))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<test>())).zero_length_array as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(test),
            "::",
            stringify!(zero_length_array)
        )
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct test2 {
    pub a: ::std::os::raw::c_int,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_test2() {
    assert_eq!(
        ::std::mem::size_of::<test2>(),
        4usize,
        concat!("Size of: ", stringify!(test2))
    );
    assert_eq!(
        ::std::mem::align_of::<test2>(),
        4usize,
        concat!("Alignment of ", stringify!(test2))
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct test3 {
    pub a: ::std::os::raw::c_int,
    pub zero_length_array: __IncompleteArrayField<::std::os::raw::c_char>,
    pub incomplete_array: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[test]
fn bindgen_test_layout_test3() {
    assert_eq!(
        ::std::mem::size_of::<test3>(),
        4usize,
        concat!("Size of: ", stringify!(test3))
    );
    assert_eq!(
        ::std::mem::align_of::<test3>(),
        4usize,
        concat!("Alignment of ", stringify!(test3))
    );
}
