#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Default)]
pub struct a {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Default)]
pub struct b {
    _unused: [u8; 0],
}
