#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub type PFN_VIGEM_X360_NOTIFICATION = ::std::option::Option<
    unsafe extern "C" fn(
        Client: *mut ::std::os::raw::c_void,
        Target: *mut ::std::os::raw::c_void,
        LargeMotor: ::std::os::raw::c_uchar,
        SmallMotor: ::std::os::raw::c_uchar,
        LedNumber: ::std::os::raw::c_uchar,
        UserData: *mut ::std::os::raw::c_void,
    ),
>;
