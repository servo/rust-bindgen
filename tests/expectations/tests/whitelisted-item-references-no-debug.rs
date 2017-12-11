/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct NoDebug {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_NoDebug() {
    assert_eq!(
        ::std::mem::size_of::<NoDebug>(),
        1usize,
        concat!("Size of: ", stringify!(NoDebug))
    );
    assert_eq!(
        ::std::mem::align_of::<NoDebug>(),
        1usize,
        concat!("Alignment of ", stringify!(NoDebug))
    );
}
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct WhitelistMe {
    pub a: NoDebug,
}
#[test]
fn bindgen_test_layout_WhitelistMe() {
    assert_eq!(
        ::std::mem::size_of::<WhitelistMe>(),
        1usize,
        concat!("Size of: ", stringify!(WhitelistMe))
    );
    assert_eq!(
        ::std::mem::align_of::<WhitelistMe>(),
        1usize,
        concat!("Alignment of ", stringify!(WhitelistMe))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<WhitelistMe>())).a as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(WhitelistMe),
            "::",
            stringify!(a)
        )
    );
}
