/* automatically generated by rust-bindgen */

#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

macro_rules ! c { ( ) => { } ; ( # include $ filename : tt $ ( $ rest : tt ) * ) => { c ! { $ ( $ rest ) * } } ; ( { $ ( $ code : tt ) * } $ ( $ rest : tt ) * ) => { c ! { $ ( $ rest ) * } } ; }
c! { # include "thread_local_vars.h" }
extern "C" {
    pub fn i() -> ::std::os::raw::c_int;
    pub fn set_i(v: ::std::os::raw::c_int);
}
c! { { int i ( ) { return i ; } } }
c! { { void set_i ( int v ) { i = v ; } } }
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct state {
    _unused: [u8; 0],
}
extern "C" {
    pub fn s() -> state;
    pub fn set_s(v: state);
}
c! { { struct state s ( ) { return s ; } } }
c! { { void set_s ( struct state v ) { s = v ; } } }
extern "C" {
    pub fn p() -> *mut ::std::os::raw::c_char;
    pub fn set_p(v: *mut ::std::os::raw::c_char);
}
c! { { char * p ( ) { return p ; } } }
c! { { void set_p ( char * v ) { p = v ; } } }
