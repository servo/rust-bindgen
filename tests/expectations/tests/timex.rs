#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timex {
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 44usize], u8>,
}
#[test]
fn bindgen_test_layout_timex() {
    assert_eq!(
        ::std::mem::size_of::<timex>(),
        48usize,
        concat!("Size of: ", stringify!(timex))
    );
    assert_eq!(
        ::std::mem::align_of::<timex>(),
        4usize,
        concat!("Alignment of ", stringify!(timex))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timex>())).tai as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timex),
            "::",
            stringify!(tai)
        )
    );
}
impl Default for timex {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_timex {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_timex {}
impl Drop for Box_timex {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(48usize, 4usize).unwrap(),
            );
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timex_named {
    pub tai: ::std::os::raw::c_int,
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 44usize], u32>,
}
#[test]
fn bindgen_test_layout_timex_named() {
    assert_eq!(
        ::std::mem::size_of::<timex_named>(),
        48usize,
        concat!("Size of: ", stringify!(timex_named))
    );
    assert_eq!(
        ::std::mem::align_of::<timex_named>(),
        4usize,
        concat!("Alignment of ", stringify!(timex_named))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<timex_named>())).tai as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timex_named),
            "::",
            stringify!(tai)
        )
    );
}
impl Default for timex_named {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
struct Box_timex_named {
    ptr: *mut ::std::ffi::c_void,
}
impl Box_timex_named {}
impl Drop for Box_timex_named {
    fn drop(&mut self) {
        unsafe {
            ::std::alloc::dealloc(
                self.ptr as *mut u8,
                ::std::alloc::Layout::from_size_align(48usize, 4usize).unwrap(),
            );
        }
    }
}
