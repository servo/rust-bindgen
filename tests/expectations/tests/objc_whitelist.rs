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
impl WhitelistMe_InterestingCategory for WhitelistMe {}
pub trait WhitelistMe_InterestingCategory: Sized + std::ops::Deref {}
