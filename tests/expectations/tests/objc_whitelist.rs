/* automatically generated by rust-bindgen */

#![cfg(target_os="macos")]

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[macro_use]
    extern crate objc;
    #[allow(non_camel_case_types)]
    pub type id = *mut objc::runtime::Object;
    #[allow(unused_imports)]
    use self::super::*;
    pub trait protocol_SomeProtocol {
        unsafe fn protocolMethod(self);
        unsafe fn protocolClassMethod();
    }
    impl protocol_SomeProtocol for id {
        unsafe fn protocolMethod(self) { msg_send!(self , protocolMethod) }
        unsafe fn protocolClassMethod() {
            msg_send!(objc :: runtime :: Class :: get ( "SomeProtocol" ) .
                      expect ( "Couldn\'t find SomeProtocol" ) ,
                      protocolClassMethod)
        }
    }
    pub trait WhitelistMe {
        unsafe fn method(self);
        unsafe fn classMethod();
    }
    impl WhitelistMe for id {
        unsafe fn method(self) { msg_send!(self , method) }
        unsafe fn classMethod() {
            msg_send!(objc :: runtime :: Class :: get ( "WhitelistMe" ) .
                      expect ( "Couldn\'t find WhitelistMe" ) , classMethod)
        }
    }
    pub trait WhitelistMe_InterestingCategory { }
    impl WhitelistMe_InterestingCategory for id { }
}
