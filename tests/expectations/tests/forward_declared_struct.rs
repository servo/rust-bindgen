#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct a {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_a() {
    assert_eq!(
        ::std::mem::size_of::<a>(),
        4usize,
        concat!("Size of: ", stringify!(a))
    );
    assert_eq!(
        ::std::mem::align_of::<a>(),
        4usize,
        concat!("Alignment of ", stringify!(a))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<a>() };
            let struct_ptr = &struct_instance as *const a;
            let field_ptr = std::ptr::addr_of!(struct_instance.b);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(a), "::", stringify!(b))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct c {
    pub d: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_c() {
    assert_eq!(
        ::std::mem::size_of::<c>(),
        4usize,
        concat!("Size of: ", stringify!(c))
    );
    assert_eq!(
        ::std::mem::align_of::<c>(),
        4usize,
        concat!("Alignment of ", stringify!(c))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<c>() };
            let struct_ptr = &struct_instance as *const c;
            let field_ptr = std::ptr::addr_of!(struct_instance.d);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!("Offset of field: ", stringify!(c), "::", stringify!(d))
    );
}
