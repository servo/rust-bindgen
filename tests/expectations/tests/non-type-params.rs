#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type Array16 = u8;
pub type ArrayInt4 = [u32; 4usize];
#[repr(C)]
#[derive(Default)]
pub struct UsesArray {
    pub array_char_16: [u8; 16usize],
    pub array_bool_8: [u8; 8usize],
    pub array_int_4: ArrayInt4,
}
#[test]
fn bindgen_test_layout_UsesArray() {
    assert_eq!(
        ::std::mem::size_of::<UsesArray>(),
        40usize,
        concat!("Size of: ", stringify!(UsesArray))
    );
    assert_eq!(
        ::std::mem::align_of::<UsesArray>(),
        4usize,
        concat!("Alignment of ", stringify!(UsesArray))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<UsesArray>())).array_char_16 as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesArray),
            "::",
            stringify!(array_char_16)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<UsesArray>())).array_bool_8 as *const _
                as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesArray),
            "::",
            stringify!(array_bool_8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<UsesArray>())).array_int_4 as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(UsesArray),
            "::",
            stringify!(array_int_4)
        )
    );
}
