/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[repr(C)]
pub struct max_align_t {
    pub _bindgen_opaque_blob: [u8; 32usize],
}
#[test]
fn bindgen_test_layout_max_align_t() {
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
}
impl Default for max_align_t {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
