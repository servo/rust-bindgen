#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

pub const Foo_Bar: Foo = 0;
pub const Foo_Qux: Foo = 1;
pub type Foo = u32;
pub mod Neg {
    pub type Type = i32;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}
