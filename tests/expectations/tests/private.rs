#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HasPrivate {
    pub mNotPrivate: ::std::os::raw::c_int,
    /// <div rustbindgen private></div>
    mIsPrivate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_HasPrivate() {
    assert_eq!(
        ::std::mem::size_of::<HasPrivate>(),
        8usize,
        concat!("Size of: ", stringify!(HasPrivate))
    );
    assert_eq!(
        ::std::mem::align_of::<HasPrivate>(),
        4usize,
        concat!("Alignment of ", stringify!(HasPrivate))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<HasPrivate>() };
            let struct_ptr = &struct_instance as *const HasPrivate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mNotPrivate);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(HasPrivate),
            "::",
            stringify!(mNotPrivate)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<HasPrivate>() };
            let struct_ptr = &struct_instance as *const HasPrivate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mIsPrivate);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(HasPrivate),
            "::",
            stringify!(mIsPrivate)
        )
    );
}
/// <div rustbindgen private></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct VeryPrivate {
    mIsPrivate: ::std::os::raw::c_int,
    mIsAlsoPrivate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_VeryPrivate() {
    assert_eq!(
        ::std::mem::size_of::<VeryPrivate>(),
        8usize,
        concat!("Size of: ", stringify!(VeryPrivate))
    );
    assert_eq!(
        ::std::mem::align_of::<VeryPrivate>(),
        4usize,
        concat!("Alignment of ", stringify!(VeryPrivate))
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<VeryPrivate>() };
            let struct_ptr = &struct_instance as *const VeryPrivate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mIsPrivate);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VeryPrivate),
            "::",
            stringify!(mIsPrivate)
        )
    );
    assert_eq!(
        {
            let struct_instance = unsafe { std::mem::zeroed::<VeryPrivate>() };
            let struct_ptr = &struct_instance as *const VeryPrivate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mIsAlsoPrivate);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(VeryPrivate),
            "::",
            stringify!(mIsAlsoPrivate)
        )
    );
}
/// <div rustbindgen private></div>
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ContradictPrivate {
    /// <div rustbindgen private="false"></div>
    pub mNotPrivate: ::std::os::raw::c_int,
    mIsPrivate: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_ContradictPrivate() {
    assert_eq!(
        ::std::mem::size_of::<ContradictPrivate>(),
        8usize,
        concat!("Size of: ", stringify!(ContradictPrivate))
    );
    assert_eq!(
        ::std::mem::align_of::<ContradictPrivate>(),
        4usize,
        concat!("Alignment of ", stringify!(ContradictPrivate))
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ContradictPrivate>() };
            let struct_ptr = &struct_instance as *const ContradictPrivate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mNotPrivate);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ContradictPrivate),
            "::",
            stringify!(mNotPrivate)
        )
    );
    assert_eq!(
        {
            let struct_instance =
                unsafe { std::mem::zeroed::<ContradictPrivate>() };
            let struct_ptr = &struct_instance as *const ContradictPrivate;
            let field_ptr = std::ptr::addr_of!(struct_instance.mIsPrivate);
            let struct_address = struct_ptr as usize;
            let field_address = field_ptr as usize;
            std::mem::forget(struct_instance);
            field_address.checked_sub(struct_address).unwrap()
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(ContradictPrivate),
            "::",
            stringify!(mIsPrivate)
        )
    );
}
