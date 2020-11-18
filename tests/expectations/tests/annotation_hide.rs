#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

/// <div rustbindgen opaque></div>
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct D {
    pub _bindgen_opaque_blob: u32,
}
#[test]
fn bindgen_test_layout_D() {
    assert_eq!(
        ::std::mem::size_of::<D>(),
        4usize,
        concat!("Size of: ", stringify!(D))
    );
    assert_eq!(
        ::std::mem::align_of::<D>(),
        4usize,
        concat!("Alignment of ", stringify!(D))
    );
}
struct Box_D {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_D {}
impl Drop for Box_D {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct NotAnnotated {
    pub f: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_NotAnnotated() {
    assert_eq!(
        ::std::mem::size_of::<NotAnnotated>(),
        4usize,
        concat!("Size of: ", stringify!(NotAnnotated))
    );
    assert_eq!(
        ::std::mem::align_of::<NotAnnotated>(),
        4usize,
        concat!("Alignment of ", stringify!(NotAnnotated))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<NotAnnotated>())).f as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(NotAnnotated),
            "::",
            stringify!(f)
        )
    );
}
struct Box_NotAnnotated {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_NotAnnotated {}
impl Drop for Box_NotAnnotated {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(4usize, 4usize).unwrap(),
            );
        }
    }
}
