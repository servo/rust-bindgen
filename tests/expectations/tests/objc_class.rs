/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]
#![cfg(target_os = "macos")]

#[macro_use]
extern crate objc;
#[allow(non_camel_case_types)]
pub type id = *mut objc::runtime::Object;
extern "C" {
    pub static mut fooVar: *mut id;
}
pub trait Foo {
    unsafe fn method(self);
}
impl Foo for id {
    unsafe fn method(self) {
        msg_send!(self, method)
    }
}
