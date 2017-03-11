/* automatically generated by rust-bindgen */

pub use self::root::*;

#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[repr(C)]
    pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
    impl <T> __BindgenUnionField<T> {
        #[inline]
        pub fn new() -> Self {
            __BindgenUnionField(::std::marker::PhantomData)
        }
        #[inline]
        pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
        #[inline]
        pub unsafe fn as_mut(&mut self) -> &mut T {
            ::std::mem::transmute(self)
        }
    }
    impl <T> ::std::default::Default for __BindgenUnionField<T> {
        #[inline]
        fn default() -> Self { Self::new() }
    }
    impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
        #[inline]
        fn clone(&self) -> Self { Self::new() }
    }
    impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
    impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
        fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            fmt.write_str("__BindgenUnionField")
        }
    }
    #[allow(unused_imports)]
    use self::super::*;
    pub const RTE_CACHE_LINE_MIN_SIZE: ::std::os::raw::c_uint = 64;
    pub const RTE_CACHE_LINE_SIZE: ::std::os::raw::c_uint = 64;
    pub type phys_addr_t = u64;
    pub type MARKER = [*mut ::std::os::raw::c_void; 0usize];
    pub type MARKER8 = [u8; 0usize];
    pub type MARKER64 = [u64; 0usize];
    /**
 * The atomic counter structure.
 */
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_atomic16_t {
        /**< An internal counter value. */
        pub cnt: i16,
    }
    #[test]
    fn bindgen_test_layout_rte_atomic16_t() {
        assert_eq!(::std::mem::size_of::<rte_atomic16_t>() , 2usize , concat !
                   ( "Size of: " , stringify ! ( rte_atomic16_t ) ));
        assert_eq! (::std::mem::align_of::<rte_atomic16_t>() , 2usize , concat
                    ! ( "Alignment of " , stringify ! ( rte_atomic16_t ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_atomic16_t ) ) . cnt as * const _
                    as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_atomic16_t ) ,
                    "::" , stringify ! ( cnt ) ));
    }
    impl Clone for rte_atomic16_t {
        fn clone(&self) -> Self { *self }
    }
    /**
 * The generic rte_mbuf, containing a packet mbuf.
 */
    #[repr(C)]
    #[derive(Debug, Copy)]
    pub struct rte_mbuf {
        pub cacheline0: MARKER,
        /**< Virtual address of segment buffer. */
        pub buf_addr: *mut ::std::os::raw::c_void,
        /**< Physical address of segment buffer. */
        pub buf_physaddr: phys_addr_t,
        /**< Length of segment buffer. */
        pub buf_len: u16,
        pub rearm_data: MARKER8,
        pub data_off: u16,
        pub __bindgen_anon_1: rte_mbuf__bindgen_ty_1,
        /**< Number of segments. */
        pub nb_segs: u8,
        /**< Input port. */
        pub port: u8,
        /**< Offload features. */
        pub ol_flags: u64,
        pub rx_descriptor_fields1: MARKER,
        pub __bindgen_anon_2: rte_mbuf__bindgen_ty_2,
        /**< Total pkt len: sum of all segments. */
        pub pkt_len: u32,
        /**< Amount of data in segment buffer. */
        pub data_len: u16,
        /** VLAN TCI (CPU order), valid if PKT_RX_VLAN_STRIPPED is set. */
        pub vlan_tci: u16,
        /**< hash information */
        pub hash: rte_mbuf__bindgen_ty_3,
        /**< Sequence number. See also rte_reorder_insert() */
        pub seqn: u32,
        /** Outer VLAN TCI (CPU order), valid if PKT_RX_QINQ_STRIPPED is set. */
        pub vlan_tci_outer: u16,
        pub cacheline1: MARKER,
        pub __bindgen_anon_3: rte_mbuf__bindgen_ty_4,
        /**< Pool from which mbuf was allocated. */
        pub pool: *mut rte_mempool,
        /**< Next segment of scattered packet. */
        pub next: *mut rte_mbuf,
        pub __bindgen_anon_4: rte_mbuf__bindgen_ty_5,
        /** Size of the application private data. In case of an indirect
	 * mbuf, it stores the direct mbuf private data size. */
        pub priv_size: u16,
        /** Timesync flags for use with IEEE1588. */
        pub timesync: u16,
        pub __bindgen_padding_0: [u32; 7usize],
    }
    /**
	 * 16-bit Reference counter.
	 * It should only be accessed using the following functions:
	 * rte_mbuf_refcnt_update(), rte_mbuf_refcnt_read(), and
	 * rte_mbuf_refcnt_set(). The functionality of these functions (atomic,
	 * or non-atomic) is controlled by the CONFIG_RTE_MBUF_REFCNT_ATOMIC
	 * config option.
	 */
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_1 {
        /**< Atomically accessed refcnt */
        pub refcnt_atomic: __BindgenUnionField<rte_atomic16_t>,
        /**< Non-atomically accessed refcnt */
        pub refcnt: __BindgenUnionField<u16>,
        pub bindgen_union_field: u16,
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_1() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_1>() , 2usize ,
                   concat ! (
                   "Size of: " , stringify ! ( rte_mbuf__bindgen_ty_1 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_1>() , 2usize
                    , concat ! (
                    "Alignment of " , stringify ! ( rte_mbuf__bindgen_ty_1 )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_1 ) ) .
                    refcnt_atomic as * const _ as usize } , 0usize , concat !
                    (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_1 ) , "::" , stringify ! (
                    refcnt_atomic ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_1 ) ) . refcnt
                    as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_1 ) , "::" , stringify ! ( refcnt )
                    ));
    }
    impl Clone for rte_mbuf__bindgen_ty_1 {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_2 {
        /**< L2/L3/L4 and tunnel information. */
        pub packet_type: __BindgenUnionField<u32>,
        pub __bindgen_anon_1: __BindgenUnionField<rte_mbuf__bindgen_ty_2__bindgen_ty_1>,
        pub bindgen_union_field: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
        pub _bitfield_1: [u8; 4usize],
        pub __bindgen_align: [u32; 0usize],
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_2__bindgen_ty_1() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>()
                   , 4usize , concat ! (
                   "Size of: " , stringify ! (
                   rte_mbuf__bindgen_ty_2__bindgen_ty_1 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_2__bindgen_ty_1>()
                    , 4usize , concat ! (
                    "Alignment of " , stringify ! (
                    rte_mbuf__bindgen_ty_2__bindgen_ty_1 ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
        fn clone(&self) -> Self { *self }
    }
    impl rte_mbuf__bindgen_ty_2__bindgen_ty_1 {
        #[inline]
        pub fn l2_type(&self) -> u32 {
            let mask = 15usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 0usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_l2_type(&mut self, val: u32) {
            let mask = 15usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 0usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn l3_type(&self) -> u32 {
            let mask = 240usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 4usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_l3_type(&mut self, val: u32) {
            let mask = 240usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 4usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn l4_type(&self) -> u32 {
            let mask = 3840usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 8usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_l4_type(&mut self, val: u32) {
            let mask = 3840usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 8usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn tun_type(&self) -> u32 {
            let mask = 61440usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 12usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_tun_type(&mut self, val: u32) {
            let mask = 61440usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 12usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn inner_l2_type(&self) -> u32 {
            let mask = 983040usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 16usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_inner_l2_type(&mut self, val: u32) {
            let mask = 983040usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 16usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn inner_l3_type(&self) -> u32 {
            let mask = 15728640usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 20usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_inner_l3_type(&mut self, val: u32) {
            let mask = 15728640usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 20usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn inner_l4_type(&self) -> u32 {
            let mask = 251658240usize as u32;
            let field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 24usize;
            unsafe { ::std::mem::transmute(val as u32) }
        }
        #[inline]
        pub fn set_inner_l4_type(&mut self, val: u32) {
            let mask = 251658240usize as u32;
            let val = val as u32 as u32;
            let mut field_val: u32 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 24usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_2() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_2>() , 4usize ,
                   concat ! (
                   "Size of: " , stringify ! ( rte_mbuf__bindgen_ty_2 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_2>() , 4usize
                    , concat ! (
                    "Alignment of " , stringify ! ( rte_mbuf__bindgen_ty_2 )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_2 ) ) .
                    packet_type as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_2 ) , "::" , stringify ! (
                    packet_type ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_2 {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_3 {
        /**< RSS hash result if RSS enabled */
        pub rss: __BindgenUnionField<u32>,
        /**< Filter identifier if FDIR enabled */
        pub fdir: __BindgenUnionField<rte_mbuf__bindgen_ty_3__bindgen_ty_1>,
        /**< Hierarchical scheduler */
        pub sched: __BindgenUnionField<rte_mbuf__bindgen_ty_3__bindgen_ty_2>,
        /**< User defined tags. See rte_distributor_process() */
        pub usr: __BindgenUnionField<u32>,
        pub bindgen_union_field: [u32; 2usize],
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1 {
        pub __bindgen_anon_1: rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1,
        pub hi: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
        pub __bindgen_anon_1: __BindgenUnionField<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>,
        pub lo: __BindgenUnionField<u32>,
        pub bindgen_union_field: u32,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
        pub hash: u16,
        pub id: u16,
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>()
                   , 4usize , concat ! (
                   "Size of: " , stringify ! (
                   rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
                   ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1>()
                    , 2usize , concat ! (
                    "Alignment of " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
                    ) ));
        assert_eq! (unsafe {
                    & (
                    * (
                    0 as * const
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
                    ) ) . hash as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
                    ) , "::" , stringify ! ( hash ) ));
        assert_eq! (unsafe {
                    & (
                    * (
                    0 as * const
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
                    ) ) . id as * const _ as usize } , 2usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1
                    ) , "::" , stringify ! ( id ) ));
    }
    impl Clone for
     rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1__bindgen_ty_1 {
        fn clone(&self) -> Self { *self }
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1>()
                   , 4usize , concat ! (
                   "Size of: " , stringify ! (
                   rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1>()
                    , 4usize , concat ! (
                    "Alignment of " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 ) ));
        assert_eq! (unsafe {
                    & (
                    * (
                    0 as * const
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 ) ) .
                    lo as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 ) ,
                    "::" , stringify ! ( lo ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_3__bindgen_ty_1__bindgen_ty_1 {
        fn clone(&self) -> Self { *self }
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_1() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>()
                   , 8usize , concat ! (
                   "Size of: " , stringify ! (
                   rte_mbuf__bindgen_ty_3__bindgen_ty_1 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_1>()
                    , 4usize , concat ! (
                    "Alignment of " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1 ) ));
        assert_eq! (unsafe {
                    & (
                    * ( 0 as * const rte_mbuf__bindgen_ty_3__bindgen_ty_1 ) )
                    . hi as * const _ as usize } , 4usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_1 ) , "::" , stringify
                    ! ( hi ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_3__bindgen_ty_1 {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_3__bindgen_ty_2 {
        pub lo: u32,
        pub hi: u32,
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_3__bindgen_ty_2() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>()
                   , 8usize , concat ! (
                   "Size of: " , stringify ! (
                   rte_mbuf__bindgen_ty_3__bindgen_ty_2 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_3__bindgen_ty_2>()
                    , 4usize , concat ! (
                    "Alignment of " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_2 ) ));
        assert_eq! (unsafe {
                    & (
                    * ( 0 as * const rte_mbuf__bindgen_ty_3__bindgen_ty_2 ) )
                    . lo as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_2 ) , "::" , stringify
                    ! ( lo ) ));
        assert_eq! (unsafe {
                    & (
                    * ( 0 as * const rte_mbuf__bindgen_ty_3__bindgen_ty_2 ) )
                    . hi as * const _ as usize } , 4usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3__bindgen_ty_2 ) , "::" , stringify
                    ! ( hi ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_3__bindgen_ty_2 {
        fn clone(&self) -> Self { *self }
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_3() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_3>() , 8usize ,
                   concat ! (
                   "Size of: " , stringify ! ( rte_mbuf__bindgen_ty_3 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_3>() , 4usize
                    , concat ! (
                    "Alignment of " , stringify ! ( rte_mbuf__bindgen_ty_3 )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_3 ) ) . rss as *
                    const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3 ) , "::" , stringify ! ( rss ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_3 ) ) . fdir as
                    * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3 ) , "::" , stringify ! ( fdir ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_3 ) ) . sched as
                    * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3 ) , "::" , stringify ! ( sched )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_3 ) ) . usr as *
                    const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_3 ) , "::" , stringify ! ( usr ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_3 {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_4 {
        /**< Can be used for external metadata */
        pub userdata: __BindgenUnionField<*mut ::std::os::raw::c_void>,
        /**< Allow 8-byte userdata on 32-bit */
        pub udata64: __BindgenUnionField<u64>,
        pub bindgen_union_field: u64,
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_4() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_4>() , 8usize ,
                   concat ! (
                   "Size of: " , stringify ! ( rte_mbuf__bindgen_ty_4 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_4>() , 8usize
                    , concat ! (
                    "Alignment of " , stringify ! ( rte_mbuf__bindgen_ty_4 )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_4 ) ) . userdata
                    as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_4 ) , "::" , stringify ! ( userdata )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_4 ) ) . udata64
                    as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_4 ) , "::" , stringify ! ( udata64 )
                    ));
    }
    impl Clone for rte_mbuf__bindgen_ty_4 {
        fn clone(&self) -> Self { *self }
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_5 {
        /**< combined for easy fetch */
        pub tx_offload: __BindgenUnionField<u64>,
        pub __bindgen_anon_1: __BindgenUnionField<rte_mbuf__bindgen_ty_5__bindgen_ty_1>,
        pub bindgen_union_field: u64,
    }
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
        pub _bitfield_1: [u16; 4usize],
        pub __bindgen_align: [u64; 0usize],
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_5__bindgen_ty_1() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>()
                   , 8usize , concat ! (
                   "Size of: " , stringify ! (
                   rte_mbuf__bindgen_ty_5__bindgen_ty_1 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_5__bindgen_ty_1>()
                    , 8usize , concat ! (
                    "Alignment of " , stringify ! (
                    rte_mbuf__bindgen_ty_5__bindgen_ty_1 ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
        fn clone(&self) -> Self { *self }
    }
    impl rte_mbuf__bindgen_ty_5__bindgen_ty_1 {
        #[inline]
        pub fn l2_len(&self) -> u64 {
            let mask = 127usize as u64;
            let field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 0usize;
            unsafe { ::std::mem::transmute(val as u64) }
        }
        #[inline]
        pub fn set_l2_len(&mut self, val: u64) {
            let mask = 127usize as u64;
            let val = val as u64 as u64;
            let mut field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 0usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn l3_len(&self) -> u64 {
            let mask = 65408usize as u64;
            let field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 7usize;
            unsafe { ::std::mem::transmute(val as u64) }
        }
        #[inline]
        pub fn set_l3_len(&mut self, val: u64) {
            let mask = 65408usize as u64;
            let val = val as u64 as u64;
            let mut field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 7usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn l4_len(&self) -> u64 {
            let mask = 16711680usize as u64;
            let field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 16usize;
            unsafe { ::std::mem::transmute(val as u64) }
        }
        #[inline]
        pub fn set_l4_len(&mut self, val: u64) {
            let mask = 16711680usize as u64;
            let val = val as u64 as u64;
            let mut field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 16usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn tso_segsz(&self) -> u64 {
            let mask = 1099494850560usize as u64;
            let field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 24usize;
            unsafe { ::std::mem::transmute(val as u64) }
        }
        #[inline]
        pub fn set_tso_segsz(&mut self, val: u64) {
            let mask = 1099494850560usize as u64;
            let val = val as u64 as u64;
            let mut field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 24usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn outer_l3_len(&self) -> u64 {
            let mask = 561850441793536usize as u64;
            let field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 40usize;
            unsafe { ::std::mem::transmute(val as u64) }
        }
        #[inline]
        pub fn set_outer_l3_len(&mut self, val: u64) {
            let mask = 561850441793536usize as u64;
            let val = val as u64 as u64;
            let mut field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 40usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
        #[inline]
        pub fn outer_l2_len(&self) -> u64 {
            let mask = 71494644084506624usize as u64;
            let field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            let val = (field_val & mask) >> 49usize;
            unsafe { ::std::mem::transmute(val as u64) }
        }
        #[inline]
        pub fn set_outer_l2_len(&mut self, val: u64) {
            let mask = 71494644084506624usize as u64;
            let val = val as u64 as u64;
            let mut field_val: u64 =
                unsafe { ::std::mem::transmute(self._bitfield_1) };
            field_val &= !mask;
            field_val |= (val << 49usize) & mask;
            self._bitfield_1 = unsafe { ::std::mem::transmute(field_val) };
        }
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf__bindgen_ty_5() {
        assert_eq!(::std::mem::size_of::<rte_mbuf__bindgen_ty_5>() , 8usize ,
                   concat ! (
                   "Size of: " , stringify ! ( rte_mbuf__bindgen_ty_5 ) ));
        assert_eq! (::std::mem::align_of::<rte_mbuf__bindgen_ty_5>() , 8usize
                    , concat ! (
                    "Alignment of " , stringify ! ( rte_mbuf__bindgen_ty_5 )
                    ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf__bindgen_ty_5 ) ) .
                    tx_offload as * const _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! (
                    rte_mbuf__bindgen_ty_5 ) , "::" , stringify ! ( tx_offload
                    ) ));
    }
    impl Clone for rte_mbuf__bindgen_ty_5 {
        fn clone(&self) -> Self { *self }
    }
    #[test]
    fn bindgen_test_layout_rte_mbuf() {
        assert_eq!(::std::mem::size_of::<rte_mbuf>() , 128usize , concat ! (
                   "Size of: " , stringify ! ( rte_mbuf ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . cacheline0 as * const
                    _ as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( cacheline0 ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . buf_addr as * const _
                    as usize } , 0usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( buf_addr ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . buf_physaddr as *
                    const _ as usize } , 8usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( buf_physaddr ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . buf_len as * const _
                    as usize } , 16usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( buf_len ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . rearm_data as * const
                    _ as usize } , 18usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( rearm_data ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . data_off as * const _
                    as usize } , 18usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( data_off ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . nb_segs as * const _
                    as usize } , 22usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( nb_segs ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . port as * const _ as
                    usize } , 23usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( port ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . ol_flags as * const _
                    as usize } , 24usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( ol_flags ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . rx_descriptor_fields1
                    as * const _ as usize } , 32usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( rx_descriptor_fields1 ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . pkt_len as * const _
                    as usize } , 36usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( pkt_len ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . data_len as * const _
                    as usize } , 40usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( data_len ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . vlan_tci as * const _
                    as usize } , 42usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( vlan_tci ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . hash as * const _ as
                    usize } , 44usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( hash ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . seqn as * const _ as
                    usize } , 52usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( seqn ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . vlan_tci_outer as *
                    const _ as usize } , 56usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( vlan_tci_outer ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . cacheline1 as * const
                    _ as usize } , 64usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( cacheline1 ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . pool as * const _ as
                    usize } , 72usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( pool ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . next as * const _ as
                    usize } , 80usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( next ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . priv_size as * const _
                    as usize } , 96usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( priv_size ) ));
        assert_eq! (unsafe {
                    & ( * ( 0 as * const rte_mbuf ) ) . timesync as * const _
                    as usize } , 98usize , concat ! (
                    "Alignment of field: " , stringify ! ( rte_mbuf ) , "::" ,
                    stringify ! ( timesync ) ));
    }
    impl Clone for rte_mbuf {
        fn clone(&self) -> Self { *self }
    }
    impl Default for rte_mbuf {
        fn default() -> Self { unsafe { ::std::mem::zeroed() } }
    }
    /**< Pool from which mbuf was allocated. */
    #[repr(C)]
    #[derive(Debug, Default, Copy)]
    pub struct rte_mempool {
        pub _address: u8,
    }
    impl Clone for rte_mempool {
        fn clone(&self) -> Self { *self }
    }
}
