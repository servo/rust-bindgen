#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct foo {
    pub member: foo__bindgen_ty_1,
}
pub const foo_FOO_A: foo__bindgen_ty_1 = foo__bindgen_ty_1(0);
pub const foo_FOO_B: foo__bindgen_ty_1 = foo__bindgen_ty_1(1);
impl ::std::ops::BitOr<foo__bindgen_ty_1> for foo__bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        foo__bindgen_ty_1(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for foo__bindgen_ty_1 {
    #[inline]
    fn bitor_assign(&mut self, rhs: foo__bindgen_ty_1) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<foo__bindgen_ty_1> for foo__bindgen_ty_1 {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        foo__bindgen_ty_1(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for foo__bindgen_ty_1 {
    #[inline]
    fn bitand_assign(&mut self, rhs: foo__bindgen_ty_1) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct foo__bindgen_ty_1(pub ::std::os::raw::c_uint);
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<foo>())).member as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(foo),
            "::",
            stringify!(member)
        )
    );
}
impl Default for foo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Foo {
    pub const Bar: Foo = Foo(0);
}
impl Foo {
    pub const Qux: Foo = Foo(1);
}
impl ::std::ops::BitOr<Foo> for Foo {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        Foo(self.0 | other.0)
    }
}
impl ::std::ops::BitOrAssign for Foo {
    #[inline]
    fn bitor_assign(&mut self, rhs: Foo) {
        self.0 |= rhs.0;
    }
}
impl ::std::ops::BitAnd<Foo> for Foo {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        Foo(self.0 & other.0)
    }
}
impl ::std::ops::BitAndAssign for Foo {
    #[inline]
    fn bitand_assign(&mut self, rhs: Foo) {
        self.0 &= rhs.0;
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Foo(pub ::std::os::raw::c_uint);
pub mod Neg {
    pub type Type = ::std::os::raw::c_int;
    pub const MinusOne: Type = -1;
    pub const One: Type = 1;
}
