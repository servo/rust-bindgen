/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::*;
    #[repr(C)]
    #[derive(Debug, Default, Copy, Clone)]
    pub struct RandomTemplate {
        pub _address: u8,
    }
    /** <div rustbindgen opaque></div> */
    pub type ShouldBeOpaque = u8;
    pub type ShouldNotBeOpaque = RandomTemplate;
}
