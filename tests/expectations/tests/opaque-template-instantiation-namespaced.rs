#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod zoidberg {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Template<T> {
            pub member: T,
            pub _phantom_0:
                ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
        }
        impl<T> Default for Template<T> {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Foo {
            pub c: ::std::os::raw::c_char,
        }
        #[test]
        fn bindgen_test_layout_Foo() {
            assert_eq!(
                ::std::mem::size_of::<Foo>(),
                1usize,
                concat!("Size of: ", stringify!(Foo))
            );
            assert_eq!(
                ::std::mem::align_of::<Foo>(),
                1usize,
                concat!("Alignment of ", stringify!(Foo))
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<Foo>())).c as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Foo),
                    "::",
                    stringify!(c)
                )
            );
        }
        struct Box_Foo {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Foo {}
        impl Drop for Box_Foo {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(1usize, 1usize)
                            .unwrap(),
                    );
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct Bar {
            pub i: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(
                ::std::mem::size_of::<Bar>(),
                4usize,
                concat!("Size of: ", stringify!(Bar))
            );
            assert_eq!(
                ::std::mem::align_of::<Bar>(),
                4usize,
                concat!("Alignment of ", stringify!(Bar))
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<Bar>())).i as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(Bar),
                    "::",
                    stringify!(i)
                )
            );
        }
        struct Box_Bar {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_Bar {}
        impl Drop for Box_Bar {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(4usize, 4usize)
                            .unwrap(),
                    );
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct ContainsInstantiation {
            pub not_opaque: root::zoidberg::Template<root::zoidberg::Foo>,
        }
        #[test]
        fn bindgen_test_layout_ContainsInstantiation() {
            assert_eq!(
                ::std::mem::size_of::<ContainsInstantiation>(),
                1usize,
                concat!("Size of: ", stringify!(ContainsInstantiation))
            );
            assert_eq!(
                ::std::mem::align_of::<ContainsInstantiation>(),
                1usize,
                concat!("Alignment of ", stringify!(ContainsInstantiation))
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<ContainsInstantiation>())).not_opaque
                        as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(ContainsInstantiation),
                    "::",
                    stringify!(not_opaque)
                )
            );
        }
        impl Default for ContainsInstantiation {
            fn default() -> Self {
                unsafe { ::std::mem::zeroed() }
            }
        }
        struct Box_ContainsInstantiation {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_ContainsInstantiation {}
        impl Drop for Box_ContainsInstantiation {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(1usize, 1usize)
                            .unwrap(),
                    );
                }
            }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
        pub struct ContainsOpaqueInstantiation {
            pub opaque: u32,
        }
        #[test]
        fn bindgen_test_layout_ContainsOpaqueInstantiation() {
            assert_eq!(
                ::std::mem::size_of::<ContainsOpaqueInstantiation>(),
                4usize,
                concat!("Size of: ", stringify!(ContainsOpaqueInstantiation))
            );
            assert_eq!(
                ::std::mem::align_of::<ContainsOpaqueInstantiation>(),
                4usize,
                concat!(
                    "Alignment of ",
                    stringify!(ContainsOpaqueInstantiation)
                )
            );
            assert_eq!(
                unsafe {
                    &(*(::std::ptr::null::<ContainsOpaqueInstantiation>()))
                        .opaque as *const _ as usize
                },
                0usize,
                concat!(
                    "Offset of field: ",
                    stringify!(ContainsOpaqueInstantiation),
                    "::",
                    stringify!(opaque)
                )
            );
        }
        struct Box_ContainsOpaqueInstantiation {
            ptr: *mut ::std::ffi::c_void,
        }
        impl Box_ContainsOpaqueInstantiation {}
        impl Drop for Box_ContainsOpaqueInstantiation {
            fn drop(&mut self) {
                unsafe {
                    ::std::alloc::dealloc(
                        self.ptr as *mut u8,
                        ::std::alloc::Layout::from_size_align(4usize, 4usize)
                            .unwrap(),
                    );
                }
            }
        }
    }
    #[test]
    fn __bindgen_test_layout_Template_open0_Foo_close0_instantiation() {
        assert_eq!(
            ::std::mem::size_of::<root::zoidberg::Template<root::zoidberg::Foo>>(
            ),
            1usize,
            concat!(
                "Size of template specialization: ",
                stringify!(root::zoidberg::Template<root::zoidberg::Foo>)
            )
        );
        assert_eq!(
            ::std::mem::align_of::<root::zoidberg::Template<root::zoidberg::Foo>>(
            ),
            1usize,
            concat!(
                "Alignment of template specialization: ",
                stringify!(root::zoidberg::Template<root::zoidberg::Foo>)
            )
        );
    }
}
