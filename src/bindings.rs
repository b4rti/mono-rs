#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    deref_nullptr,
    improper_ctypes
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
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
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
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
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
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
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>, [T; 0]);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub const fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData, [])
    }
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self as *const _ as *const T
    }
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self as *mut _ as *mut T
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
pub const _STDINT_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 36;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const _THREAD_MUTEX_INTERNAL_H: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const MONO_ALLOCATOR_VTABLE_VERSION: u32 = 1;
pub const MONO_ZERO_LEN_ARRAY: u32 = 0;
pub const _STDIO_H: u32 = 1;
pub const __GNUC_VA_LIST: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const P_tmpdir: &[u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const _MONO_METADATA_LOADER_H_: u32 = 1;
pub const MONO_DECLSEC_ACTION_MIN: u32 = 1;
pub const MONO_DECLSEC_ACTION_MAX: u32 = 18;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    fn test_field___val() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__fsid_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__fsid_t),
                "::",
                stringify!(__val)
            )
        );
    }
    test_field___val();
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        ::std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    fn test_field_quot() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<div_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(div_t),
                "::",
                stringify!(quot)
            )
        );
    }
    test_field_quot();
    fn test_field_rem() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<div_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(div_t),
                "::",
                stringify!(rem)
            )
        );
    }
    test_field_rem();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    fn test_field_quot() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(ldiv_t),
                "::",
                stringify!(quot)
            )
        );
    }
    test_field_quot();
    fn test_field_rem() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<ldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(ldiv_t),
                "::",
                stringify!(rem)
            )
        );
    }
    test_field_rem();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    fn test_field_quot() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(lldiv_t),
                "::",
                stringify!(quot)
            )
        );
    }
    test_field_quot();
    fn test_field_rem() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<lldiv_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(lldiv_t),
                "::",
                stringify!(rem)
            )
        );
    }
    test_field_rem();
}
extern "C" {
    pub fn __ctype_get_mb_cur_max() -> size_t;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtold(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> u128;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type ssize_t = __ssize_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    fn test_field___val() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__sigset_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__sigset_t),
                "::",
                stringify!(__val)
            )
        );
    }
    test_field___val();
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    fn test_field_tv_sec() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<timeval>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(timeval),
                "::",
                stringify!(tv_sec)
            )
        );
    }
    test_field_tv_sec();
    fn test_field_tv_usec() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<timeval>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tv_usec) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(timeval),
                "::",
                stringify!(tv_usec)
            )
        );
    }
    test_field_tv_usec();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    fn test_field_tv_sec() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<timespec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(timespec),
                "::",
                stringify!(tv_sec)
            )
        );
    }
    test_field_tv_sec();
    fn test_field_tv_nsec() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<timespec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tv_nsec) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(timespec),
                "::",
                stringify!(tv_nsec)
            )
        );
    }
    test_field_tv_nsec();
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    fn test_field___fds_bits() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<fd_set>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__fds_bits) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(fd_set),
                "::",
                stringify!(__fds_bits)
            )
        );
    }
    test_field___fds_bits();
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __atomic_wide_counter {
    pub __value64: ::std::os::raw::c_ulonglong,
    pub __value32: __atomic_wide_counter__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __atomic_wide_counter__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___atomic_wide_counter__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__atomic_wide_counter__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__atomic_wide_counter__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__atomic_wide_counter__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__atomic_wide_counter__bindgen_ty_1)
        )
    );
    fn test_field___low() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<__atomic_wide_counter__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__low) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__atomic_wide_counter__bindgen_ty_1),
                "::",
                stringify!(__low)
            )
        );
    }
    test_field___low();
    fn test_field___high() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<__atomic_wide_counter__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__high) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__atomic_wide_counter__bindgen_ty_1),
                "::",
                stringify!(__high)
            )
        );
    }
    test_field___high();
}
#[test]
fn bindgen_test_layout___atomic_wide_counter() {
    assert_eq!(
        ::std::mem::size_of::<__atomic_wide_counter>(),
        8usize,
        concat!("Size of: ", stringify!(__atomic_wide_counter))
    );
    assert_eq!(
        ::std::mem::align_of::<__atomic_wide_counter>(),
        8usize,
        concat!("Alignment of ", stringify!(__atomic_wide_counter))
    );
    fn test_field___value64() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__atomic_wide_counter>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__value64) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__atomic_wide_counter),
                "::",
                stringify!(__value64)
            )
        );
    }
    test_field___value64();
    fn test_field___value32() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__atomic_wide_counter>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__value32) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__atomic_wide_counter),
                "::",
                stringify!(__value32)
            )
        );
    }
    test_field___value32();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    fn test_field___prev() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_internal_list>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__prev) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_internal_list),
                "::",
                stringify!(__prev)
            )
        );
    }
    test_field___prev();
    fn test_field___next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_internal_list>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_internal_list),
                "::",
                stringify!(__next)
            )
        );
    }
    test_field___next();
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
#[test]
fn bindgen_test_layout___pthread_internal_slist() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_slist))
    );
    fn test_field___next() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_internal_slist>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_internal_slist),
                "::",
                stringify!(__next)
            )
        );
    }
    test_field___next();
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    fn test_field___lock() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__lock) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__lock)
            )
        );
    }
    test_field___lock();
    fn test_field___count() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__count) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__count)
            )
        );
    }
    test_field___count();
    fn test_field___owner() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__owner) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__owner)
            )
        );
    }
    test_field___owner();
    fn test_field___nusers() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__nusers) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__nusers)
            )
        );
    }
    test_field___nusers();
    fn test_field___kind() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__kind) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__kind)
            )
        );
    }
    test_field___kind();
    fn test_field___spins() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__spins) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__spins)
            )
        );
    }
    test_field___spins();
    fn test_field___elision() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__elision) as usize - ptr as usize
            },
            22usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__elision)
            )
        );
    }
    test_field___elision();
    fn test_field___list() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_mutex_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__list) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_mutex_s),
                "::",
                stringify!(__list)
            )
        );
    }
    test_field___list();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    fn test_field___readers() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__readers) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__readers)
            )
        );
    }
    test_field___readers();
    fn test_field___writers() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__writers) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__writers)
            )
        );
    }
    test_field___writers();
    fn test_field___wrphase_futex() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wrphase_futex) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__wrphase_futex)
            )
        );
    }
    test_field___wrphase_futex();
    fn test_field___writers_futex() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__writers_futex) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__writers_futex)
            )
        );
    }
    test_field___writers_futex();
    fn test_field___pad3() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad3) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__pad3)
            )
        );
    }
    test_field___pad3();
    fn test_field___pad4() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad4) as usize - ptr as usize
            },
            20usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__pad4)
            )
        );
    }
    test_field___pad4();
    fn test_field___cur_writer() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__cur_writer) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__cur_writer)
            )
        );
    }
    test_field___cur_writer();
    fn test_field___shared() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__shared) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__shared)
            )
        );
    }
    test_field___shared();
    fn test_field___rwelision() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__rwelision) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__rwelision)
            )
        );
    }
    test_field___rwelision();
    fn test_field___pad1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad1) as usize - ptr as usize
            },
            33usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__pad1)
            )
        );
    }
    test_field___pad1();
    fn test_field___pad2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad2) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__pad2)
            )
        );
    }
    test_field___pad2();
    fn test_field___flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_rwlock_arch_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__flags) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_rwlock_arch_t),
                "::",
                stringify!(__flags)
            )
        );
    }
    test_field___flags();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    fn test_field___wseq() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wseq) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__wseq)
            )
        );
    }
    test_field___wseq();
    fn test_field___g1_start() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__g1_start) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__g1_start)
            )
        );
    }
    test_field___g1_start();
    fn test_field___g_refs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__g_refs) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__g_refs)
            )
        );
    }
    test_field___g_refs();
    fn test_field___g_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__g_size) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__g_size)
            )
        );
    }
    test_field___g_size();
    fn test_field___g1_orig_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__g1_orig_size) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__g1_orig_size)
            )
        );
    }
    test_field___g1_orig_size();
    fn test_field___wrefs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wrefs) as usize - ptr as usize
            },
            36usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__wrefs)
            )
        );
    }
    test_field___wrefs();
    fn test_field___g_signals() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__pthread_cond_s>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__g_signals) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(__pthread_cond_s),
                "::",
                stringify!(__g_signals)
            )
        );
    }
    test_field___g_signals();
}
pub type __tss_t = ::std::os::raw::c_uint;
pub type __thrd_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __once_flag {
    pub __data: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___once_flag() {
    assert_eq!(
        ::std::mem::size_of::<__once_flag>(),
        4usize,
        concat!("Size of: ", stringify!(__once_flag))
    );
    assert_eq!(
        ::std::mem::align_of::<__once_flag>(),
        4usize,
        concat!("Alignment of ", stringify!(__once_flag))
    );
    fn test_field___data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__once_flag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__once_flag),
                "::",
                stringify!(__data)
            )
        );
    }
    test_field___data();
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_mutexattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_mutexattr_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_mutexattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_mutexattr_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_condattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_condattr_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_condattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_condattr_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_attr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_attr_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_attr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_attr_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    fn test_field___data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_mutex_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_mutex_t),
                "::",
                stringify!(__data)
            )
        );
    }
    test_field___data();
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_mutex_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_mutex_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_mutex_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_mutex_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    fn test_field___data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_cond_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_cond_t),
                "::",
                stringify!(__data)
            )
        );
    }
    test_field___data();
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_cond_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_cond_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_cond_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_cond_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    fn test_field___data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_rwlock_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_rwlock_t),
                "::",
                stringify!(__data)
            )
        );
    }
    test_field___data();
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_rwlock_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_rwlock_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_rwlock_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_rwlock_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_rwlockattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_rwlockattr_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_rwlockattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_rwlockattr_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_barrier_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_barrier_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_barrier_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_barrier_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    assert_eq!(
        ::std::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    fn test_field___size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_barrierattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_barrierattr_t),
                "::",
                stringify!(__size)
            )
        );
    }
    test_field___size();
    fn test_field___align() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<pthread_barrierattr_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(pthread_barrierattr_t),
                "::",
                stringify!(__align)
            )
        );
    }
    test_field___align();
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    assert_eq!(
        ::std::mem::size_of::<random_data>(),
        48usize,
        concat!("Size of: ", stringify!(random_data))
    );
    assert_eq!(
        ::std::mem::align_of::<random_data>(),
        8usize,
        concat!("Alignment of ", stringify!(random_data))
    );
    fn test_field_fptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fptr) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(fptr)
            )
        );
    }
    test_field_fptr();
    fn test_field_rptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rptr) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(rptr)
            )
        );
    }
    test_field_rptr();
    fn test_field_state() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(state)
            )
        );
    }
    test_field_state();
    fn test_field_rand_type() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rand_type) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(rand_type)
            )
        );
    }
    test_field_rand_type();
    fn test_field_rand_deg() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rand_deg) as usize - ptr as usize
            },
            28usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(rand_deg)
            )
        );
    }
    test_field_rand_deg();
    fn test_field_rand_sep() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rand_sep) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(rand_sep)
            )
        );
    }
    test_field_rand_sep();
    fn test_field_end_ptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<random_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).end_ptr) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(random_data),
                "::",
                stringify!(end_ptr)
            )
        );
    }
    test_field_end_ptr();
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: size_t,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    assert_eq!(
        ::std::mem::size_of::<drand48_data>(),
        24usize,
        concat!("Size of: ", stringify!(drand48_data))
    );
    assert_eq!(
        ::std::mem::align_of::<drand48_data>(),
        8usize,
        concat!("Alignment of ", stringify!(drand48_data))
    );
    fn test_field___x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<drand48_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__x) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(drand48_data),
                "::",
                stringify!(__x)
            )
        );
    }
    test_field___x();
    fn test_field___old_x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<drand48_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__old_x) as usize - ptr as usize
            },
            6usize,
            concat!(
                "Offset of field: ",
                stringify!(drand48_data),
                "::",
                stringify!(__old_x)
            )
        );
    }
    test_field___old_x();
    fn test_field___c() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<drand48_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__c) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(drand48_data),
                "::",
                stringify!(__c)
            )
        );
    }
    test_field___c();
    fn test_field___init() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<drand48_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__init) as usize - ptr as usize
            },
            14usize,
            concat!(
                "Offset of field: ",
                stringify!(drand48_data),
                "::",
                stringify!(__init)
            )
        );
    }
    test_field___init();
    fn test_field___a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<drand48_data>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__a) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(drand48_data),
                "::",
                stringify!(__a)
            )
        );
    }
    test_field___a();
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn arc4random() -> __uint32_t;
}
extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __size: size_t);
}
extern "C" {
    pub fn arc4random_uniform(__upper_bound: __uint32_t) -> __uint32_t;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __nmemb: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn alloca(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(
        __alignment: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qecvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qfcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn qgcvt(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qecvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qfcvt_r(
        __value: u128,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(
        __pwcs: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn wcstombs(
        __s: *mut ::std::os::raw::c_char,
        __pwcs: *const wchar_t,
        __n: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
pub type mono_bool = i32;
pub type mono_byte = u8;
pub type MonoBoolean = mono_byte;
pub type mono_unichar2 = u16;
pub type mono_unichar4 = u32;
pub type MonoFunc = ::std::option::Option<
    unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, user_data: *mut ::std::os::raw::c_void),
>;
pub type MonoHFunc = ::std::option::Option<
    unsafe extern "C" fn(
        key: *mut ::std::os::raw::c_void,
        value: *mut ::std::os::raw::c_void,
        user_data: *mut ::std::os::raw::c_void,
    ),
>;
extern "C" {
    pub fn mono_free(arg1: *mut ::std::os::raw::c_void);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoAllocatorVTable {
    pub version: ::std::os::raw::c_int,
    pub malloc:
        ::std::option::Option<unsafe extern "C" fn(size: size_t) -> *mut ::std::os::raw::c_void>,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            mem: *mut ::std::os::raw::c_void,
            count: size_t,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub free: ::std::option::Option<unsafe extern "C" fn(mem: *mut ::std::os::raw::c_void)>,
    pub calloc: ::std::option::Option<
        unsafe extern "C" fn(count: size_t, size: size_t) -> *mut ::std::os::raw::c_void,
    >,
}
#[test]
fn bindgen_test_layout_MonoAllocatorVTable() {
    assert_eq!(
        ::std::mem::size_of::<MonoAllocatorVTable>(),
        40usize,
        concat!("Size of: ", stringify!(MonoAllocatorVTable))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoAllocatorVTable>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoAllocatorVTable))
    );
    fn test_field_version() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoAllocatorVTable>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).version) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoAllocatorVTable),
                "::",
                stringify!(version)
            )
        );
    }
    test_field_version();
    fn test_field_malloc() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoAllocatorVTable>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).malloc) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoAllocatorVTable),
                "::",
                stringify!(malloc)
            )
        );
    }
    test_field_malloc();
    fn test_field_realloc() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoAllocatorVTable>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).realloc) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoAllocatorVTable),
                "::",
                stringify!(realloc)
            )
        );
    }
    test_field_realloc();
    fn test_field_free() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoAllocatorVTable>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).free) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoAllocatorVTable),
                "::",
                stringify!(free)
            )
        );
    }
    test_field_free();
    fn test_field_calloc() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoAllocatorVTable>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).calloc) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoAllocatorVTable),
                "::",
                stringify!(calloc)
            )
        );
    }
    test_field_calloc();
}
extern "C" {
    pub fn mono_set_allocator_vtable(vtable: *mut MonoAllocatorVTable) -> mono_bool;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoDomain {
    _unused: [u8; 0],
}
pub type MonoDomain = _MonoDomain;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoJitInfo {
    _unused: [u8; 0],
}
pub type MonoJitInfo = _MonoJitInfo;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoClass {
    _unused: [u8; 0],
}
pub type MonoClass = _MonoClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoImage {
    _unused: [u8; 0],
}
pub type MonoImage = _MonoImage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoMethod {
    _unused: [u8; 0],
}
pub type MonoMethod = _MonoMethod;
pub type MonoObject = _MonoObject;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoException {
    _unused: [u8; 0],
}
pub type MonoException = _MonoException;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionAssembly {
    _unused: [u8; 0],
}
pub type MonoReflectionAssembly = _MonoReflectionAssembly;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionTypeBuilder {
    _unused: [u8; 0],
}
pub type MonoReflectionTypeBuilder = _MonoReflectionTypeBuilder;
pub const MonoTypeEnum_MONO_TYPE_END: MonoTypeEnum = 0;
pub const MonoTypeEnum_MONO_TYPE_VOID: MonoTypeEnum = 1;
pub const MonoTypeEnum_MONO_TYPE_BOOLEAN: MonoTypeEnum = 2;
pub const MonoTypeEnum_MONO_TYPE_CHAR: MonoTypeEnum = 3;
pub const MonoTypeEnum_MONO_TYPE_I1: MonoTypeEnum = 4;
pub const MonoTypeEnum_MONO_TYPE_U1: MonoTypeEnum = 5;
pub const MonoTypeEnum_MONO_TYPE_I2: MonoTypeEnum = 6;
pub const MonoTypeEnum_MONO_TYPE_U2: MonoTypeEnum = 7;
pub const MonoTypeEnum_MONO_TYPE_I4: MonoTypeEnum = 8;
pub const MonoTypeEnum_MONO_TYPE_U4: MonoTypeEnum = 9;
pub const MonoTypeEnum_MONO_TYPE_I8: MonoTypeEnum = 10;
pub const MonoTypeEnum_MONO_TYPE_U8: MonoTypeEnum = 11;
pub const MonoTypeEnum_MONO_TYPE_R4: MonoTypeEnum = 12;
pub const MonoTypeEnum_MONO_TYPE_R8: MonoTypeEnum = 13;
pub const MonoTypeEnum_MONO_TYPE_STRING: MonoTypeEnum = 14;
pub const MonoTypeEnum_MONO_TYPE_PTR: MonoTypeEnum = 15;
pub const MonoTypeEnum_MONO_TYPE_BYREF: MonoTypeEnum = 16;
pub const MonoTypeEnum_MONO_TYPE_VALUETYPE: MonoTypeEnum = 17;
pub const MonoTypeEnum_MONO_TYPE_CLASS: MonoTypeEnum = 18;
pub const MonoTypeEnum_MONO_TYPE_VAR: MonoTypeEnum = 19;
pub const MonoTypeEnum_MONO_TYPE_ARRAY: MonoTypeEnum = 20;
pub const MonoTypeEnum_MONO_TYPE_GENERICINST: MonoTypeEnum = 21;
pub const MonoTypeEnum_MONO_TYPE_TYPEDBYREF: MonoTypeEnum = 22;
pub const MonoTypeEnum_MONO_TYPE_I: MonoTypeEnum = 24;
pub const MonoTypeEnum_MONO_TYPE_U: MonoTypeEnum = 25;
pub const MonoTypeEnum_MONO_TYPE_FNPTR: MonoTypeEnum = 27;
pub const MonoTypeEnum_MONO_TYPE_OBJECT: MonoTypeEnum = 28;
pub const MonoTypeEnum_MONO_TYPE_SZARRAY: MonoTypeEnum = 29;
pub const MonoTypeEnum_MONO_TYPE_MVAR: MonoTypeEnum = 30;
pub const MonoTypeEnum_MONO_TYPE_CMOD_REQD: MonoTypeEnum = 31;
pub const MonoTypeEnum_MONO_TYPE_CMOD_OPT: MonoTypeEnum = 32;
pub const MonoTypeEnum_MONO_TYPE_INTERNAL: MonoTypeEnum = 33;
pub const MonoTypeEnum_MONO_TYPE_MODIFIER: MonoTypeEnum = 64;
pub const MonoTypeEnum_MONO_TYPE_SENTINEL: MonoTypeEnum = 65;
pub const MonoTypeEnum_MONO_TYPE_PINNED: MonoTypeEnum = 69;
pub const MonoTypeEnum_MONO_TYPE_ENUM: MonoTypeEnum = 85;
pub type MonoTypeEnum = ::std::os::raw::c_uint;
pub const MonoMetaTableEnum_MONO_TABLE_MODULE: MonoMetaTableEnum = 0;
pub const MonoMetaTableEnum_MONO_TABLE_TYPEREF: MonoMetaTableEnum = 1;
pub const MonoMetaTableEnum_MONO_TABLE_TYPEDEF: MonoMetaTableEnum = 2;
pub const MonoMetaTableEnum_MONO_TABLE_FIELD_POINTER: MonoMetaTableEnum = 3;
pub const MonoMetaTableEnum_MONO_TABLE_FIELD: MonoMetaTableEnum = 4;
pub const MonoMetaTableEnum_MONO_TABLE_METHOD_POINTER: MonoMetaTableEnum = 5;
pub const MonoMetaTableEnum_MONO_TABLE_METHOD: MonoMetaTableEnum = 6;
pub const MonoMetaTableEnum_MONO_TABLE_PARAM_POINTER: MonoMetaTableEnum = 7;
pub const MonoMetaTableEnum_MONO_TABLE_PARAM: MonoMetaTableEnum = 8;
pub const MonoMetaTableEnum_MONO_TABLE_INTERFACEIMPL: MonoMetaTableEnum = 9;
pub const MonoMetaTableEnum_MONO_TABLE_MEMBERREF: MonoMetaTableEnum = 10;
pub const MonoMetaTableEnum_MONO_TABLE_CONSTANT: MonoMetaTableEnum = 11;
pub const MonoMetaTableEnum_MONO_TABLE_CUSTOMATTRIBUTE: MonoMetaTableEnum = 12;
pub const MonoMetaTableEnum_MONO_TABLE_FIELDMARSHAL: MonoMetaTableEnum = 13;
pub const MonoMetaTableEnum_MONO_TABLE_DECLSECURITY: MonoMetaTableEnum = 14;
pub const MonoMetaTableEnum_MONO_TABLE_CLASSLAYOUT: MonoMetaTableEnum = 15;
pub const MonoMetaTableEnum_MONO_TABLE_FIELDLAYOUT: MonoMetaTableEnum = 16;
pub const MonoMetaTableEnum_MONO_TABLE_STANDALONESIG: MonoMetaTableEnum = 17;
pub const MonoMetaTableEnum_MONO_TABLE_EVENTMAP: MonoMetaTableEnum = 18;
pub const MonoMetaTableEnum_MONO_TABLE_EVENT_POINTER: MonoMetaTableEnum = 19;
pub const MonoMetaTableEnum_MONO_TABLE_EVENT: MonoMetaTableEnum = 20;
pub const MonoMetaTableEnum_MONO_TABLE_PROPERTYMAP: MonoMetaTableEnum = 21;
pub const MonoMetaTableEnum_MONO_TABLE_PROPERTY_POINTER: MonoMetaTableEnum = 22;
pub const MonoMetaTableEnum_MONO_TABLE_PROPERTY: MonoMetaTableEnum = 23;
pub const MonoMetaTableEnum_MONO_TABLE_METHODSEMANTICS: MonoMetaTableEnum = 24;
pub const MonoMetaTableEnum_MONO_TABLE_METHODIMPL: MonoMetaTableEnum = 25;
pub const MonoMetaTableEnum_MONO_TABLE_MODULEREF: MonoMetaTableEnum = 26;
pub const MonoMetaTableEnum_MONO_TABLE_TYPESPEC: MonoMetaTableEnum = 27;
pub const MonoMetaTableEnum_MONO_TABLE_IMPLMAP: MonoMetaTableEnum = 28;
pub const MonoMetaTableEnum_MONO_TABLE_FIELDRVA: MonoMetaTableEnum = 29;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED6: MonoMetaTableEnum = 30;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED7: MonoMetaTableEnum = 31;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLY: MonoMetaTableEnum = 32;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYPROCESSOR: MonoMetaTableEnum = 33;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYOS: MonoMetaTableEnum = 34;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREF: MonoMetaTableEnum = 35;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFPROCESSOR: MonoMetaTableEnum = 36;
pub const MonoMetaTableEnum_MONO_TABLE_ASSEMBLYREFOS: MonoMetaTableEnum = 37;
pub const MonoMetaTableEnum_MONO_TABLE_FILE: MonoMetaTableEnum = 38;
pub const MonoMetaTableEnum_MONO_TABLE_EXPORTEDTYPE: MonoMetaTableEnum = 39;
pub const MonoMetaTableEnum_MONO_TABLE_MANIFESTRESOURCE: MonoMetaTableEnum = 40;
pub const MonoMetaTableEnum_MONO_TABLE_NESTEDCLASS: MonoMetaTableEnum = 41;
pub const MonoMetaTableEnum_MONO_TABLE_GENERICPARAM: MonoMetaTableEnum = 42;
pub const MonoMetaTableEnum_MONO_TABLE_METHODSPEC: MonoMetaTableEnum = 43;
pub const MonoMetaTableEnum_MONO_TABLE_GENERICPARAMCONSTRAINT: MonoMetaTableEnum = 44;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED8: MonoMetaTableEnum = 45;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED9: MonoMetaTableEnum = 46;
pub const MonoMetaTableEnum_MONO_TABLE_UNUSED10: MonoMetaTableEnum = 47;
pub const MonoMetaTableEnum_MONO_TABLE_DOCUMENT: MonoMetaTableEnum = 48;
pub const MonoMetaTableEnum_MONO_TABLE_METHODBODY: MonoMetaTableEnum = 49;
pub const MonoMetaTableEnum_MONO_TABLE_LOCALSCOPE: MonoMetaTableEnum = 50;
pub const MonoMetaTableEnum_MONO_TABLE_LOCALVARIABLE: MonoMetaTableEnum = 51;
pub const MonoMetaTableEnum_MONO_TABLE_LOCALCONSTANT: MonoMetaTableEnum = 52;
pub const MonoMetaTableEnum_MONO_TABLE_IMPORTSCOPE: MonoMetaTableEnum = 53;
pub const MonoMetaTableEnum_MONO_TABLE_STATEMACHINEMETHOD: MonoMetaTableEnum = 54;
pub const MonoMetaTableEnum_MONO_TABLE_CUSTOMDEBUGINFORMATION: MonoMetaTableEnum = 55;
pub type MonoMetaTableEnum = ::std::os::raw::c_uint;
pub const MONO_ASSEMBLY_HASH_ALG: _bindgen_ty_1 = 0;
pub const MONO_ASSEMBLY_MAJOR_VERSION: _bindgen_ty_1 = 1;
pub const MONO_ASSEMBLY_MINOR_VERSION: _bindgen_ty_1 = 2;
pub const MONO_ASSEMBLY_BUILD_NUMBER: _bindgen_ty_1 = 3;
pub const MONO_ASSEMBLY_REV_NUMBER: _bindgen_ty_1 = 4;
pub const MONO_ASSEMBLY_FLAGS: _bindgen_ty_1 = 5;
pub const MONO_ASSEMBLY_PUBLIC_KEY: _bindgen_ty_1 = 6;
pub const MONO_ASSEMBLY_NAME: _bindgen_ty_1 = 7;
pub const MONO_ASSEMBLY_CULTURE: _bindgen_ty_1 = 8;
pub const MONO_ASSEMBLY_SIZE: _bindgen_ty_1 = 9;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
pub const MONO_ASSEMBLYOS_PLATFORM: _bindgen_ty_2 = 0;
pub const MONO_ASSEMBLYOS_MAJOR_VERSION: _bindgen_ty_2 = 1;
pub const MONO_ASSEMBLYOS_MINOR_VERSION: _bindgen_ty_2 = 2;
pub const MONO_ASSEMBLYOS_SIZE: _bindgen_ty_2 = 3;
pub type _bindgen_ty_2 = ::std::os::raw::c_uint;
pub const MONO_ASSEMBLY_PROCESSOR: _bindgen_ty_3 = 0;
pub const MONO_ASSEMBLY_PROCESSOR_SIZE: _bindgen_ty_3 = 1;
pub type _bindgen_ty_3 = ::std::os::raw::c_uint;
pub const MONO_ASSEMBLYREF_MAJOR_VERSION: _bindgen_ty_4 = 0;
pub const MONO_ASSEMBLYREF_MINOR_VERSION: _bindgen_ty_4 = 1;
pub const MONO_ASSEMBLYREF_BUILD_NUMBER: _bindgen_ty_4 = 2;
pub const MONO_ASSEMBLYREF_REV_NUMBER: _bindgen_ty_4 = 3;
pub const MONO_ASSEMBLYREF_FLAGS: _bindgen_ty_4 = 4;
pub const MONO_ASSEMBLYREF_PUBLIC_KEY: _bindgen_ty_4 = 5;
pub const MONO_ASSEMBLYREF_NAME: _bindgen_ty_4 = 6;
pub const MONO_ASSEMBLYREF_CULTURE: _bindgen_ty_4 = 7;
pub const MONO_ASSEMBLYREF_HASH_VALUE: _bindgen_ty_4 = 8;
pub const MONO_ASSEMBLYREF_SIZE: _bindgen_ty_4 = 9;
pub type _bindgen_ty_4 = ::std::os::raw::c_uint;
pub const MONO_ASSEMBLYREFOS_PLATFORM: _bindgen_ty_5 = 0;
pub const MONO_ASSEMBLYREFOS_MAJOR_VERSION: _bindgen_ty_5 = 1;
pub const MONO_ASSEMBLYREFOS_MINOR_VERSION: _bindgen_ty_5 = 2;
pub const MONO_ASSEMBLYREFOS_ASSEMBLYREF: _bindgen_ty_5 = 3;
pub const MONO_ASSEMBLYREFOS_SIZE: _bindgen_ty_5 = 4;
pub type _bindgen_ty_5 = ::std::os::raw::c_uint;
pub const MONO_ASSEMBLYREFPROC_PROCESSOR: _bindgen_ty_6 = 0;
pub const MONO_ASSEMBLYREFPROC_ASSEMBLYREF: _bindgen_ty_6 = 1;
pub const MONO_ASSEMBLYREFPROC_SIZE: _bindgen_ty_6 = 2;
pub type _bindgen_ty_6 = ::std::os::raw::c_uint;
pub const MONO_CLASS_LAYOUT_PACKING_SIZE: _bindgen_ty_7 = 0;
pub const MONO_CLASS_LAYOUT_CLASS_SIZE: _bindgen_ty_7 = 1;
pub const MONO_CLASS_LAYOUT_PARENT: _bindgen_ty_7 = 2;
pub const MONO_CLASS_LAYOUT_SIZE: _bindgen_ty_7 = 3;
pub type _bindgen_ty_7 = ::std::os::raw::c_uint;
pub const MONO_CONSTANT_TYPE: _bindgen_ty_8 = 0;
pub const MONO_CONSTANT_PADDING: _bindgen_ty_8 = 1;
pub const MONO_CONSTANT_PARENT: _bindgen_ty_8 = 2;
pub const MONO_CONSTANT_VALUE: _bindgen_ty_8 = 3;
pub const MONO_CONSTANT_SIZE: _bindgen_ty_8 = 4;
pub type _bindgen_ty_8 = ::std::os::raw::c_uint;
pub const MONO_CUSTOM_ATTR_PARENT: _bindgen_ty_9 = 0;
pub const MONO_CUSTOM_ATTR_TYPE: _bindgen_ty_9 = 1;
pub const MONO_CUSTOM_ATTR_VALUE: _bindgen_ty_9 = 2;
pub const MONO_CUSTOM_ATTR_SIZE: _bindgen_ty_9 = 3;
pub type _bindgen_ty_9 = ::std::os::raw::c_uint;
pub const MONO_DECL_SECURITY_ACTION: _bindgen_ty_10 = 0;
pub const MONO_DECL_SECURITY_PARENT: _bindgen_ty_10 = 1;
pub const MONO_DECL_SECURITY_PERMISSIONSET: _bindgen_ty_10 = 2;
pub const MONO_DECL_SECURITY_SIZE: _bindgen_ty_10 = 3;
pub type _bindgen_ty_10 = ::std::os::raw::c_uint;
pub const MONO_EVENT_MAP_PARENT: _bindgen_ty_11 = 0;
pub const MONO_EVENT_MAP_EVENTLIST: _bindgen_ty_11 = 1;
pub const MONO_EVENT_MAP_SIZE: _bindgen_ty_11 = 2;
pub type _bindgen_ty_11 = ::std::os::raw::c_uint;
pub const MONO_EVENT_FLAGS: _bindgen_ty_12 = 0;
pub const MONO_EVENT_NAME: _bindgen_ty_12 = 1;
pub const MONO_EVENT_TYPE: _bindgen_ty_12 = 2;
pub const MONO_EVENT_SIZE: _bindgen_ty_12 = 3;
pub type _bindgen_ty_12 = ::std::os::raw::c_uint;
pub const MONO_EVENT_POINTER_EVENT: _bindgen_ty_13 = 0;
pub const MONO_EVENT_POINTER_SIZE: _bindgen_ty_13 = 1;
pub type _bindgen_ty_13 = ::std::os::raw::c_uint;
pub const MONO_EXP_TYPE_FLAGS: _bindgen_ty_14 = 0;
pub const MONO_EXP_TYPE_TYPEDEF: _bindgen_ty_14 = 1;
pub const MONO_EXP_TYPE_NAME: _bindgen_ty_14 = 2;
pub const MONO_EXP_TYPE_NAMESPACE: _bindgen_ty_14 = 3;
pub const MONO_EXP_TYPE_IMPLEMENTATION: _bindgen_ty_14 = 4;
pub const MONO_EXP_TYPE_SIZE: _bindgen_ty_14 = 5;
pub type _bindgen_ty_14 = ::std::os::raw::c_uint;
pub const MONO_FIELD_FLAGS: _bindgen_ty_15 = 0;
pub const MONO_FIELD_NAME: _bindgen_ty_15 = 1;
pub const MONO_FIELD_SIGNATURE: _bindgen_ty_15 = 2;
pub const MONO_FIELD_SIZE: _bindgen_ty_15 = 3;
pub type _bindgen_ty_15 = ::std::os::raw::c_uint;
pub const MONO_FIELD_LAYOUT_OFFSET: _bindgen_ty_16 = 0;
pub const MONO_FIELD_LAYOUT_FIELD: _bindgen_ty_16 = 1;
pub const MONO_FIELD_LAYOUT_SIZE: _bindgen_ty_16 = 2;
pub type _bindgen_ty_16 = ::std::os::raw::c_uint;
pub const MONO_FIELD_MARSHAL_PARENT: _bindgen_ty_17 = 0;
pub const MONO_FIELD_MARSHAL_NATIVE_TYPE: _bindgen_ty_17 = 1;
pub const MONO_FIELD_MARSHAL_SIZE: _bindgen_ty_17 = 2;
pub type _bindgen_ty_17 = ::std::os::raw::c_uint;
pub const MONO_FIELD_POINTER_FIELD: _bindgen_ty_18 = 0;
pub const MONO_FIELD_POINTER_SIZE: _bindgen_ty_18 = 1;
pub type _bindgen_ty_18 = ::std::os::raw::c_uint;
pub const MONO_FIELD_RVA_RVA: _bindgen_ty_19 = 0;
pub const MONO_FIELD_RVA_FIELD: _bindgen_ty_19 = 1;
pub const MONO_FIELD_RVA_SIZE: _bindgen_ty_19 = 2;
pub type _bindgen_ty_19 = ::std::os::raw::c_uint;
pub const MONO_FILE_FLAGS: _bindgen_ty_20 = 0;
pub const MONO_FILE_NAME: _bindgen_ty_20 = 1;
pub const MONO_FILE_HASH_VALUE: _bindgen_ty_20 = 2;
pub const MONO_FILE_SIZE: _bindgen_ty_20 = 3;
pub type _bindgen_ty_20 = ::std::os::raw::c_uint;
pub const MONO_IMPLMAP_FLAGS: _bindgen_ty_21 = 0;
pub const MONO_IMPLMAP_MEMBER: _bindgen_ty_21 = 1;
pub const MONO_IMPLMAP_NAME: _bindgen_ty_21 = 2;
pub const MONO_IMPLMAP_SCOPE: _bindgen_ty_21 = 3;
pub const MONO_IMPLMAP_SIZE: _bindgen_ty_21 = 4;
pub type _bindgen_ty_21 = ::std::os::raw::c_uint;
pub const MONO_INTERFACEIMPL_CLASS: _bindgen_ty_22 = 0;
pub const MONO_INTERFACEIMPL_INTERFACE: _bindgen_ty_22 = 1;
pub const MONO_INTERFACEIMPL_SIZE: _bindgen_ty_22 = 2;
pub type _bindgen_ty_22 = ::std::os::raw::c_uint;
pub const MONO_MANIFEST_OFFSET: _bindgen_ty_23 = 0;
pub const MONO_MANIFEST_FLAGS: _bindgen_ty_23 = 1;
pub const MONO_MANIFEST_NAME: _bindgen_ty_23 = 2;
pub const MONO_MANIFEST_IMPLEMENTATION: _bindgen_ty_23 = 3;
pub const MONO_MANIFEST_SIZE: _bindgen_ty_23 = 4;
pub type _bindgen_ty_23 = ::std::os::raw::c_uint;
pub const MONO_MEMBERREF_CLASS: _bindgen_ty_24 = 0;
pub const MONO_MEMBERREF_NAME: _bindgen_ty_24 = 1;
pub const MONO_MEMBERREF_SIGNATURE: _bindgen_ty_24 = 2;
pub const MONO_MEMBERREF_SIZE: _bindgen_ty_24 = 3;
pub type _bindgen_ty_24 = ::std::os::raw::c_uint;
pub const MONO_METHOD_RVA: _bindgen_ty_25 = 0;
pub const MONO_METHOD_IMPLFLAGS: _bindgen_ty_25 = 1;
pub const MONO_METHOD_FLAGS: _bindgen_ty_25 = 2;
pub const MONO_METHOD_NAME: _bindgen_ty_25 = 3;
pub const MONO_METHOD_SIGNATURE: _bindgen_ty_25 = 4;
pub const MONO_METHOD_PARAMLIST: _bindgen_ty_25 = 5;
pub const MONO_METHOD_SIZE: _bindgen_ty_25 = 6;
pub type _bindgen_ty_25 = ::std::os::raw::c_uint;
pub const MONO_METHODIMPL_CLASS: _bindgen_ty_26 = 0;
pub const MONO_METHODIMPL_BODY: _bindgen_ty_26 = 1;
pub const MONO_METHODIMPL_DECLARATION: _bindgen_ty_26 = 2;
pub const MONO_METHODIMPL_SIZE: _bindgen_ty_26 = 3;
pub type _bindgen_ty_26 = ::std::os::raw::c_uint;
pub const MONO_METHOD_POINTER_METHOD: _bindgen_ty_27 = 0;
pub const MONO_METHOD_POINTER_SIZE: _bindgen_ty_27 = 1;
pub type _bindgen_ty_27 = ::std::os::raw::c_uint;
pub const MONO_METHOD_SEMA_SEMANTICS: _bindgen_ty_28 = 0;
pub const MONO_METHOD_SEMA_METHOD: _bindgen_ty_28 = 1;
pub const MONO_METHOD_SEMA_ASSOCIATION: _bindgen_ty_28 = 2;
pub const MONO_METHOD_SEMA_SIZE: _bindgen_ty_28 = 3;
pub type _bindgen_ty_28 = ::std::os::raw::c_uint;
pub const MONO_MODULE_GENERATION: _bindgen_ty_29 = 0;
pub const MONO_MODULE_NAME: _bindgen_ty_29 = 1;
pub const MONO_MODULE_MVID: _bindgen_ty_29 = 2;
pub const MONO_MODULE_ENC: _bindgen_ty_29 = 3;
pub const MONO_MODULE_ENCBASE: _bindgen_ty_29 = 4;
pub const MONO_MODULE_SIZE: _bindgen_ty_29 = 5;
pub type _bindgen_ty_29 = ::std::os::raw::c_uint;
pub const MONO_MODULEREF_NAME: _bindgen_ty_30 = 0;
pub const MONO_MODULEREF_SIZE: _bindgen_ty_30 = 1;
pub type _bindgen_ty_30 = ::std::os::raw::c_uint;
pub const MONO_NESTED_CLASS_NESTED: _bindgen_ty_31 = 0;
pub const MONO_NESTED_CLASS_ENCLOSING: _bindgen_ty_31 = 1;
pub const MONO_NESTED_CLASS_SIZE: _bindgen_ty_31 = 2;
pub type _bindgen_ty_31 = ::std::os::raw::c_uint;
pub const MONO_PARAM_FLAGS: _bindgen_ty_32 = 0;
pub const MONO_PARAM_SEQUENCE: _bindgen_ty_32 = 1;
pub const MONO_PARAM_NAME: _bindgen_ty_32 = 2;
pub const MONO_PARAM_SIZE: _bindgen_ty_32 = 3;
pub type _bindgen_ty_32 = ::std::os::raw::c_uint;
pub const MONO_PARAM_POINTER_PARAM: _bindgen_ty_33 = 0;
pub const MONO_PARAM_POINTER_SIZE: _bindgen_ty_33 = 1;
pub type _bindgen_ty_33 = ::std::os::raw::c_uint;
pub const MONO_PROPERTY_FLAGS: _bindgen_ty_34 = 0;
pub const MONO_PROPERTY_NAME: _bindgen_ty_34 = 1;
pub const MONO_PROPERTY_TYPE: _bindgen_ty_34 = 2;
pub const MONO_PROPERTY_SIZE: _bindgen_ty_34 = 3;
pub type _bindgen_ty_34 = ::std::os::raw::c_uint;
pub const MONO_PROPERTY_POINTER_PROPERTY: _bindgen_ty_35 = 0;
pub const MONO_PROPERTY_POINTER_SIZE: _bindgen_ty_35 = 1;
pub type _bindgen_ty_35 = ::std::os::raw::c_uint;
pub const MONO_PROPERTY_MAP_PARENT: _bindgen_ty_36 = 0;
pub const MONO_PROPERTY_MAP_PROPERTY_LIST: _bindgen_ty_36 = 1;
pub const MONO_PROPERTY_MAP_SIZE: _bindgen_ty_36 = 2;
pub type _bindgen_ty_36 = ::std::os::raw::c_uint;
pub const MONO_STAND_ALONE_SIGNATURE: _bindgen_ty_37 = 0;
pub const MONO_STAND_ALONE_SIGNATURE_SIZE: _bindgen_ty_37 = 1;
pub type _bindgen_ty_37 = ::std::os::raw::c_uint;
pub const MONO_TYPEDEF_FLAGS: _bindgen_ty_38 = 0;
pub const MONO_TYPEDEF_NAME: _bindgen_ty_38 = 1;
pub const MONO_TYPEDEF_NAMESPACE: _bindgen_ty_38 = 2;
pub const MONO_TYPEDEF_EXTENDS: _bindgen_ty_38 = 3;
pub const MONO_TYPEDEF_FIELD_LIST: _bindgen_ty_38 = 4;
pub const MONO_TYPEDEF_METHOD_LIST: _bindgen_ty_38 = 5;
pub const MONO_TYPEDEF_SIZE: _bindgen_ty_38 = 6;
pub type _bindgen_ty_38 = ::std::os::raw::c_uint;
pub const MONO_TYPEREF_SCOPE: _bindgen_ty_39 = 0;
pub const MONO_TYPEREF_NAME: _bindgen_ty_39 = 1;
pub const MONO_TYPEREF_NAMESPACE: _bindgen_ty_39 = 2;
pub const MONO_TYPEREF_SIZE: _bindgen_ty_39 = 3;
pub type _bindgen_ty_39 = ::std::os::raw::c_uint;
pub const MONO_TYPESPEC_SIGNATURE: _bindgen_ty_40 = 0;
pub const MONO_TYPESPEC_SIZE: _bindgen_ty_40 = 1;
pub type _bindgen_ty_40 = ::std::os::raw::c_uint;
pub const MONO_GENERICPARAM_NUMBER: _bindgen_ty_41 = 0;
pub const MONO_GENERICPARAM_FLAGS: _bindgen_ty_41 = 1;
pub const MONO_GENERICPARAM_OWNER: _bindgen_ty_41 = 2;
pub const MONO_GENERICPARAM_NAME: _bindgen_ty_41 = 3;
pub const MONO_GENERICPARAM_SIZE: _bindgen_ty_41 = 4;
pub type _bindgen_ty_41 = ::std::os::raw::c_uint;
pub const MONO_METHODSPEC_METHOD: _bindgen_ty_42 = 0;
pub const MONO_METHODSPEC_SIGNATURE: _bindgen_ty_42 = 1;
pub const MONO_METHODSPEC_SIZE: _bindgen_ty_42 = 2;
pub type _bindgen_ty_42 = ::std::os::raw::c_uint;
pub const MONO_GENPARCONSTRAINT_GENERICPAR: _bindgen_ty_43 = 0;
pub const MONO_GENPARCONSTRAINT_CONSTRAINT: _bindgen_ty_43 = 1;
pub const MONO_GENPARCONSTRAINT_SIZE: _bindgen_ty_43 = 2;
pub type _bindgen_ty_43 = ::std::os::raw::c_uint;
pub const MONO_DOCUMENT_NAME: _bindgen_ty_44 = 0;
pub const MONO_DOCUMENT_HASHALG: _bindgen_ty_44 = 1;
pub const MONO_DOCUMENT_HASH: _bindgen_ty_44 = 2;
pub const MONO_DOCUMENT_LANGUAGE: _bindgen_ty_44 = 3;
pub const MONO_DOCUMENT_SIZE: _bindgen_ty_44 = 4;
pub type _bindgen_ty_44 = ::std::os::raw::c_uint;
pub const MONO_METHODBODY_DOCUMENT: _bindgen_ty_45 = 0;
pub const MONO_METHODBODY_SEQ_POINTS: _bindgen_ty_45 = 1;
pub const MONO_METHODBODY_SIZE: _bindgen_ty_45 = 2;
pub type _bindgen_ty_45 = ::std::os::raw::c_uint;
pub const MONO_LOCALSCOPE_METHOD: _bindgen_ty_46 = 0;
pub const MONO_LOCALSCOPE_IMPORTSCOPE: _bindgen_ty_46 = 1;
pub const MONO_LOCALSCOPE_VARIABLELIST: _bindgen_ty_46 = 2;
pub const MONO_LOCALSCOPE_CONSTANTLIST: _bindgen_ty_46 = 3;
pub const MONO_LOCALSCOPE_STARTOFFSET: _bindgen_ty_46 = 4;
pub const MONO_LOCALSCOPE_LENGTH: _bindgen_ty_46 = 5;
pub const MONO_LOCALSCOPE_SIZE: _bindgen_ty_46 = 6;
pub type _bindgen_ty_46 = ::std::os::raw::c_uint;
pub const MONO_LOCALVARIABLE_ATTRIBUTES: _bindgen_ty_47 = 0;
pub const MONO_LOCALVARIABLE_INDEX: _bindgen_ty_47 = 1;
pub const MONO_LOCALVARIABLE_NAME: _bindgen_ty_47 = 2;
pub const MONO_LOCALVARIABLE_SIZE: _bindgen_ty_47 = 3;
pub type _bindgen_ty_47 = ::std::os::raw::c_uint;
pub const MONO_CUSTOMDEBUGINFORMATION_PARENT: _bindgen_ty_48 = 0;
pub const MONO_CUSTOMDEBUGINFORMATION_KIND: _bindgen_ty_48 = 1;
pub const MONO_CUSTOMDEBUGINFORMATION_VALUE: _bindgen_ty_48 = 2;
pub const MONO_CUSTOMDEBUGINFORMATION_SIZE: _bindgen_ty_48 = 3;
pub type _bindgen_ty_48 = ::std::os::raw::c_uint;
pub const MONO_TYPEDEFORREF_TYPEDEF: _bindgen_ty_49 = 0;
pub const MONO_TYPEDEFORREF_TYPEREF: _bindgen_ty_49 = 1;
pub const MONO_TYPEDEFORREF_TYPESPEC: _bindgen_ty_49 = 2;
pub const MONO_TYPEDEFORREF_BITS: _bindgen_ty_49 = 2;
pub const MONO_TYPEDEFORREF_MASK: _bindgen_ty_49 = 3;
pub type _bindgen_ty_49 = ::std::os::raw::c_uint;
pub const MONO_HASCONSTANT_FIEDDEF: _bindgen_ty_50 = 0;
pub const MONO_HASCONSTANT_PARAM: _bindgen_ty_50 = 1;
pub const MONO_HASCONSTANT_PROPERTY: _bindgen_ty_50 = 2;
pub const MONO_HASCONSTANT_BITS: _bindgen_ty_50 = 2;
pub const MONO_HASCONSTANT_MASK: _bindgen_ty_50 = 3;
pub type _bindgen_ty_50 = ::std::os::raw::c_uint;
pub const MONO_CUSTOM_ATTR_METHODDEF: _bindgen_ty_51 = 0;
pub const MONO_CUSTOM_ATTR_FIELDDEF: _bindgen_ty_51 = 1;
pub const MONO_CUSTOM_ATTR_TYPEREF: _bindgen_ty_51 = 2;
pub const MONO_CUSTOM_ATTR_TYPEDEF: _bindgen_ty_51 = 3;
pub const MONO_CUSTOM_ATTR_PARAMDEF: _bindgen_ty_51 = 4;
pub const MONO_CUSTOM_ATTR_INTERFACE: _bindgen_ty_51 = 5;
pub const MONO_CUSTOM_ATTR_MEMBERREF: _bindgen_ty_51 = 6;
pub const MONO_CUSTOM_ATTR_MODULE: _bindgen_ty_51 = 7;
pub const MONO_CUSTOM_ATTR_PERMISSION: _bindgen_ty_51 = 8;
pub const MONO_CUSTOM_ATTR_PROPERTY: _bindgen_ty_51 = 9;
pub const MONO_CUSTOM_ATTR_EVENT: _bindgen_ty_51 = 10;
pub const MONO_CUSTOM_ATTR_SIGNATURE: _bindgen_ty_51 = 11;
pub const MONO_CUSTOM_ATTR_MODULEREF: _bindgen_ty_51 = 12;
pub const MONO_CUSTOM_ATTR_TYPESPEC: _bindgen_ty_51 = 13;
pub const MONO_CUSTOM_ATTR_ASSEMBLY: _bindgen_ty_51 = 14;
pub const MONO_CUSTOM_ATTR_ASSEMBLYREF: _bindgen_ty_51 = 15;
pub const MONO_CUSTOM_ATTR_FILE: _bindgen_ty_51 = 16;
pub const MONO_CUSTOM_ATTR_EXP_TYPE: _bindgen_ty_51 = 17;
pub const MONO_CUSTOM_ATTR_MANIFEST: _bindgen_ty_51 = 18;
pub const MONO_CUSTOM_ATTR_GENERICPAR: _bindgen_ty_51 = 19;
pub const MONO_CUSTOM_ATTR_GENERICPARAMCONSTRAINT: _bindgen_ty_51 = 20;
pub const MONO_CUSTOM_ATTR_BITS: _bindgen_ty_51 = 5;
pub const MONO_CUSTOM_ATTR_MASK: _bindgen_ty_51 = 31;
pub type _bindgen_ty_51 = ::std::os::raw::c_uint;
pub const MONO_HAS_FIELD_MARSHAL_FIELDSREF: _bindgen_ty_52 = 0;
pub const MONO_HAS_FIELD_MARSHAL_PARAMDEF: _bindgen_ty_52 = 1;
pub const MONO_HAS_FIELD_MARSHAL_BITS: _bindgen_ty_52 = 1;
pub const MONO_HAS_FIELD_MARSHAL_MASK: _bindgen_ty_52 = 1;
pub type _bindgen_ty_52 = ::std::os::raw::c_uint;
pub const MONO_HAS_DECL_SECURITY_TYPEDEF: _bindgen_ty_53 = 0;
pub const MONO_HAS_DECL_SECURITY_METHODDEF: _bindgen_ty_53 = 1;
pub const MONO_HAS_DECL_SECURITY_ASSEMBLY: _bindgen_ty_53 = 2;
pub const MONO_HAS_DECL_SECURITY_BITS: _bindgen_ty_53 = 2;
pub const MONO_HAS_DECL_SECURITY_MASK: _bindgen_ty_53 = 3;
pub type _bindgen_ty_53 = ::std::os::raw::c_uint;
pub const MONO_MEMBERREF_PARENT_TYPEDEF: _bindgen_ty_54 = 0;
pub const MONO_MEMBERREF_PARENT_TYPEREF: _bindgen_ty_54 = 1;
pub const MONO_MEMBERREF_PARENT_MODULEREF: _bindgen_ty_54 = 2;
pub const MONO_MEMBERREF_PARENT_METHODDEF: _bindgen_ty_54 = 3;
pub const MONO_MEMBERREF_PARENT_TYPESPEC: _bindgen_ty_54 = 4;
pub const MONO_MEMBERREF_PARENT_BITS: _bindgen_ty_54 = 3;
pub const MONO_MEMBERREF_PARENT_MASK: _bindgen_ty_54 = 7;
pub type _bindgen_ty_54 = ::std::os::raw::c_uint;
pub const MONO_HAS_SEMANTICS_EVENT: _bindgen_ty_55 = 0;
pub const MONO_HAS_SEMANTICS_PROPERTY: _bindgen_ty_55 = 1;
pub const MONO_HAS_SEMANTICS_BITS: _bindgen_ty_55 = 1;
pub const MONO_HAS_SEMANTICS_MASK: _bindgen_ty_55 = 1;
pub type _bindgen_ty_55 = ::std::os::raw::c_uint;
pub const MONO_METHODDEFORREF_METHODDEF: _bindgen_ty_56 = 0;
pub const MONO_METHODDEFORREF_METHODREF: _bindgen_ty_56 = 1;
pub const MONO_METHODDEFORREF_BITS: _bindgen_ty_56 = 1;
pub const MONO_METHODDEFORREF_MASK: _bindgen_ty_56 = 1;
pub type _bindgen_ty_56 = ::std::os::raw::c_uint;
pub const MONO_MEMBERFORWD_FIELDDEF: _bindgen_ty_57 = 0;
pub const MONO_MEMBERFORWD_METHODDEF: _bindgen_ty_57 = 1;
pub const MONO_MEMBERFORWD_BITS: _bindgen_ty_57 = 1;
pub const MONO_MEMBERFORWD_MASK: _bindgen_ty_57 = 1;
pub type _bindgen_ty_57 = ::std::os::raw::c_uint;
pub const MONO_IMPLEMENTATION_FILE: _bindgen_ty_58 = 0;
pub const MONO_IMPLEMENTATION_ASSEMBLYREF: _bindgen_ty_58 = 1;
pub const MONO_IMPLEMENTATION_EXP_TYPE: _bindgen_ty_58 = 2;
pub const MONO_IMPLEMENTATION_BITS: _bindgen_ty_58 = 2;
pub const MONO_IMPLEMENTATION_MASK: _bindgen_ty_58 = 3;
pub type _bindgen_ty_58 = ::std::os::raw::c_uint;
pub const MONO_CUSTOM_ATTR_TYPE_TYPEREF: _bindgen_ty_59 = 0;
pub const MONO_CUSTOM_ATTR_TYPE_TYPEDEF: _bindgen_ty_59 = 1;
pub const MONO_CUSTOM_ATTR_TYPE_METHODDEF: _bindgen_ty_59 = 2;
pub const MONO_CUSTOM_ATTR_TYPE_MEMBERREF: _bindgen_ty_59 = 3;
pub const MONO_CUSTOM_ATTR_TYPE_STRING: _bindgen_ty_59 = 4;
pub const MONO_CUSTOM_ATTR_TYPE_BITS: _bindgen_ty_59 = 3;
pub const MONO_CUSTOM_ATTR_TYPE_MASK: _bindgen_ty_59 = 7;
pub type _bindgen_ty_59 = ::std::os::raw::c_uint;
pub const MONO_RESOLUTION_SCOPE_MODULE: _bindgen_ty_60 = 0;
pub const MONO_RESOLUTION_SCOPE_MODULEREF: _bindgen_ty_60 = 1;
pub const MONO_RESOLUTION_SCOPE_ASSEMBLYREF: _bindgen_ty_60 = 2;
pub const MONO_RESOLUTION_SCOPE_TYPEREF: _bindgen_ty_60 = 3;
pub const MONO_RESOLUTION_SCOPE_BITS: _bindgen_ty_60 = 2;
pub const MONO_RESOLUTION_SCOPE_MASK: _bindgen_ty_60 = 3;
pub type _bindgen_ty_60 = ::std::os::raw::c_uint;
pub const MONO_RESOLTION_SCOPE_MODULE: _bindgen_ty_61 = 0;
pub const MONO_RESOLTION_SCOPE_MODULEREF: _bindgen_ty_61 = 1;
pub const MONO_RESOLTION_SCOPE_ASSEMBLYREF: _bindgen_ty_61 = 2;
pub const MONO_RESOLTION_SCOPE_TYPEREF: _bindgen_ty_61 = 3;
pub const MONO_RESOLTION_SCOPE_BITS: _bindgen_ty_61 = 2;
pub const MONO_RESOLTION_SCOPE_MASK: _bindgen_ty_61 = 3;
pub type _bindgen_ty_61 = ::std::os::raw::c_uint;
pub const MONO_TYPEORMETHOD_TYPE: _bindgen_ty_62 = 0;
pub const MONO_TYPEORMETHOD_METHOD: _bindgen_ty_62 = 1;
pub const MONO_TYPEORMETHOD_BITS: _bindgen_ty_62 = 1;
pub const MONO_TYPEORMETHOD_MASK: _bindgen_ty_62 = 1;
pub type _bindgen_ty_62 = ::std::os::raw::c_uint;
pub type va_list = __builtin_va_list;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
}
#[test]
fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    fn test_field___wch() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__mbstate_t__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wch) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__mbstate_t__bindgen_ty_1),
                "::",
                stringify!(__wch)
            )
        );
    }
    test_field___wch();
    fn test_field___wchb() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__mbstate_t__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__wchb) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__mbstate_t__bindgen_ty_1),
                "::",
                stringify!(__wchb)
            )
        );
    }
    test_field___wchb();
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        8usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    fn test_field___count() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__mbstate_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__count) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__mbstate_t),
                "::",
                stringify!(__count)
            )
        );
    }
    test_field___count();
    fn test_field___value() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__mbstate_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__value) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__mbstate_t),
                "::",
                stringify!(__value)
            )
        );
    }
    test_field___value();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos_t() {
    assert_eq!(
        ::std::mem::size_of::<_G_fpos_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos_t))
    );
    fn test_field___pos() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_G_fpos_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pos) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_G_fpos_t),
                "::",
                stringify!(__pos)
            )
        );
    }
    test_field___pos();
    fn test_field___state() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_G_fpos_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__state) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_G_fpos_t),
                "::",
                stringify!(__state)
            )
        );
    }
    test_field___state();
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos64_t() {
    assert_eq!(
        ::std::mem::size_of::<_G_fpos64_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos64_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos64_t))
    );
    fn test_field___pos() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_G_fpos64_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pos) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_G_fpos64_t),
                "::",
                stringify!(__pos)
            )
        );
    }
    test_field___pos();
    fn test_field___state() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_G_fpos64_t>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__state) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_G_fpos64_t),
                "::",
                stringify!(__state)
            )
        );
    }
    test_field___state();
}
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: size_t,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    fn test_field__flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_flags)
            )
        );
    }
    test_field__flags();
    fn test_field__IO_read_ptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_read_ptr) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_read_ptr)
            )
        );
    }
    test_field__IO_read_ptr();
    fn test_field__IO_read_end() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_read_end) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_read_end)
            )
        );
    }
    test_field__IO_read_end();
    fn test_field__IO_read_base() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_read_base) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_read_base)
            )
        );
    }
    test_field__IO_read_base();
    fn test_field__IO_write_base() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_write_base) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_write_base)
            )
        );
    }
    test_field__IO_write_base();
    fn test_field__IO_write_ptr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_write_ptr) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_write_ptr)
            )
        );
    }
    test_field__IO_write_ptr();
    fn test_field__IO_write_end() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_write_end) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_write_end)
            )
        );
    }
    test_field__IO_write_end();
    fn test_field__IO_buf_base() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_buf_base) as usize - ptr as usize
            },
            56usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_buf_base)
            )
        );
    }
    test_field__IO_buf_base();
    fn test_field__IO_buf_end() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_buf_end) as usize - ptr as usize
            },
            64usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_buf_end)
            )
        );
    }
    test_field__IO_buf_end();
    fn test_field__IO_save_base() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_save_base) as usize - ptr as usize
            },
            72usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_save_base)
            )
        );
    }
    test_field__IO_save_base();
    fn test_field__IO_backup_base() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_backup_base) as usize - ptr as usize
            },
            80usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_backup_base)
            )
        );
    }
    test_field__IO_backup_base();
    fn test_field__IO_save_end() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._IO_save_end) as usize - ptr as usize
            },
            88usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_IO_save_end)
            )
        );
    }
    test_field__IO_save_end();
    fn test_field__markers() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._markers) as usize - ptr as usize
            },
            96usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_markers)
            )
        );
    }
    test_field__markers();
    fn test_field__chain() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._chain) as usize - ptr as usize
            },
            104usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_chain)
            )
        );
    }
    test_field__chain();
    fn test_field__fileno() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._fileno) as usize - ptr as usize
            },
            112usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_fileno)
            )
        );
    }
    test_field__fileno();
    fn test_field__flags2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._flags2) as usize - ptr as usize
            },
            116usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_flags2)
            )
        );
    }
    test_field__flags2();
    fn test_field__old_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._old_offset) as usize - ptr as usize
            },
            120usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_old_offset)
            )
        );
    }
    test_field__old_offset();
    fn test_field__cur_column() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._cur_column) as usize - ptr as usize
            },
            128usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_cur_column)
            )
        );
    }
    test_field__cur_column();
    fn test_field__vtable_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._vtable_offset) as usize - ptr as usize
            },
            130usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_vtable_offset)
            )
        );
    }
    test_field__vtable_offset();
    fn test_field__shortbuf() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._shortbuf) as usize - ptr as usize
            },
            131usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_shortbuf)
            )
        );
    }
    test_field__shortbuf();
    fn test_field__lock() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._lock) as usize - ptr as usize
            },
            136usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_lock)
            )
        );
    }
    test_field__lock();
    fn test_field__offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._offset) as usize - ptr as usize
            },
            144usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_offset)
            )
        );
    }
    test_field__offset();
    fn test_field__codecvt() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._codecvt) as usize - ptr as usize
            },
            152usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_codecvt)
            )
        );
    }
    test_field__codecvt();
    fn test_field__wide_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._wide_data) as usize - ptr as usize
            },
            160usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_wide_data)
            )
        );
    }
    test_field__wide_data();
    fn test_field__freeres_list() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._freeres_list) as usize - ptr as usize
            },
            168usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_freeres_list)
            )
        );
    }
    test_field__freeres_list();
    fn test_field__freeres_buf() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._freeres_buf) as usize - ptr as usize
            },
            176usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_freeres_buf)
            )
        );
    }
    test_field__freeres_buf();
    fn test_field___pad5() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).__pad5) as usize - ptr as usize
            },
            184usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(__pad5)
            )
        );
    }
    test_field___pad5();
    fn test_field__mode() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._mode) as usize - ptr as usize
            },
            192usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_mode)
            )
        );
    }
    test_field__mode();
    fn test_field__unused2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_IO_FILE>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr)._unused2) as usize - ptr as usize
            },
            196usize,
            concat!(
                "Offset of field: ",
                stringify!(_IO_FILE),
                "::",
                stringify!(_unused2)
            )
        );
    }
    test_field__unused2();
}
pub type fpos_t = __fpos_t;
extern "C" {
    pub static mut stdin: *mut FILE;
}
extern "C" {
    pub static mut stdout: *mut FILE;
}
extern "C" {
    pub static mut stderr: *mut FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: size_t,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut size_t,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: size_t);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut size_t,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut size_t,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut size_t,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __s: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
pub const MONO_ERROR_FREE_STRINGS: _bindgen_ty_63 = 1;
pub const MONO_ERROR_INCOMPLETE: _bindgen_ty_63 = 2;
pub const MONO_ERROR_MEMPOOL_BOXED: _bindgen_ty_63 = 4;
pub type _bindgen_ty_63 = ::std::os::raw::c_uint;
pub const MONO_ERROR_NONE: _bindgen_ty_64 = 0;
pub const MONO_ERROR_MISSING_METHOD: _bindgen_ty_64 = 1;
pub const MONO_ERROR_MISSING_FIELD: _bindgen_ty_64 = 2;
pub const MONO_ERROR_TYPE_LOAD: _bindgen_ty_64 = 3;
pub const MONO_ERROR_FILE_NOT_FOUND: _bindgen_ty_64 = 4;
pub const MONO_ERROR_BAD_IMAGE: _bindgen_ty_64 = 5;
pub const MONO_ERROR_OUT_OF_MEMORY: _bindgen_ty_64 = 6;
pub const MONO_ERROR_ARGUMENT: _bindgen_ty_64 = 7;
pub const MONO_ERROR_ARGUMENT_NULL: _bindgen_ty_64 = 11;
pub const MONO_ERROR_ARGUMENT_OUT_OF_RANGE: _bindgen_ty_64 = 14;
pub const MONO_ERROR_NOT_VERIFIABLE: _bindgen_ty_64 = 8;
pub const MONO_ERROR_INVALID_PROGRAM: _bindgen_ty_64 = 12;
pub const MONO_ERROR_MEMBER_ACCESS: _bindgen_ty_64 = 13;
pub const MONO_ERROR_GENERIC: _bindgen_ty_64 = 9;
pub const MONO_ERROR_EXCEPTION_INSTANCE: _bindgen_ty_64 = 10;
pub const MONO_ERROR_CLEANUP_CALLED_SENTINEL: _bindgen_ty_64 = 65535;
pub type _bindgen_ty_64 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub union _MonoError {
    pub init: u32,
    pub __bindgen_anon_1: _MonoError__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoError__bindgen_ty_1 {
    pub error_code: u16,
    pub private_flags: u16,
    pub hidden_1: [*mut ::std::os::raw::c_void; 12usize],
}
#[test]
fn bindgen_test_layout__MonoError__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<_MonoError__bindgen_ty_1>(),
        104usize,
        concat!("Size of: ", stringify!(_MonoError__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<_MonoError__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(_MonoError__bindgen_ty_1))
    );
    fn test_field_error_code() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoError__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).error_code) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoError__bindgen_ty_1),
                "::",
                stringify!(error_code)
            )
        );
    }
    test_field_error_code();
    fn test_field_private_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoError__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).private_flags) as usize - ptr as usize
            },
            2usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoError__bindgen_ty_1),
                "::",
                stringify!(private_flags)
            )
        );
    }
    test_field_private_flags();
    fn test_field_hidden_1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoError__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).hidden_1) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoError__bindgen_ty_1),
                "::",
                stringify!(hidden_1)
            )
        );
    }
    test_field_hidden_1();
}
#[test]
fn bindgen_test_layout__MonoError() {
    assert_eq!(
        ::std::mem::size_of::<_MonoError>(),
        104usize,
        concat!("Size of: ", stringify!(_MonoError))
    );
    assert_eq!(
        ::std::mem::align_of::<_MonoError>(),
        8usize,
        concat!("Alignment of ", stringify!(_MonoError))
    );
    fn test_field_init() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoError>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).init) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoError),
                "::",
                stringify!(init)
            )
        );
    }
    test_field_init();
}
pub type MonoErrorExternal = _MonoError;
pub type MonoError = MonoErrorExternal;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoErrorBoxed {
    _unused: [u8; 0],
}
pub type MonoErrorBoxed = _MonoErrorBoxed;
extern "C" {
    pub fn mono_error_init(error: *mut MonoError);
}
extern "C" {
    pub fn mono_error_init_flags(error: *mut MonoError, flags: ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn mono_error_cleanup(error: *mut MonoError);
}
extern "C" {
    pub fn mono_error_ok(error: *mut MonoError) -> mono_bool;
}
extern "C" {
    pub fn mono_error_get_error_code(error: *mut MonoError) -> ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn mono_error_get_message(error: *mut MonoError) -> *const ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoAssembly {
    _unused: [u8; 0],
}
pub type MonoAssembly = _MonoAssembly;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoAssemblyName {
    _unused: [u8; 0],
}
pub type MonoAssemblyName = _MonoAssemblyName;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoTableInfo {
    _unused: [u8; 0],
}
pub type MonoTableInfo = _MonoTableInfo;
pub const MonoImageOpenStatus_MONO_IMAGE_OK: MonoImageOpenStatus = 0;
pub const MonoImageOpenStatus_MONO_IMAGE_ERROR_ERRNO: MonoImageOpenStatus = 1;
pub const MonoImageOpenStatus_MONO_IMAGE_MISSING_ASSEMBLYREF: MonoImageOpenStatus = 2;
pub const MonoImageOpenStatus_MONO_IMAGE_IMAGE_INVALID: MonoImageOpenStatus = 3;
pub type MonoImageOpenStatus = ::std::os::raw::c_uint;
extern "C" {
    pub fn mono_images_init();
}
extern "C" {
    pub fn mono_images_cleanup();
}
extern "C" {
    pub fn mono_image_open(
        fname: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_open_full(
        fname: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
        refonly: mono_bool,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_pe_file_open(
        fname: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_open_from_data(
        data: *mut ::std::os::raw::c_char,
        data_len: u32,
        need_copy: mono_bool,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_open_from_data_full(
        data: *mut ::std::os::raw::c_char,
        data_len: u32,
        need_copy: mono_bool,
        status: *mut MonoImageOpenStatus,
        refonly: mono_bool,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_open_from_data_with_name(
        data: *mut ::std::os::raw::c_char,
        data_len: u32,
        need_copy: mono_bool,
        status: *mut MonoImageOpenStatus,
        refonly: mono_bool,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_fixup_vtable(image: *mut MonoImage);
}
extern "C" {
    pub fn mono_image_loaded(name: *const ::std::os::raw::c_char) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_loaded_full(
        name: *const ::std::os::raw::c_char,
        refonly: mono_bool,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_loaded_by_guid(guid: *const ::std::os::raw::c_char) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_loaded_by_guid_full(
        guid: *const ::std::os::raw::c_char,
        refonly: mono_bool,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_init(image: *mut MonoImage);
}
extern "C" {
    pub fn mono_image_close(image: *mut MonoImage);
}
extern "C" {
    pub fn mono_image_addref(image: *mut MonoImage);
}
extern "C" {
    pub fn mono_image_strerror(status: MonoImageOpenStatus) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_ensure_section(
        image: *mut MonoImage,
        section: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_image_ensure_section_idx(
        image: *mut MonoImage,
        section: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_image_get_entry_point(image: *mut MonoImage) -> u32;
}
extern "C" {
    pub fn mono_image_get_resource(
        image: *mut MonoImage,
        offset: u32,
        size: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_load_file_for_image(
        image: *mut MonoImage,
        fileidx: ::std::os::raw::c_int,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_load_module(
        image: *mut MonoImage,
        idx: ::std::os::raw::c_int,
    ) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_image_get_name(image: *mut MonoImage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_get_filename(image: *mut MonoImage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_get_guid(image: *mut MonoImage) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_get_assembly(image: *mut MonoImage) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_image_is_dynamic(image: *mut MonoImage) -> mono_bool;
}
extern "C" {
    pub fn mono_image_rva_map(image: *mut MonoImage, rva: u32) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_get_table_info(
        image: *mut MonoImage,
        table_id: ::std::os::raw::c_int,
    ) -> *const MonoTableInfo;
}
extern "C" {
    pub fn mono_image_get_table_rows(
        image: *mut MonoImage,
        table_id: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_table_info_get_rows(table: *const MonoTableInfo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_image_lookup_resource(
        image: *mut MonoImage,
        res_id: u32,
        lang_id: u32,
        name: *mut mono_unichar2,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_image_get_public_key(
        image: *mut MonoImage,
        size: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_get_strong_name(
        image: *mut MonoImage,
        size: *mut u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_image_strong_name_position(image: *mut MonoImage, size: *mut u32) -> u32;
}
extern "C" {
    pub fn mono_image_add_to_name_cache(
        image: *mut MonoImage,
        nspace: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        idx: u32,
    );
}
extern "C" {
    pub fn mono_image_has_authenticode_entry(image: *mut MonoImage) -> mono_bool;
}
pub const MonoExceptionEnum_MONO_EXCEPTION_CLAUSE_NONE: MonoExceptionEnum = 0;
pub const MonoExceptionEnum_MONO_EXCEPTION_CLAUSE_FILTER: MonoExceptionEnum = 1;
pub const MonoExceptionEnum_MONO_EXCEPTION_CLAUSE_FINALLY: MonoExceptionEnum = 2;
pub const MonoExceptionEnum_MONO_EXCEPTION_CLAUSE_FAULT: MonoExceptionEnum = 4;
pub type MonoExceptionEnum = ::std::os::raw::c_uint;
pub const MonoCallConvention_MONO_CALL_DEFAULT: MonoCallConvention = 0;
pub const MonoCallConvention_MONO_CALL_C: MonoCallConvention = 1;
pub const MonoCallConvention_MONO_CALL_STDCALL: MonoCallConvention = 2;
pub const MonoCallConvention_MONO_CALL_THISCALL: MonoCallConvention = 3;
pub const MonoCallConvention_MONO_CALL_FASTCALL: MonoCallConvention = 4;
pub const MonoCallConvention_MONO_CALL_VARARG: MonoCallConvention = 5;
pub type MonoCallConvention = ::std::os::raw::c_uint;
pub const MonoMarshalNative_MONO_NATIVE_BOOLEAN: MonoMarshalNative = 2;
pub const MonoMarshalNative_MONO_NATIVE_I1: MonoMarshalNative = 3;
pub const MonoMarshalNative_MONO_NATIVE_U1: MonoMarshalNative = 4;
pub const MonoMarshalNative_MONO_NATIVE_I2: MonoMarshalNative = 5;
pub const MonoMarshalNative_MONO_NATIVE_U2: MonoMarshalNative = 6;
pub const MonoMarshalNative_MONO_NATIVE_I4: MonoMarshalNative = 7;
pub const MonoMarshalNative_MONO_NATIVE_U4: MonoMarshalNative = 8;
pub const MonoMarshalNative_MONO_NATIVE_I8: MonoMarshalNative = 9;
pub const MonoMarshalNative_MONO_NATIVE_U8: MonoMarshalNative = 10;
pub const MonoMarshalNative_MONO_NATIVE_R4: MonoMarshalNative = 11;
pub const MonoMarshalNative_MONO_NATIVE_R8: MonoMarshalNative = 12;
pub const MonoMarshalNative_MONO_NATIVE_CURRENCY: MonoMarshalNative = 15;
pub const MonoMarshalNative_MONO_NATIVE_BSTR: MonoMarshalNative = 19;
pub const MonoMarshalNative_MONO_NATIVE_LPSTR: MonoMarshalNative = 20;
pub const MonoMarshalNative_MONO_NATIVE_LPWSTR: MonoMarshalNative = 21;
pub const MonoMarshalNative_MONO_NATIVE_LPTSTR: MonoMarshalNative = 22;
pub const MonoMarshalNative_MONO_NATIVE_BYVALTSTR: MonoMarshalNative = 23;
pub const MonoMarshalNative_MONO_NATIVE_IUNKNOWN: MonoMarshalNative = 25;
pub const MonoMarshalNative_MONO_NATIVE_IDISPATCH: MonoMarshalNative = 26;
pub const MonoMarshalNative_MONO_NATIVE_STRUCT: MonoMarshalNative = 27;
pub const MonoMarshalNative_MONO_NATIVE_INTERFACE: MonoMarshalNative = 28;
pub const MonoMarshalNative_MONO_NATIVE_SAFEARRAY: MonoMarshalNative = 29;
pub const MonoMarshalNative_MONO_NATIVE_BYVALARRAY: MonoMarshalNative = 30;
pub const MonoMarshalNative_MONO_NATIVE_INT: MonoMarshalNative = 31;
pub const MonoMarshalNative_MONO_NATIVE_UINT: MonoMarshalNative = 32;
pub const MonoMarshalNative_MONO_NATIVE_VBBYREFSTR: MonoMarshalNative = 34;
pub const MonoMarshalNative_MONO_NATIVE_ANSIBSTR: MonoMarshalNative = 35;
pub const MonoMarshalNative_MONO_NATIVE_TBSTR: MonoMarshalNative = 36;
pub const MonoMarshalNative_MONO_NATIVE_VARIANTBOOL: MonoMarshalNative = 37;
pub const MonoMarshalNative_MONO_NATIVE_FUNC: MonoMarshalNative = 38;
pub const MonoMarshalNative_MONO_NATIVE_ASANY: MonoMarshalNative = 40;
pub const MonoMarshalNative_MONO_NATIVE_LPARRAY: MonoMarshalNative = 42;
pub const MonoMarshalNative_MONO_NATIVE_LPSTRUCT: MonoMarshalNative = 43;
pub const MonoMarshalNative_MONO_NATIVE_CUSTOM: MonoMarshalNative = 44;
pub const MonoMarshalNative_MONO_NATIVE_ERROR: MonoMarshalNative = 45;
pub const MonoMarshalNative_MONO_NATIVE_UTF8STR: MonoMarshalNative = 48;
pub const MonoMarshalNative_MONO_NATIVE_MAX: MonoMarshalNative = 80;
pub type MonoMarshalNative = ::std::os::raw::c_uint;
pub const MonoMarshalVariant_MONO_VARIANT_EMPTY: MonoMarshalVariant = 0;
pub const MonoMarshalVariant_MONO_VARIANT_NULL: MonoMarshalVariant = 1;
pub const MonoMarshalVariant_MONO_VARIANT_I2: MonoMarshalVariant = 2;
pub const MonoMarshalVariant_MONO_VARIANT_I4: MonoMarshalVariant = 3;
pub const MonoMarshalVariant_MONO_VARIANT_R4: MonoMarshalVariant = 4;
pub const MonoMarshalVariant_MONO_VARIANT_R8: MonoMarshalVariant = 5;
pub const MonoMarshalVariant_MONO_VARIANT_CY: MonoMarshalVariant = 6;
pub const MonoMarshalVariant_MONO_VARIANT_DATE: MonoMarshalVariant = 7;
pub const MonoMarshalVariant_MONO_VARIANT_BSTR: MonoMarshalVariant = 8;
pub const MonoMarshalVariant_MONO_VARIANT_DISPATCH: MonoMarshalVariant = 9;
pub const MonoMarshalVariant_MONO_VARIANT_ERROR: MonoMarshalVariant = 10;
pub const MonoMarshalVariant_MONO_VARIANT_BOOL: MonoMarshalVariant = 11;
pub const MonoMarshalVariant_MONO_VARIANT_VARIANT: MonoMarshalVariant = 12;
pub const MonoMarshalVariant_MONO_VARIANT_UNKNOWN: MonoMarshalVariant = 13;
pub const MonoMarshalVariant_MONO_VARIANT_DECIMAL: MonoMarshalVariant = 14;
pub const MonoMarshalVariant_MONO_VARIANT_I1: MonoMarshalVariant = 16;
pub const MonoMarshalVariant_MONO_VARIANT_UI1: MonoMarshalVariant = 17;
pub const MonoMarshalVariant_MONO_VARIANT_UI2: MonoMarshalVariant = 18;
pub const MonoMarshalVariant_MONO_VARIANT_UI4: MonoMarshalVariant = 19;
pub const MonoMarshalVariant_MONO_VARIANT_I8: MonoMarshalVariant = 20;
pub const MonoMarshalVariant_MONO_VARIANT_UI8: MonoMarshalVariant = 21;
pub const MonoMarshalVariant_MONO_VARIANT_INT: MonoMarshalVariant = 22;
pub const MonoMarshalVariant_MONO_VARIANT_UINT: MonoMarshalVariant = 23;
pub const MonoMarshalVariant_MONO_VARIANT_VOID: MonoMarshalVariant = 24;
pub const MonoMarshalVariant_MONO_VARIANT_HRESULT: MonoMarshalVariant = 25;
pub const MonoMarshalVariant_MONO_VARIANT_PTR: MonoMarshalVariant = 26;
pub const MonoMarshalVariant_MONO_VARIANT_SAFEARRAY: MonoMarshalVariant = 27;
pub const MonoMarshalVariant_MONO_VARIANT_CARRAY: MonoMarshalVariant = 28;
pub const MonoMarshalVariant_MONO_VARIANT_USERDEFINED: MonoMarshalVariant = 29;
pub const MonoMarshalVariant_MONO_VARIANT_LPSTR: MonoMarshalVariant = 30;
pub const MonoMarshalVariant_MONO_VARIANT_LPWSTR: MonoMarshalVariant = 31;
pub const MonoMarshalVariant_MONO_VARIANT_RECORD: MonoMarshalVariant = 36;
pub const MonoMarshalVariant_MONO_VARIANT_FILETIME: MonoMarshalVariant = 64;
pub const MonoMarshalVariant_MONO_VARIANT_BLOB: MonoMarshalVariant = 65;
pub const MonoMarshalVariant_MONO_VARIANT_STREAM: MonoMarshalVariant = 66;
pub const MonoMarshalVariant_MONO_VARIANT_STORAGE: MonoMarshalVariant = 67;
pub const MonoMarshalVariant_MONO_VARIANT_STREAMED_OBJECT: MonoMarshalVariant = 68;
pub const MonoMarshalVariant_MONO_VARIANT_STORED_OBJECT: MonoMarshalVariant = 69;
pub const MonoMarshalVariant_MONO_VARIANT_BLOB_OBJECT: MonoMarshalVariant = 70;
pub const MonoMarshalVariant_MONO_VARIANT_CF: MonoMarshalVariant = 71;
pub const MonoMarshalVariant_MONO_VARIANT_CLSID: MonoMarshalVariant = 72;
pub const MonoMarshalVariant_MONO_VARIANT_VECTOR: MonoMarshalVariant = 4096;
pub const MonoMarshalVariant_MONO_VARIANT_ARRAY: MonoMarshalVariant = 8192;
pub const MonoMarshalVariant_MONO_VARIANT_BYREF: MonoMarshalVariant = 16384;
pub type MonoMarshalVariant = ::std::os::raw::c_uint;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_NONE: MonoMarshalConv = 0;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_BOOL_VARIANTBOOL: MonoMarshalConv = 1;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_BOOL_I4: MonoMarshalConv = 2;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_BSTR: MonoMarshalConv = 3;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_LPSTR: MonoMarshalConv = 4;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_LPSTR_STR: MonoMarshalConv = 5;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_LPTSTR_STR: MonoMarshalConv = 6;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_LPWSTR: MonoMarshalConv = 7;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_LPWSTR_STR: MonoMarshalConv = 8;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_LPTSTR: MonoMarshalConv = 9;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_ANSIBSTR: MonoMarshalConv = 10;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_TBSTR: MonoMarshalConv = 11;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_BYVALSTR: MonoMarshalConv = 12;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_BYVALWSTR: MonoMarshalConv = 13;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_SB_LPSTR: MonoMarshalConv = 14;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_SB_LPTSTR: MonoMarshalConv = 15;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_SB_LPWSTR: MonoMarshalConv = 16;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_LPSTR_SB: MonoMarshalConv = 17;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_LPTSTR_SB: MonoMarshalConv = 18;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_LPWSTR_SB: MonoMarshalConv = 19;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_ARRAY_BYVALARRAY: MonoMarshalConv = 20;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_ARRAY_BYVALCHARARRAY: MonoMarshalConv = 21;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_ARRAY_SAVEARRAY: MonoMarshalConv = 22;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_ARRAY_LPARRAY: MonoMarshalConv = 23;
pub const MonoMarshalConv_MONO_MARSHAL_FREE_LPARRAY: MonoMarshalConv = 24;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_OBJECT_INTERFACE: MonoMarshalConv = 25;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_OBJECT_IDISPATCH: MonoMarshalConv = 26;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_OBJECT_IUNKNOWN: MonoMarshalConv = 27;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_OBJECT_STRUCT: MonoMarshalConv = 28;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_DEL_FTN: MonoMarshalConv = 29;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_FTN_DEL: MonoMarshalConv = 30;
pub const MonoMarshalConv_MONO_MARSHAL_FREE_ARRAY: MonoMarshalConv = 31;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_BSTR_STR: MonoMarshalConv = 32;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_SAFEHANDLE: MonoMarshalConv = 33;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_HANDLEREF: MonoMarshalConv = 34;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_STR_UTF8STR: MonoMarshalConv = 35;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_SB_UTF8STR: MonoMarshalConv = 36;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_UTF8STR_STR: MonoMarshalConv = 37;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_UTF8STR_SB: MonoMarshalConv = 38;
pub const MonoMarshalConv_MONO_MARSHAL_CONV_FIXED_BUFFER: MonoMarshalConv = 39;
pub type MonoMarshalConv = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MonoMarshalSpec {
    pub native: MonoMarshalNative,
    pub data: MonoMarshalSpec__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MonoMarshalSpec__bindgen_ty_1 {
    pub array_data: MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1,
    pub custom_data: MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2,
    pub safearray_data: MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1 {
    pub elem_type: MonoMarshalNative,
    pub num_elem: i32,
    pub param_num: i16,
    pub elem_mult: i16,
}
#[test]
fn bindgen_test_layout_MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1>(),
        12usize,
        concat!(
            "Size of: ",
            stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1)
        )
    );
    fn test_field_elem_type() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).elem_type) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(elem_type)
            )
        );
    }
    test_field_elem_type();
    fn test_field_num_elem() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).num_elem) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(num_elem)
            )
        );
    }
    test_field_num_elem();
    fn test_field_param_num() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).param_num) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(param_num)
            )
        );
    }
    test_field_param_num();
    fn test_field_elem_mult() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).elem_mult) as usize - ptr as usize
            },
            10usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(elem_mult)
            )
        );
    }
    test_field_elem_mult();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2 {
    pub custom_name: *mut ::std::os::raw::c_char,
    pub cookie: *mut ::std::os::raw::c_char,
    pub image: *mut MonoImage,
}
#[test]
fn bindgen_test_layout_MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2)
        )
    );
    fn test_field_custom_name() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).custom_name) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(custom_name)
            )
        );
    }
    test_field_custom_name();
    fn test_field_cookie() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cookie) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(cookie)
            )
        );
    }
    test_field_cookie();
    fn test_field_image() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).image) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(image)
            )
        );
    }
    test_field_image();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3 {
    pub elem_type: MonoMarshalVariant,
    pub num_elem: i32,
}
#[test]
fn bindgen_test_layout_MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3() {
    assert_eq!(
        ::std::mem::size_of::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3>(),
        8usize,
        concat!(
            "Size of: ",
            stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3)
        )
    );
    fn test_field_elem_type() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).elem_type) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3),
                "::",
                stringify!(elem_type)
            )
        );
    }
    test_field_elem_type();
    fn test_field_num_elem() {
        assert_eq!(
            unsafe {
                let uninit =
                    ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3>::uninit(
                    );
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).num_elem) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1__bindgen_ty_3),
                "::",
                stringify!(num_elem)
            )
        );
    }
    test_field_num_elem();
}
#[test]
fn bindgen_test_layout_MonoMarshalSpec__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<MonoMarshalSpec__bindgen_ty_1>(),
        24usize,
        concat!("Size of: ", stringify!(MonoMarshalSpec__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoMarshalSpec__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoMarshalSpec__bindgen_ty_1))
    );
    fn test_field_array_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).array_data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1),
                "::",
                stringify!(array_data)
            )
        );
    }
    test_field_array_data();
    fn test_field_custom_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).custom_data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1),
                "::",
                stringify!(custom_data)
            )
        );
    }
    test_field_custom_data();
    fn test_field_safearray_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoMarshalSpec__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).safearray_data) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec__bindgen_ty_1),
                "::",
                stringify!(safearray_data)
            )
        );
    }
    test_field_safearray_data();
}
#[test]
fn bindgen_test_layout_MonoMarshalSpec() {
    assert_eq!(
        ::std::mem::size_of::<MonoMarshalSpec>(),
        32usize,
        concat!("Size of: ", stringify!(MonoMarshalSpec))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoMarshalSpec>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoMarshalSpec))
    );
    fn test_field_native() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoMarshalSpec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).native) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec),
                "::",
                stringify!(native)
            )
        );
    }
    test_field_native();
    fn test_field_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoMarshalSpec>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoMarshalSpec),
                "::",
                stringify!(data)
            )
        );
    }
    test_field_data();
}
extern "C" {
    pub fn mono_metadata_init();
}
extern "C" {
    pub fn mono_metadata_decode_row(
        t: *const MonoTableInfo,
        idx: ::std::os::raw::c_int,
        res: *mut u32,
        res_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mono_metadata_decode_row_col(
        t: *const MonoTableInfo,
        idx: ::std::os::raw::c_int,
        col: ::std::os::raw::c_uint,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_compute_size(
        meta: *mut MonoImage,
        tableindex: ::std::os::raw::c_int,
        result_bitfield: *mut u32,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_metadata_locate(
        meta: *mut MonoImage,
        table: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_locate_token(
        meta: *mut MonoImage,
        token: u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_string_heap(
        meta: *mut MonoImage,
        table_index: u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_blob_heap(
        meta: *mut MonoImage,
        table_index: u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_user_string(
        meta: *mut MonoImage,
        table_index: u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_guid_heap(
        meta: *mut MonoImage,
        table_index: u32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_typedef_from_field(meta: *mut MonoImage, table_index: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_typedef_from_method(meta: *mut MonoImage, table_index: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_nested_in_typedef(meta: *mut MonoImage, table_index: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_nesting_typedef(
        meta: *mut MonoImage,
        table_index: u32,
        start_index: u32,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_interfaces_from_typedef(
        meta: *mut MonoImage,
        table_index: u32,
        count: *mut ::std::os::raw::c_uint,
    ) -> *mut *mut MonoClass;
}
extern "C" {
    pub fn mono_metadata_events_from_typedef(
        meta: *mut MonoImage,
        table_index: u32,
        end_idx: *mut ::std::os::raw::c_uint,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_methods_from_event(
        meta: *mut MonoImage,
        table_index: u32,
        end: *mut ::std::os::raw::c_uint,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_properties_from_typedef(
        meta: *mut MonoImage,
        table_index: u32,
        end: *mut ::std::os::raw::c_uint,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_methods_from_property(
        meta: *mut MonoImage,
        table_index: u32,
        end: *mut ::std::os::raw::c_uint,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_packing_from_typedef(
        meta: *mut MonoImage,
        table_index: u32,
        packing: *mut u32,
        size: *mut u32,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_get_marshal_info(
        meta: *mut MonoImage,
        idx: u32,
        is_field: mono_bool,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_custom_attrs_from_index(meta: *mut MonoImage, cattr_index: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_parse_marshal_spec(
        image: *mut MonoImage,
        ptr: *const ::std::os::raw::c_char,
    ) -> *mut MonoMarshalSpec;
}
extern "C" {
    pub fn mono_metadata_free_marshal_spec(spec: *mut MonoMarshalSpec);
}
extern "C" {
    pub fn mono_metadata_implmap_from_method(meta: *mut MonoImage, method_idx: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_field_info(
        meta: *mut MonoImage,
        table_index: u32,
        offset: *mut u32,
        rva: *mut u32,
        marshal_spec: *mut *mut MonoMarshalSpec,
    );
}
extern "C" {
    pub fn mono_metadata_get_constant_index(meta: *mut MonoImage, token: u32, hint: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_decode_value(
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_decode_signed_value(
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> i32;
}
extern "C" {
    pub fn mono_metadata_decode_blob_size(
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_encode_value(
        value: u32,
        bug: *mut ::std::os::raw::c_char,
        endbuf: *mut *mut ::std::os::raw::c_char,
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MonoExceptionClause {
    pub flags: u32,
    pub try_offset: u32,
    pub try_len: u32,
    pub handler_offset: u32,
    pub handler_len: u32,
    pub data: MonoExceptionClause__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union MonoExceptionClause__bindgen_ty_1 {
    pub filter_offset: u32,
    pub catch_class: *mut MonoClass,
}
#[test]
fn bindgen_test_layout_MonoExceptionClause__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<MonoExceptionClause__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(MonoExceptionClause__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoExceptionClause__bindgen_ty_1>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(MonoExceptionClause__bindgen_ty_1)
        )
    );
    fn test_field_filter_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).filter_offset) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause__bindgen_ty_1),
                "::",
                stringify!(filter_offset)
            )
        );
    }
    test_field_filter_offset();
    fn test_field_catch_class() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause__bindgen_ty_1>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).catch_class) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause__bindgen_ty_1),
                "::",
                stringify!(catch_class)
            )
        );
    }
    test_field_catch_class();
}
#[test]
fn bindgen_test_layout_MonoExceptionClause() {
    assert_eq!(
        ::std::mem::size_of::<MonoExceptionClause>(),
        32usize,
        concat!("Size of: ", stringify!(MonoExceptionClause))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoExceptionClause>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoExceptionClause))
    );
    fn test_field_flags() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).flags) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause),
                "::",
                stringify!(flags)
            )
        );
    }
    test_field_flags();
    fn test_field_try_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).try_offset) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause),
                "::",
                stringify!(try_offset)
            )
        );
    }
    test_field_try_offset();
    fn test_field_try_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).try_len) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause),
                "::",
                stringify!(try_len)
            )
        );
    }
    test_field_try_len();
    fn test_field_handler_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).handler_offset) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause),
                "::",
                stringify!(handler_offset)
            )
        );
    }
    test_field_handler_offset();
    fn test_field_handler_len() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).handler_len) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause),
                "::",
                stringify!(handler_len)
            )
        );
    }
    test_field_handler_len();
    fn test_field_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoExceptionClause>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoExceptionClause),
                "::",
                stringify!(data)
            )
        );
    }
    test_field_data();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoType {
    _unused: [u8; 0],
}
pub type MonoType = _MonoType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoGenericInst {
    _unused: [u8; 0],
}
pub type MonoGenericInst = _MonoGenericInst;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoGenericClass {
    _unused: [u8; 0],
}
pub type MonoGenericClass = _MonoGenericClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoGenericContext {
    _unused: [u8; 0],
}
pub type MonoGenericContext = _MonoGenericContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoGenericContainer {
    _unused: [u8; 0],
}
pub type MonoGenericContainer = _MonoGenericContainer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoGenericParam {
    _unused: [u8; 0],
}
pub type MonoGenericParam = _MonoGenericParam;
pub type MonoArrayType = _MonoArrayType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoMethodSignature {
    _unused: [u8; 0],
}
pub type MonoMethodSignature = _MonoMethodSignature;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct invalid_name {
    _unused: [u8; 0],
}
pub type MonoGenericMethod = invalid_name;
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct MonoCustomMod {
    pub _bitfield_align_1: [u32; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_MonoCustomMod() {
    assert_eq!(
        ::std::mem::size_of::<MonoCustomMod>(),
        4usize,
        concat!("Size of: ", stringify!(MonoCustomMod))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoCustomMod>(),
        4usize,
        concat!("Alignment of ", stringify!(MonoCustomMod))
    );
}
impl MonoCustomMod {
    #[inline]
    pub fn required(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_required(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn token(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 31u8) as u32) }
    }
    #[inline]
    pub fn set_token(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 31u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        required: ::std::os::raw::c_uint,
        token: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let required: u32 = unsafe { ::std::mem::transmute(required) };
            required as u64
        });
        __bindgen_bitfield_unit.set(1usize, 31u8, {
            let token: u32 = unsafe { ::std::mem::transmute(token) };
            token as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoCustomModContainer {
    pub count: u8,
    pub image: *mut MonoImage,
    pub modifiers: [MonoCustomMod; 1usize],
}
#[test]
fn bindgen_test_layout__MonoCustomModContainer() {
    assert_eq!(
        ::std::mem::size_of::<_MonoCustomModContainer>(),
        24usize,
        concat!("Size of: ", stringify!(_MonoCustomModContainer))
    );
    assert_eq!(
        ::std::mem::align_of::<_MonoCustomModContainer>(),
        8usize,
        concat!("Alignment of ", stringify!(_MonoCustomModContainer))
    );
    fn test_field_count() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoCustomModContainer>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).count) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoCustomModContainer),
                "::",
                stringify!(count)
            )
        );
    }
    test_field_count();
    fn test_field_image() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoCustomModContainer>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).image) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoCustomModContainer),
                "::",
                stringify!(image)
            )
        );
    }
    test_field_image();
    fn test_field_modifiers() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoCustomModContainer>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).modifiers) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoCustomModContainer),
                "::",
                stringify!(modifiers)
            )
        );
    }
    test_field_modifiers();
}
pub type MonoCustomModContainer = _MonoCustomModContainer;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoArrayType {
    pub eklass: *mut MonoClass,
    pub rank: u8,
    pub numsizes: u8,
    pub numlobounds: u8,
    pub sizes: *mut ::std::os::raw::c_int,
    pub lobounds: *mut ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__MonoArrayType() {
    assert_eq!(
        ::std::mem::size_of::<_MonoArrayType>(),
        32usize,
        concat!("Size of: ", stringify!(_MonoArrayType))
    );
    assert_eq!(
        ::std::mem::align_of::<_MonoArrayType>(),
        8usize,
        concat!("Alignment of ", stringify!(_MonoArrayType))
    );
    fn test_field_eklass() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoArrayType>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).eklass) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoArrayType),
                "::",
                stringify!(eklass)
            )
        );
    }
    test_field_eklass();
    fn test_field_rank() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoArrayType>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).rank) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoArrayType),
                "::",
                stringify!(rank)
            )
        );
    }
    test_field_rank();
    fn test_field_numsizes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoArrayType>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).numsizes) as usize - ptr as usize
            },
            9usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoArrayType),
                "::",
                stringify!(numsizes)
            )
        );
    }
    test_field_numsizes();
    fn test_field_numlobounds() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoArrayType>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).numlobounds) as usize - ptr as usize
            },
            10usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoArrayType),
                "::",
                stringify!(numlobounds)
            )
        );
    }
    test_field_numlobounds();
    fn test_field_sizes() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoArrayType>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).sizes) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoArrayType),
                "::",
                stringify!(sizes)
            )
        );
    }
    test_field_sizes();
    fn test_field_lobounds() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoArrayType>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).lobounds) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoArrayType),
                "::",
                stringify!(lobounds)
            )
        );
    }
    test_field_lobounds();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoMethodHeader {
    _unused: [u8; 0],
}
pub type MonoMethodHeader = _MonoMethodHeader;
pub const MonoParseTypeMode_MONO_PARSE_TYPE: MonoParseTypeMode = 0;
pub const MonoParseTypeMode_MONO_PARSE_MOD_TYPE: MonoParseTypeMode = 1;
pub const MonoParseTypeMode_MONO_PARSE_LOCAL: MonoParseTypeMode = 2;
pub const MonoParseTypeMode_MONO_PARSE_PARAM: MonoParseTypeMode = 3;
pub const MonoParseTypeMode_MONO_PARSE_RET: MonoParseTypeMode = 4;
pub const MonoParseTypeMode_MONO_PARSE_FIELD: MonoParseTypeMode = 5;
pub type MonoParseTypeMode = ::std::os::raw::c_uint;
extern "C" {
    pub fn mono_type_is_byref(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_type_get_type(type_: *mut MonoType) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_type_get_signature(type_: *mut MonoType) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_type_get_class(type_: *mut MonoType) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_type_get_array_type(type_: *mut MonoType) -> *mut MonoArrayType;
}
extern "C" {
    pub fn mono_type_get_ptr_type(type_: *mut MonoType) -> *mut MonoType;
}
extern "C" {
    pub fn mono_type_get_modifiers(
        type_: *mut MonoType,
        is_required: *mut mono_bool,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_type_is_struct(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_type_is_void(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_type_is_pointer(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_type_is_reference(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_type_is_generic_parameter(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_signature_get_return_type(sig: *mut MonoMethodSignature) -> *mut MonoType;
}
extern "C" {
    pub fn mono_signature_get_params(
        sig: *mut MonoMethodSignature,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_signature_get_param_count(sig: *mut MonoMethodSignature) -> u32;
}
extern "C" {
    pub fn mono_signature_get_call_conv(sig: *mut MonoMethodSignature) -> u32;
}
extern "C" {
    pub fn mono_signature_vararg_start(sig: *mut MonoMethodSignature) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_signature_is_instance(sig: *mut MonoMethodSignature) -> mono_bool;
}
extern "C" {
    pub fn mono_signature_explicit_this(sig: *mut MonoMethodSignature) -> mono_bool;
}
extern "C" {
    pub fn mono_signature_param_is_out(
        sig: *mut MonoMethodSignature,
        param_num: ::std::os::raw::c_int,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_metadata_parse_typedef_or_ref(
        m: *mut MonoImage,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_parse_custom_mod(
        m: *mut MonoImage,
        dest: *mut MonoCustomMod,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_metadata_parse_array(
        m: *mut MonoImage,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> *mut MonoArrayType;
}
extern "C" {
    pub fn mono_metadata_free_array(array: *mut MonoArrayType);
}
extern "C" {
    pub fn mono_metadata_parse_type(
        m: *mut MonoImage,
        mode: MonoParseTypeMode,
        opt_attrs: ::std::os::raw::c_short,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_metadata_parse_param(
        m: *mut MonoImage,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_metadata_parse_field_type(
        m: *mut MonoImage,
        field_flags: ::std::os::raw::c_short,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_type_create_from_typespec(image: *mut MonoImage, type_spec: u32) -> *mut MonoType;
}
extern "C" {
    pub fn mono_metadata_free_type(type_: *mut MonoType);
}
extern "C" {
    pub fn mono_type_size(
        type_: *mut MonoType,
        alignment: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_type_stack_size(
        type_: *mut MonoType,
        alignment: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_type_generic_inst_is_valuetype(type_: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_metadata_generic_class_is_valuetype(gclass: *mut MonoGenericClass) -> mono_bool;
}
extern "C" {
    pub fn mono_metadata_type_hash(t1: *mut MonoType) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn mono_metadata_type_equal(t1: *mut MonoType, t2: *mut MonoType) -> mono_bool;
}
extern "C" {
    pub fn mono_metadata_signature_alloc(
        image: *mut MonoImage,
        nparams: u32,
    ) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_metadata_signature_dup(sig: *mut MonoMethodSignature) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_metadata_parse_signature(
        image: *mut MonoImage,
        token: u32,
    ) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_metadata_parse_method_signature(
        m: *mut MonoImage,
        def: ::std::os::raw::c_int,
        ptr: *const ::std::os::raw::c_char,
        rptr: *mut *const ::std::os::raw::c_char,
    ) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_metadata_free_method_signature(method: *mut MonoMethodSignature);
}
extern "C" {
    pub fn mono_metadata_signature_equal(
        sig1: *mut MonoMethodSignature,
        sig2: *mut MonoMethodSignature,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_signature_hash(sig: *mut MonoMethodSignature) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn mono_metadata_parse_mh(
        m: *mut MonoImage,
        ptr: *const ::std::os::raw::c_char,
    ) -> *mut MonoMethodHeader;
}
extern "C" {
    pub fn mono_metadata_free_mh(mh: *mut MonoMethodHeader);
}
extern "C" {
    pub fn mono_method_header_get_code(
        header: *mut MonoMethodHeader,
        code_size: *mut u32,
        max_stack: *mut u32,
    ) -> *const ::std::os::raw::c_uchar;
}
extern "C" {
    pub fn mono_method_header_get_locals(
        header: *mut MonoMethodHeader,
        num_locals: *mut u32,
        init_locals: *mut mono_bool,
    ) -> *mut *mut MonoType;
}
extern "C" {
    pub fn mono_method_header_get_num_clauses(
        header: *mut MonoMethodHeader,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_method_header_get_clauses(
        header: *mut MonoMethodHeader,
        method: *mut MonoMethod,
        iter: *mut *mut ::std::os::raw::c_void,
        clause: *mut MonoExceptionClause,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_type_to_unmanaged(
        type_: *mut MonoType,
        mspec: *mut MonoMarshalSpec,
        as_field: mono_bool,
        unicode: mono_bool,
        conv: *mut MonoMarshalConv,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_token_from_dor(dor_index: u32) -> u32;
}
extern "C" {
    pub fn mono_guid_to_string(guid: *const u8) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_guid_to_string_minimal(guid: *const u8) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_metadata_declsec_from_index(meta: *mut MonoImage, idx: u32) -> u32;
}
extern "C" {
    pub fn mono_metadata_translate_token_index(
        image: *mut MonoImage,
        table: ::std::os::raw::c_int,
        idx: u32,
    ) -> u32;
}
extern "C" {
    pub fn mono_metadata_decode_table_row(
        image: *mut MonoImage,
        table: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_int,
        res: *mut u32,
        res_size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mono_metadata_decode_table_row_col(
        image: *mut MonoImage,
        table: ::std::os::raw::c_int,
        idx: ::std::os::raw::c_int,
        col: ::std::os::raw::c_uint,
    ) -> u32;
}
pub type MonoStackWalk = ::std::option::Option<
    unsafe extern "C" fn(
        method: *mut MonoMethod,
        native_offset: i32,
        il_offset: i32,
        managed: mono_bool,
        data: *mut ::std::os::raw::c_void,
    ) -> mono_bool,
>;
extern "C" {
    pub fn mono_get_method(
        image: *mut MonoImage,
        token: u32,
        klass: *mut MonoClass,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_get_method_full(
        image: *mut MonoImage,
        token: u32,
        klass: *mut MonoClass,
        context: *mut MonoGenericContext,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_get_method_constrained(
        image: *mut MonoImage,
        token: u32,
        constrained_class: *mut MonoClass,
        context: *mut MonoGenericContext,
        cil_method: *mut *mut MonoMethod,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_free_method(method: *mut MonoMethod);
}
extern "C" {
    pub fn mono_method_get_signature_full(
        method: *mut MonoMethod,
        image: *mut MonoImage,
        token: u32,
        context: *mut MonoGenericContext,
    ) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_method_get_signature(
        method: *mut MonoMethod,
        image: *mut MonoImage,
        token: u32,
    ) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_method_signature(method: *mut MonoMethod) -> *mut MonoMethodSignature;
}
extern "C" {
    pub fn mono_method_get_header(method: *mut MonoMethod) -> *mut MonoMethodHeader;
}
extern "C" {
    pub fn mono_method_get_name(method: *mut MonoMethod) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_method_get_class(method: *mut MonoMethod) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_method_get_token(method: *mut MonoMethod) -> u32;
}
extern "C" {
    pub fn mono_method_get_flags(method: *mut MonoMethod, iflags: *mut u32) -> u32;
}
extern "C" {
    pub fn mono_method_get_index(method: *mut MonoMethod) -> u32;
}
extern "C" {
    pub fn mono_add_internal_call(
        name: *const ::std::os::raw::c_char,
        method: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_dangerous_add_raw_internal_call(
        name: *const ::std::os::raw::c_char,
        method: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_lookup_internal_call(method: *mut MonoMethod) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_lookup_icall_symbol(m: *mut MonoMethod) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_dllmap_insert(
        assembly: *mut MonoImage,
        dll: *const ::std::os::raw::c_char,
        func: *const ::std::os::raw::c_char,
        tdll: *const ::std::os::raw::c_char,
        tfunc: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mono_lookup_pinvoke_call(
        method: *mut MonoMethod,
        exc_class: *mut *const ::std::os::raw::c_char,
        exc_arg: *mut *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_method_get_param_names(
        method: *mut MonoMethod,
        names: *mut *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mono_method_get_param_token(method: *mut MonoMethod, idx: ::std::os::raw::c_int) -> u32;
}
extern "C" {
    pub fn mono_method_get_marshal_info(method: *mut MonoMethod, mspecs: *mut *mut MonoMarshalSpec);
}
extern "C" {
    pub fn mono_method_has_marshal_info(method: *mut MonoMethod) -> mono_bool;
}
extern "C" {
    pub fn mono_method_get_last_managed() -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_stack_walk(func: MonoStackWalk, user_data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn mono_stack_walk_no_il(func: MonoStackWalk, user_data: *mut ::std::os::raw::c_void);
}
pub type MonoStackWalkAsyncSafe = ::std::option::Option<
    unsafe extern "C" fn(
        method: *mut MonoMethod,
        domain: *mut MonoDomain,
        base_address: *mut ::std::os::raw::c_void,
        offset: ::std::os::raw::c_int,
        data: *mut ::std::os::raw::c_void,
    ) -> mono_bool,
>;
extern "C" {
    pub fn mono_stack_walk_async_safe(
        func: MonoStackWalkAsyncSafe,
        initial_sig_context: *mut ::std::os::raw::c_void,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_method_get_header_checked(
        method: *mut MonoMethod,
        error: *mut MonoError,
    ) -> *mut MonoMethodHeader;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoVTable {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoClassField {
    _unused: [u8; 0],
}
pub type MonoClassField = _MonoClassField;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoProperty {
    _unused: [u8; 0],
}
pub type MonoProperty = _MonoProperty;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoEvent {
    _unused: [u8; 0],
}
pub type MonoEvent = _MonoEvent;
pub const MonoTypeNameFormat_MONO_TYPE_NAME_FORMAT_IL: MonoTypeNameFormat = 0;
pub const MonoTypeNameFormat_MONO_TYPE_NAME_FORMAT_REFLECTION: MonoTypeNameFormat = 1;
pub const MonoTypeNameFormat_MONO_TYPE_NAME_FORMAT_FULL_NAME: MonoTypeNameFormat = 2;
pub const MonoTypeNameFormat_MONO_TYPE_NAME_FORMAT_ASSEMBLY_QUALIFIED: MonoTypeNameFormat = 3;
pub type MonoTypeNameFormat = ::std::os::raw::c_uint;
extern "C" {
    pub fn mono_class_get(image: *mut MonoImage, type_token: u32) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_get_full(
        image: *mut MonoImage,
        type_token: u32,
        context: *mut MonoGenericContext,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_init(klass: *mut MonoClass) -> mono_bool;
}
extern "C" {
    pub fn mono_class_vtable(domain: *mut MonoDomain, klass: *mut MonoClass) -> *mut MonoVTable;
}
extern "C" {
    pub fn mono_class_from_name(
        image: *mut MonoImage,
        name_space: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_from_name_case(
        image: *mut MonoImage,
        name_space: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_get_method_from_name_flags(
        klass: *mut MonoClass,
        name: *const ::std::os::raw::c_char,
        param_count: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_class_from_typeref(image: *mut MonoImage, type_token: u32) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_from_typeref_checked(
        image: *mut MonoImage,
        type_token: u32,
        error: *mut MonoError,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_from_generic_parameter(
        param: *mut MonoGenericParam,
        image: *mut MonoImage,
        is_mvar: mono_bool,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_inflate_generic_type(
        type_: *mut MonoType,
        context: *mut MonoGenericContext,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_class_inflate_generic_method(
        method: *mut MonoMethod,
        context: *mut MonoGenericContext,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_get_inflated_method(method: *mut MonoMethod) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_field_from_token(
        image: *mut MonoImage,
        token: u32,
        retklass: *mut *mut MonoClass,
        context: *mut MonoGenericContext,
    ) -> *mut MonoClassField;
}
extern "C" {
    pub fn mono_bounded_array_class_get(
        element_class: *mut MonoClass,
        rank: u32,
        bounded: mono_bool,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_array_class_get(element_class: *mut MonoClass, rank: u32) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_ptr_class_get(type_: *mut MonoType) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_get_field(klass: *mut MonoClass, field_token: u32) -> *mut MonoClassField;
}
extern "C" {
    pub fn mono_class_get_field_from_name(
        klass: *mut MonoClass,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoClassField;
}
extern "C" {
    pub fn mono_class_get_field_token(field: *mut MonoClassField) -> u32;
}
extern "C" {
    pub fn mono_class_get_event_token(event: *mut MonoEvent) -> u32;
}
extern "C" {
    pub fn mono_class_get_property_from_name(
        klass: *mut MonoClass,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoProperty;
}
extern "C" {
    pub fn mono_class_get_property_token(prop: *mut MonoProperty) -> u32;
}
extern "C" {
    pub fn mono_array_element_size(ac: *mut MonoClass) -> i32;
}
extern "C" {
    pub fn mono_class_instance_size(klass: *mut MonoClass) -> i32;
}
extern "C" {
    pub fn mono_class_array_element_size(klass: *mut MonoClass) -> i32;
}
extern "C" {
    pub fn mono_class_data_size(klass: *mut MonoClass) -> i32;
}
extern "C" {
    pub fn mono_class_value_size(klass: *mut MonoClass, align: *mut u32) -> i32;
}
extern "C" {
    pub fn mono_class_min_align(klass: *mut MonoClass) -> i32;
}
extern "C" {
    pub fn mono_class_from_mono_type(type_: *mut MonoType) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_is_subclass_of(
        klass: *mut MonoClass,
        klassc: *mut MonoClass,
        check_interfaces: mono_bool,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_class_is_assignable_from(
        klass: *mut MonoClass,
        oklass: *mut MonoClass,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_ldtoken(
        image: *mut MonoImage,
        token: u32,
        retclass: *mut *mut MonoClass,
        context: *mut MonoGenericContext,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_type_get_name_full(
        type_: *mut MonoType,
        format: MonoTypeNameFormat,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_type_get_name(type_: *mut MonoType) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_type_get_underlying_type(type_: *mut MonoType) -> *mut MonoType;
}
extern "C" {
    pub fn mono_class_get_image(klass: *mut MonoClass) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_class_get_element_class(klass: *mut MonoClass) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_is_valuetype(klass: *mut MonoClass) -> mono_bool;
}
extern "C" {
    pub fn mono_class_is_enum(klass: *mut MonoClass) -> mono_bool;
}
extern "C" {
    pub fn mono_class_enum_basetype(klass: *mut MonoClass) -> *mut MonoType;
}
extern "C" {
    pub fn mono_class_get_parent(klass: *mut MonoClass) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_get_nesting_type(klass: *mut MonoClass) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_get_rank(klass: *mut MonoClass) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_class_get_flags(klass: *mut MonoClass) -> u32;
}
extern "C" {
    pub fn mono_class_get_name(klass: *mut MonoClass) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_class_get_namespace(klass: *mut MonoClass) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_class_get_type(klass: *mut MonoClass) -> *mut MonoType;
}
extern "C" {
    pub fn mono_class_get_type_token(klass: *mut MonoClass) -> u32;
}
extern "C" {
    pub fn mono_class_get_byref_type(klass: *mut MonoClass) -> *mut MonoType;
}
extern "C" {
    pub fn mono_class_num_fields(klass: *mut MonoClass) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_class_num_methods(klass: *mut MonoClass) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_class_num_properties(klass: *mut MonoClass) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_class_num_events(klass: *mut MonoClass) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_class_get_fields(
        klass: *mut MonoClass,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoClassField;
}
extern "C" {
    pub fn mono_class_get_methods(
        klass: *mut MonoClass,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_class_get_properties(
        klass: *mut MonoClass,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoProperty;
}
extern "C" {
    pub fn mono_class_get_events(
        klass: *mut MonoClass,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoEvent;
}
extern "C" {
    pub fn mono_class_get_interfaces(
        klass: *mut MonoClass,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_get_nested_types(
        klass: *mut MonoClass,
        iter: *mut *mut ::std::os::raw::c_void,
    ) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_class_is_delegate(klass: *mut MonoClass) -> mono_bool;
}
extern "C" {
    pub fn mono_class_implements_interface(
        klass: *mut MonoClass,
        iface: *mut MonoClass,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_field_get_name(field: *mut MonoClassField) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_field_get_type(field: *mut MonoClassField) -> *mut MonoType;
}
extern "C" {
    pub fn mono_field_get_parent(field: *mut MonoClassField) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_field_get_flags(field: *mut MonoClassField) -> u32;
}
extern "C" {
    pub fn mono_field_get_offset(field: *mut MonoClassField) -> u32;
}
extern "C" {
    pub fn mono_field_get_data(field: *mut MonoClassField) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_property_get_name(prop: *mut MonoProperty) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_property_get_set_method(prop: *mut MonoProperty) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_property_get_get_method(prop: *mut MonoProperty) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_property_get_parent(prop: *mut MonoProperty) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_property_get_flags(prop: *mut MonoProperty) -> u32;
}
extern "C" {
    pub fn mono_event_get_name(event: *mut MonoEvent) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_event_get_add_method(event: *mut MonoEvent) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_event_get_remove_method(event: *mut MonoEvent) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_event_get_raise_method(event: *mut MonoEvent) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_event_get_parent(event: *mut MonoEvent) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_event_get_flags(event: *mut MonoEvent) -> u32;
}
extern "C" {
    pub fn mono_class_get_method_from_name(
        klass: *mut MonoClass,
        name: *const ::std::os::raw::c_char,
        param_count: ::std::os::raw::c_int,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_class_name_from_token(
        image: *mut MonoImage,
        type_token: u32,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_method_can_access_field(
        method: *mut MonoMethod,
        field: *mut MonoClassField,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_method_can_access_method(
        method: *mut MonoMethod,
        called: *mut MonoMethod,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_class_is_nullable(klass: *mut MonoClass) -> mono_bool;
}
extern "C" {
    pub fn mono_class_get_nullable_param(klass: *mut MonoClass) -> *mut MonoClass;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoString {
    _unused: [u8; 0],
}
pub type MonoString = _MonoString;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoArray {
    _unused: [u8; 0],
}
pub type MonoArray = _MonoArray;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionMethod {
    _unused: [u8; 0],
}
pub type MonoReflectionMethod = _MonoReflectionMethod;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionModule {
    _unused: [u8; 0],
}
pub type MonoReflectionModule = _MonoReflectionModule;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionField {
    _unused: [u8; 0],
}
pub type MonoReflectionField = _MonoReflectionField;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionProperty {
    _unused: [u8; 0],
}
pub type MonoReflectionProperty = _MonoReflectionProperty;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionEvent {
    _unused: [u8; 0],
}
pub type MonoReflectionEvent = _MonoReflectionEvent;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionType {
    _unused: [u8; 0],
}
pub type MonoReflectionType = _MonoReflectionType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoDelegate {
    _unused: [u8; 0],
}
pub type MonoDelegate = _MonoDelegate;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoThreadsSync {
    _unused: [u8; 0],
}
pub type MonoThreadsSync = _MonoThreadsSync;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoThread {
    _unused: [u8; 0],
}
pub type MonoThread = _MonoThread;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoDynamicAssembly {
    _unused: [u8; 0],
}
pub type MonoDynamicAssembly = _MonoDynamicAssembly;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoDynamicImage {
    _unused: [u8; 0],
}
pub type MonoDynamicImage = _MonoDynamicImage;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReflectionMethodBody {
    _unused: [u8; 0],
}
pub type MonoReflectionMethodBody = _MonoReflectionMethodBody;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoAppContext {
    _unused: [u8; 0],
}
pub type MonoAppContext = _MonoAppContext;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoObject {
    pub vtable: *mut MonoVTable,
    pub synchronisation: *mut MonoThreadsSync,
}
#[test]
fn bindgen_test_layout__MonoObject() {
    assert_eq!(
        ::std::mem::size_of::<_MonoObject>(),
        16usize,
        concat!("Size of: ", stringify!(_MonoObject))
    );
    assert_eq!(
        ::std::mem::align_of::<_MonoObject>(),
        8usize,
        concat!("Alignment of ", stringify!(_MonoObject))
    );
    fn test_field_vtable() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoObject>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).vtable) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoObject),
                "::",
                stringify!(vtable)
            )
        );
    }
    test_field_vtable();
    fn test_field_synchronisation() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<_MonoObject>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).synchronisation) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(_MonoObject),
                "::",
                stringify!(synchronisation)
            )
        );
    }
    test_field_synchronisation();
}
pub type MonoInvokeFunc = ::std::option::Option<
    unsafe extern "C" fn(
        method: *mut MonoMethod,
        obj: *mut ::std::os::raw::c_void,
        params: *mut *mut ::std::os::raw::c_void,
        exc: *mut *mut MonoObject,
        error: *mut MonoError,
    ) -> *mut MonoObject,
>;
pub type MonoCompileFunc = ::std::option::Option<
    unsafe extern "C" fn(method: *mut MonoMethod) -> *mut ::std::os::raw::c_void,
>;
pub type MonoMainThreadFunc =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
extern "C" {
    pub fn mono_string_chars(s: *mut MonoString) -> *mut mono_unichar2;
}
extern "C" {
    pub fn mono_string_length(s: *mut MonoString) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_object_new(domain: *mut MonoDomain, klass: *mut MonoClass) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_new_specific(vtable: *mut MonoVTable) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_new_fast(vtable: *mut MonoVTable) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_new_alloc_specific(vtable: *mut MonoVTable) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_new_from_token(
        domain: *mut MonoDomain,
        image: *mut MonoImage,
        token: u32,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_array_new(
        domain: *mut MonoDomain,
        eclass: *mut MonoClass,
        n: usize,
    ) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_array_new_full(
        domain: *mut MonoDomain,
        array_class: *mut MonoClass,
        lengths: *mut usize,
        lower_bounds: *mut isize,
    ) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_array_new_specific(vtable: *mut MonoVTable, n: usize) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_array_clone(array: *mut MonoArray) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_array_addr_with_size(
        array: *mut MonoArray,
        size: ::std::os::raw::c_int,
        idx: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_array_length(array: *mut MonoArray) -> usize;
}
extern "C" {
    pub fn mono_string_empty(domain: *mut MonoDomain) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_empty_wrapper() -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_new_utf16(
        domain: *mut MonoDomain,
        text: *const mono_unichar2,
        len: i32,
    ) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_new_size(domain: *mut MonoDomain, len: i32) -> *mut MonoString;
}
extern "C" {
    pub fn mono_ldstr(
        domain: *mut MonoDomain,
        image: *mut MonoImage,
        str_index: u32,
    ) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_is_interned(str_: *mut MonoString) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_intern(str_: *mut MonoString) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_new(
        domain: *mut MonoDomain,
        text: *const ::std::os::raw::c_char,
    ) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_new_wrapper(text: *const ::std::os::raw::c_char) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_new_len(
        domain: *mut MonoDomain,
        text: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_uint,
    ) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_new_utf32(
        domain: *mut MonoDomain,
        text: *const mono_unichar4,
        len: i32,
    ) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_to_utf8(string_obj: *mut MonoString) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_string_to_utf8_checked(
        string_obj: *mut MonoString,
        error: *mut MonoError,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_string_to_utf16(string_obj: *mut MonoString) -> *mut mono_unichar2;
}
extern "C" {
    pub fn mono_string_to_utf32(string_obj: *mut MonoString) -> *mut mono_unichar4;
}
extern "C" {
    pub fn mono_string_from_utf16(data: *mut mono_unichar2) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_from_utf32(data: *mut mono_unichar4) -> *mut MonoString;
}
extern "C" {
    pub fn mono_string_equal(s1: *mut MonoString, s2: *mut MonoString) -> mono_bool;
}
extern "C" {
    pub fn mono_string_hash(s: *mut MonoString) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn mono_object_hash(obj: *mut MonoObject) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_object_to_string(
        obj: *mut MonoObject,
        exc: *mut *mut MonoObject,
    ) -> *mut MonoString;
}
extern "C" {
    pub fn mono_value_box(
        domain: *mut MonoDomain,
        klass: *mut MonoClass,
        val: *mut ::std::os::raw::c_void,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_value_copy(
        dest: *mut ::std::os::raw::c_void,
        src: *mut ::std::os::raw::c_void,
        klass: *mut MonoClass,
    );
}
extern "C" {
    pub fn mono_value_copy_array(
        dest: *mut MonoArray,
        dest_idx: ::std::os::raw::c_int,
        src: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mono_object_get_vtable(obj: *mut MonoObject) -> *mut MonoVTable;
}
extern "C" {
    pub fn mono_object_get_domain(obj: *mut MonoObject) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_object_get_class(obj: *mut MonoObject) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_object_unbox(obj: *mut MonoObject) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_object_clone(obj: *mut MonoObject) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_isinst(obj: *mut MonoObject, klass: *mut MonoClass) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_isinst_mbyref(
        obj: *mut MonoObject,
        klass: *mut MonoClass,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_object_castclass_mbyref(
        obj: *mut MonoObject,
        klass: *mut MonoClass,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_monitor_try_enter(obj: *mut MonoObject, ms: u32) -> mono_bool;
}
extern "C" {
    pub fn mono_monitor_enter(obj: *mut MonoObject) -> mono_bool;
}
extern "C" {
    pub fn mono_monitor_enter_v4(obj: *mut MonoObject, lock_taken: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn mono_object_get_size(o: *mut MonoObject) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn mono_monitor_exit(obj: *mut MonoObject);
}
extern "C" {
    pub fn mono_raise_exception(ex: *mut MonoException);
}
extern "C" {
    pub fn mono_runtime_set_pending_exception(
        exc: *mut MonoException,
        overwrite: mono_bool,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_reraise_exception(ex: *mut MonoException);
}
extern "C" {
    pub fn mono_runtime_object_init(this_obj: *mut MonoObject);
}
extern "C" {
    pub fn mono_runtime_class_init(vtable: *mut MonoVTable);
}
extern "C" {
    pub fn mono_vtable_domain(vtable: *mut MonoVTable) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_vtable_class(vtable: *mut MonoVTable) -> *mut MonoClass;
}
extern "C" {
    pub fn mono_object_get_virtual_method(
        obj: *mut MonoObject,
        method: *mut MonoMethod,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_runtime_invoke(
        method: *mut MonoMethod,
        obj: *mut ::std::os::raw::c_void,
        params: *mut *mut ::std::os::raw::c_void,
        exc: *mut *mut MonoObject,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_get_delegate_invoke(klass: *mut MonoClass) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_get_delegate_begin_invoke(klass: *mut MonoClass) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_get_delegate_end_invoke(klass: *mut MonoClass) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_runtime_delegate_invoke(
        delegate: *mut MonoObject,
        params: *mut *mut ::std::os::raw::c_void,
        exc: *mut *mut MonoObject,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_runtime_invoke_array(
        method: *mut MonoMethod,
        obj: *mut ::std::os::raw::c_void,
        params: *mut MonoArray,
        exc: *mut *mut MonoObject,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_method_get_unmanaged_thunk(method: *mut MonoMethod) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_runtime_get_main_args() -> *mut MonoArray;
}
extern "C" {
    pub fn mono_runtime_exec_managed_code(
        domain: *mut MonoDomain,
        main_func: MonoMainThreadFunc,
        main_args: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_runtime_run_main(
        method: *mut MonoMethod,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
        exc: *mut *mut MonoObject,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_runtime_exec_main(
        method: *mut MonoMethod,
        args: *mut MonoArray,
        exc: *mut *mut MonoObject,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_runtime_set_main_args(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_load_remote_field(
        this_obj: *mut MonoObject,
        klass: *mut MonoClass,
        field: *mut MonoClassField,
        res: *mut *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_load_remote_field_new(
        this_obj: *mut MonoObject,
        klass: *mut MonoClass,
        field: *mut MonoClassField,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_store_remote_field(
        this_obj: *mut MonoObject,
        klass: *mut MonoClass,
        field: *mut MonoClassField,
        val: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_store_remote_field_new(
        this_obj: *mut MonoObject,
        klass: *mut MonoClass,
        field: *mut MonoClassField,
        arg: *mut MonoObject,
    );
}
extern "C" {
    pub fn mono_unhandled_exception(exc: *mut MonoObject);
}
extern "C" {
    pub fn mono_print_unhandled_exception(exc: *mut MonoObject);
}
extern "C" {
    pub fn mono_compile_method(method: *mut MonoMethod) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_field_set_value(
        obj: *mut MonoObject,
        field: *mut MonoClassField,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_field_static_set_value(
        vt: *mut MonoVTable,
        field: *mut MonoClassField,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_field_get_value(
        obj: *mut MonoObject,
        field: *mut MonoClassField,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_field_static_get_value(
        vt: *mut MonoVTable,
        field: *mut MonoClassField,
        value: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_field_get_value_object(
        domain: *mut MonoDomain,
        field: *mut MonoClassField,
        obj: *mut MonoObject,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_property_set_value(
        prop: *mut MonoProperty,
        obj: *mut ::std::os::raw::c_void,
        params: *mut *mut ::std::os::raw::c_void,
        exc: *mut *mut MonoObject,
    );
}
extern "C" {
    pub fn mono_property_get_value(
        prop: *mut MonoProperty,
        obj: *mut ::std::os::raw::c_void,
        params: *mut *mut ::std::os::raw::c_void,
        exc: *mut *mut MonoObject,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_gchandle_new(obj: *mut MonoObject, pinned: mono_bool) -> u32;
}
extern "C" {
    pub fn mono_gchandle_new_weakref(obj: *mut MonoObject, track_resurrection: mono_bool) -> u32;
}
extern "C" {
    pub fn mono_gchandle_get_target(gchandle: u32) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_gchandle_free(gchandle: u32);
}
pub type mono_reference_queue_callback =
    ::std::option::Option<unsafe extern "C" fn(user_data: *mut ::std::os::raw::c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoReferenceQueue {
    _unused: [u8; 0],
}
pub type MonoReferenceQueue = _MonoReferenceQueue;
extern "C" {
    pub fn mono_gc_reference_queue_new(
        callback: mono_reference_queue_callback,
    ) -> *mut MonoReferenceQueue;
}
extern "C" {
    pub fn mono_gc_reference_queue_free(queue: *mut MonoReferenceQueue);
}
extern "C" {
    pub fn mono_gc_reference_queue_add(
        queue: *mut MonoReferenceQueue,
        obj: *mut MonoObject,
        user_data: *mut ::std::os::raw::c_void,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_gc_wbarrier_set_field(
        obj: *mut MonoObject,
        field_ptr: *mut ::std::os::raw::c_void,
        value: *mut MonoObject,
    );
}
extern "C" {
    pub fn mono_gc_wbarrier_set_arrayref(
        arr: *mut MonoArray,
        slot_ptr: *mut ::std::os::raw::c_void,
        value: *mut MonoObject,
    );
}
extern "C" {
    pub fn mono_gc_wbarrier_arrayref_copy(
        dest_ptr: *mut ::std::os::raw::c_void,
        src_ptr: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mono_gc_wbarrier_generic_store(ptr: *mut ::std::os::raw::c_void, value: *mut MonoObject);
}
extern "C" {
    pub fn mono_gc_wbarrier_generic_store_atomic(
        ptr: *mut ::std::os::raw::c_void,
        value: *mut MonoObject,
    );
}
extern "C" {
    pub fn mono_gc_wbarrier_generic_nostore(ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn mono_gc_wbarrier_value_copy(
        dest: *mut ::std::os::raw::c_void,
        src: *mut ::std::os::raw::c_void,
        count: ::std::os::raw::c_int,
        klass: *mut MonoClass,
    );
}
extern "C" {
    pub fn mono_gc_wbarrier_object_copy(obj: *mut MonoObject, src: *mut MonoObject);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoTypeNameParse {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoCustomAttrEntry {
    pub ctor: *mut MonoMethod,
    pub data_size: u32,
    pub data: *const mono_byte,
}
#[test]
fn bindgen_test_layout_MonoCustomAttrEntry() {
    assert_eq!(
        ::std::mem::size_of::<MonoCustomAttrEntry>(),
        24usize,
        concat!("Size of: ", stringify!(MonoCustomAttrEntry))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoCustomAttrEntry>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoCustomAttrEntry))
    );
    fn test_field_ctor() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrEntry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).ctor) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrEntry),
                "::",
                stringify!(ctor)
            )
        );
    }
    test_field_ctor();
    fn test_field_data_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrEntry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data_size) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrEntry),
                "::",
                stringify!(data_size)
            )
        );
    }
    test_field_data_size();
    fn test_field_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrEntry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrEntry),
                "::",
                stringify!(data)
            )
        );
    }
    test_field_data();
}
#[repr(C)]
#[derive(Debug)]
pub struct MonoCustomAttrInfo {
    pub num_attrs: ::std::os::raw::c_int,
    pub cached: ::std::os::raw::c_int,
    pub image: *mut MonoImage,
    pub attrs: __IncompleteArrayField<MonoCustomAttrEntry>,
}
#[test]
fn bindgen_test_layout_MonoCustomAttrInfo() {
    assert_eq!(
        ::std::mem::size_of::<MonoCustomAttrInfo>(),
        16usize,
        concat!("Size of: ", stringify!(MonoCustomAttrInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoCustomAttrInfo>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoCustomAttrInfo))
    );
    fn test_field_num_attrs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrInfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).num_attrs) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrInfo),
                "::",
                stringify!(num_attrs)
            )
        );
    }
    test_field_num_attrs();
    fn test_field_cached() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrInfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).cached) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrInfo),
                "::",
                stringify!(cached)
            )
        );
    }
    test_field_cached();
    fn test_field_image() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrInfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).image) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrInfo),
                "::",
                stringify!(image)
            )
        );
    }
    test_field_image();
    fn test_field_attrs() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoCustomAttrInfo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).attrs) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoCustomAttrInfo),
                "::",
                stringify!(attrs)
            )
        );
    }
    test_field_attrs();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoReflectionMethodAux {
    pub param_names: *mut *mut ::std::os::raw::c_char,
    pub param_marshall: *mut *mut MonoMarshalSpec,
    pub param_cattr: *mut *mut MonoCustomAttrInfo,
    pub param_defaults: *mut *mut u8,
    pub param_default_types: *mut u32,
    pub dllentry: *mut ::std::os::raw::c_char,
    pub dll: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_MonoReflectionMethodAux() {
    assert_eq!(
        ::std::mem::size_of::<MonoReflectionMethodAux>(),
        56usize,
        concat!("Size of: ", stringify!(MonoReflectionMethodAux))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoReflectionMethodAux>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoReflectionMethodAux))
    );
    fn test_field_param_names() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).param_names) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(param_names)
            )
        );
    }
    test_field_param_names();
    fn test_field_param_marshall() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).param_marshall) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(param_marshall)
            )
        );
    }
    test_field_param_marshall();
    fn test_field_param_cattr() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).param_cattr) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(param_cattr)
            )
        );
    }
    test_field_param_cattr();
    fn test_field_param_defaults() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).param_defaults) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(param_defaults)
            )
        );
    }
    test_field_param_defaults();
    fn test_field_param_default_types() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).param_default_types) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(param_default_types)
            )
        );
    }
    test_field_param_default_types();
    fn test_field_dllentry() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).dllentry) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(dllentry)
            )
        );
    }
    test_field_dllentry();
    fn test_field_dll() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoReflectionMethodAux>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).dll) as usize - ptr as usize
            },
            48usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoReflectionMethodAux),
                "::",
                stringify!(dll)
            )
        );
    }
    test_field_dll();
}
pub const MonoResolveTokenError_ResolveTokenError_OutOfRange: MonoResolveTokenError = 0;
pub const MonoResolveTokenError_ResolveTokenError_BadTable: MonoResolveTokenError = 1;
pub const MonoResolveTokenError_ResolveTokenError_Other: MonoResolveTokenError = 2;
pub type MonoResolveTokenError = ::std::os::raw::c_uint;
extern "C" {
    pub fn mono_reflection_parse_type(
        name: *mut ::std::os::raw::c_char,
        info: *mut MonoTypeNameParse,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_reflection_get_type(
        image: *mut MonoImage,
        info: *mut MonoTypeNameParse,
        ignorecase: mono_bool,
        type_resolve: *mut mono_bool,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_reflection_free_type_info(info: *mut MonoTypeNameParse);
}
extern "C" {
    pub fn mono_reflection_type_from_name(
        name: *mut ::std::os::raw::c_char,
        image: *mut MonoImage,
    ) -> *mut MonoType;
}
extern "C" {
    pub fn mono_reflection_get_token(obj: *mut MonoObject) -> u32;
}
extern "C" {
    pub fn mono_assembly_get_object(
        domain: *mut MonoDomain,
        assembly: *mut MonoAssembly,
    ) -> *mut MonoReflectionAssembly;
}
extern "C" {
    pub fn mono_module_get_object(
        domain: *mut MonoDomain,
        image: *mut MonoImage,
    ) -> *mut MonoReflectionModule;
}
extern "C" {
    pub fn mono_module_file_get_object(
        domain: *mut MonoDomain,
        image: *mut MonoImage,
        table_index: ::std::os::raw::c_int,
    ) -> *mut MonoReflectionModule;
}
extern "C" {
    pub fn mono_type_get_object(
        domain: *mut MonoDomain,
        type_: *mut MonoType,
    ) -> *mut MonoReflectionType;
}
extern "C" {
    pub fn mono_method_get_object(
        domain: *mut MonoDomain,
        method: *mut MonoMethod,
        refclass: *mut MonoClass,
    ) -> *mut MonoReflectionMethod;
}
extern "C" {
    pub fn mono_field_get_object(
        domain: *mut MonoDomain,
        klass: *mut MonoClass,
        field: *mut MonoClassField,
    ) -> *mut MonoReflectionField;
}
extern "C" {
    pub fn mono_property_get_object(
        domain: *mut MonoDomain,
        klass: *mut MonoClass,
        property: *mut MonoProperty,
    ) -> *mut MonoReflectionProperty;
}
extern "C" {
    pub fn mono_event_get_object(
        domain: *mut MonoDomain,
        klass: *mut MonoClass,
        event: *mut MonoEvent,
    ) -> *mut MonoReflectionEvent;
}
extern "C" {
    pub fn mono_param_get_objects(
        domain: *mut MonoDomain,
        method: *mut MonoMethod,
    ) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_method_body_get_object(
        domain: *mut MonoDomain,
        method: *mut MonoMethod,
    ) -> *mut MonoReflectionMethodBody;
}
extern "C" {
    pub fn mono_get_dbnull_object(domain: *mut MonoDomain) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_reflection_get_custom_attrs_by_type(
        obj: *mut MonoObject,
        attr_klass: *mut MonoClass,
        error: *mut MonoError,
    ) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_reflection_get_custom_attrs(obj: *mut MonoObject) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_reflection_get_custom_attrs_data(obj: *mut MonoObject) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_reflection_get_custom_attrs_blob(
        assembly: *mut MonoReflectionAssembly,
        ctor: *mut MonoObject,
        ctorArgs: *mut MonoArray,
        properties: *mut MonoArray,
        porpValues: *mut MonoArray,
        fields: *mut MonoArray,
        fieldValues: *mut MonoArray,
    ) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_reflection_get_custom_attrs_info(obj: *mut MonoObject) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_construct(cinfo: *mut MonoCustomAttrInfo) -> *mut MonoArray;
}
extern "C" {
    pub fn mono_custom_attrs_from_index(image: *mut MonoImage, idx: u32)
        -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_method(method: *mut MonoMethod) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_class(klass: *mut MonoClass) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_assembly(assembly: *mut MonoAssembly) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_property(
        klass: *mut MonoClass,
        property: *mut MonoProperty,
    ) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_event(
        klass: *mut MonoClass,
        event: *mut MonoEvent,
    ) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_field(
        klass: *mut MonoClass,
        field: *mut MonoClassField,
    ) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_from_param(
        method: *mut MonoMethod,
        param: u32,
    ) -> *mut MonoCustomAttrInfo;
}
extern "C" {
    pub fn mono_custom_attrs_has_attr(
        ainfo: *mut MonoCustomAttrInfo,
        attr_klass: *mut MonoClass,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_custom_attrs_get_attr(
        ainfo: *mut MonoCustomAttrInfo,
        attr_klass: *mut MonoClass,
    ) -> *mut MonoObject;
}
extern "C" {
    pub fn mono_custom_attrs_free(ainfo: *mut MonoCustomAttrInfo);
}
pub const MONO_DECLSEC_FLAG_REQUEST: _bindgen_ty_65 = 1;
pub const MONO_DECLSEC_FLAG_DEMAND: _bindgen_ty_65 = 2;
pub const MONO_DECLSEC_FLAG_ASSERT: _bindgen_ty_65 = 4;
pub const MONO_DECLSEC_FLAG_DENY: _bindgen_ty_65 = 8;
pub const MONO_DECLSEC_FLAG_PERMITONLY: _bindgen_ty_65 = 16;
pub const MONO_DECLSEC_FLAG_LINKDEMAND: _bindgen_ty_65 = 32;
pub const MONO_DECLSEC_FLAG_INHERITANCEDEMAND: _bindgen_ty_65 = 64;
pub const MONO_DECLSEC_FLAG_REQUEST_MINIMUM: _bindgen_ty_65 = 128;
pub const MONO_DECLSEC_FLAG_REQUEST_OPTIONAL: _bindgen_ty_65 = 256;
pub const MONO_DECLSEC_FLAG_REQUEST_REFUSE: _bindgen_ty_65 = 512;
pub const MONO_DECLSEC_FLAG_PREJIT_GRANT: _bindgen_ty_65 = 1024;
pub const MONO_DECLSEC_FLAG_PREJIT_DENY: _bindgen_ty_65 = 2048;
pub const MONO_DECLSEC_FLAG_NONCAS_DEMAND: _bindgen_ty_65 = 4096;
pub const MONO_DECLSEC_FLAG_NONCAS_LINKDEMAND: _bindgen_ty_65 = 8192;
pub const MONO_DECLSEC_FLAG_NONCAS_INHERITANCEDEMAND: _bindgen_ty_65 = 16384;
pub const MONO_DECLSEC_FLAG_LINKDEMAND_CHOICE: _bindgen_ty_65 = 32768;
pub const MONO_DECLSEC_FLAG_INHERITANCEDEMAND_CHOICE: _bindgen_ty_65 = 65536;
pub const MONO_DECLSEC_FLAG_DEMAND_CHOICE: _bindgen_ty_65 = 131072;
pub type _bindgen_ty_65 = ::std::os::raw::c_uint;
extern "C" {
    pub fn mono_declsec_flags_from_method(method: *mut MonoMethod) -> u32;
}
extern "C" {
    pub fn mono_declsec_flags_from_class(klass: *mut MonoClass) -> u32;
}
extern "C" {
    pub fn mono_declsec_flags_from_assembly(assembly: *mut MonoAssembly) -> u32;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoDeclSecurityEntry {
    pub blob: *mut ::std::os::raw::c_char,
    pub size: u32,
    pub index: u32,
}
#[test]
fn bindgen_test_layout_MonoDeclSecurityEntry() {
    assert_eq!(
        ::std::mem::size_of::<MonoDeclSecurityEntry>(),
        16usize,
        concat!("Size of: ", stringify!(MonoDeclSecurityEntry))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoDeclSecurityEntry>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoDeclSecurityEntry))
    );
    fn test_field_blob() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDeclSecurityEntry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).blob) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDeclSecurityEntry),
                "::",
                stringify!(blob)
            )
        );
    }
    test_field_blob();
    fn test_field_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDeclSecurityEntry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDeclSecurityEntry),
                "::",
                stringify!(size)
            )
        );
    }
    test_field_size();
    fn test_field_index() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDeclSecurityEntry>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).index) as usize - ptr as usize
            },
            12usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDeclSecurityEntry),
                "::",
                stringify!(index)
            )
        );
    }
    test_field_index();
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoDeclSecurityActions {
    pub demand: MonoDeclSecurityEntry,
    pub noncasdemand: MonoDeclSecurityEntry,
    pub demandchoice: MonoDeclSecurityEntry,
}
#[test]
fn bindgen_test_layout_MonoDeclSecurityActions() {
    assert_eq!(
        ::std::mem::size_of::<MonoDeclSecurityActions>(),
        48usize,
        concat!("Size of: ", stringify!(MonoDeclSecurityActions))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoDeclSecurityActions>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoDeclSecurityActions))
    );
    fn test_field_demand() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDeclSecurityActions>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).demand) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDeclSecurityActions),
                "::",
                stringify!(demand)
            )
        );
    }
    test_field_demand();
    fn test_field_noncasdemand() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDeclSecurityActions>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).noncasdemand) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDeclSecurityActions),
                "::",
                stringify!(noncasdemand)
            )
        );
    }
    test_field_noncasdemand();
    fn test_field_demandchoice() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDeclSecurityActions>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).demandchoice) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDeclSecurityActions),
                "::",
                stringify!(demandchoice)
            )
        );
    }
    test_field_demandchoice();
}
extern "C" {
    pub fn mono_declsec_get_demands(
        callee: *mut MonoMethod,
        demands: *mut MonoDeclSecurityActions,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_declsec_get_linkdemands(
        callee: *mut MonoMethod,
        klass: *mut MonoDeclSecurityActions,
        cmethod: *mut MonoDeclSecurityActions,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_declsec_get_inheritdemands_class(
        klass: *mut MonoClass,
        demands: *mut MonoDeclSecurityActions,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_declsec_get_inheritdemands_method(
        callee: *mut MonoMethod,
        demands: *mut MonoDeclSecurityActions,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_declsec_get_method_action(
        method: *mut MonoMethod,
        action: u32,
        entry: *mut MonoDeclSecurityEntry,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_declsec_get_class_action(
        klass: *mut MonoClass,
        action: u32,
        entry: *mut MonoDeclSecurityEntry,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_declsec_get_assembly_action(
        assembly: *mut MonoAssembly,
        action: u32,
        entry: *mut MonoDeclSecurityEntry,
    ) -> MonoBoolean;
}
extern "C" {
    pub fn mono_reflection_type_get_type(reftype: *mut MonoReflectionType) -> *mut MonoType;
}
extern "C" {
    pub fn mono_reflection_assembly_get_assembly(
        refassembly: *mut MonoReflectionAssembly,
    ) -> *mut MonoAssembly;
}
pub type MonoThreadStartCB = ::std::option::Option<
    unsafe extern "C" fn(
        tid: isize,
        stack_start: *mut ::std::os::raw::c_void,
        func: *mut ::std::os::raw::c_void,
    ),
>;
pub type MonoThreadAttachCB = ::std::option::Option<
    unsafe extern "C" fn(tid: isize, stack_start: *mut ::std::os::raw::c_void),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _MonoAppDomain {
    _unused: [u8; 0],
}
pub type MonoAppDomain = _MonoAppDomain;
pub type MonoDomainFunc = ::std::option::Option<
    unsafe extern "C" fn(domain: *mut MonoDomain, user_data: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn mono_init(filename: *const ::std::os::raw::c_char) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_init_from_assembly(
        domain_name: *const ::std::os::raw::c_char,
        filename: *const ::std::os::raw::c_char,
    ) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_init_version(
        domain_name: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
    ) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_get_root_domain() -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_runtime_init(
        domain: *mut MonoDomain,
        start_cb: MonoThreadStartCB,
        attach_cb: MonoThreadAttachCB,
    );
}
extern "C" {
    pub fn mono_runtime_cleanup(domain: *mut MonoDomain);
}
extern "C" {
    pub fn mono_install_runtime_cleanup(func: MonoDomainFunc);
}
extern "C" {
    pub fn mono_runtime_quit();
}
extern "C" {
    pub fn mono_runtime_set_shutting_down();
}
extern "C" {
    pub fn mono_runtime_is_shutting_down() -> mono_bool;
}
extern "C" {
    pub fn mono_check_corlib_version() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_domain_create() -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_domain_create_appdomain(
        friendly_name: *mut ::std::os::raw::c_char,
        configuration_file: *mut ::std::os::raw::c_char,
    ) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_domain_set_config(
        domain: *mut MonoDomain,
        base_dir: *const ::std::os::raw::c_char,
        config_file_name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mono_domain_get() -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_domain_get_by_id(domainid: i32) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_domain_get_id(domain: *mut MonoDomain) -> i32;
}
extern "C" {
    pub fn mono_domain_get_friendly_name(domain: *mut MonoDomain) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_domain_set(domain: *mut MonoDomain, force: mono_bool) -> mono_bool;
}
extern "C" {
    pub fn mono_domain_set_internal(domain: *mut MonoDomain);
}
extern "C" {
    pub fn mono_domain_unload(domain: *mut MonoDomain);
}
extern "C" {
    pub fn mono_domain_try_unload(domain: *mut MonoDomain, exc: *mut *mut MonoObject);
}
extern "C" {
    pub fn mono_domain_is_unloading(domain: *mut MonoDomain) -> mono_bool;
}
extern "C" {
    pub fn mono_domain_from_appdomain(appdomain: *mut MonoAppDomain) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_domain_foreach(func: MonoDomainFunc, user_data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn mono_domain_assembly_open(
        domain: *mut MonoDomain,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_domain_finalize(domain: *mut MonoDomain, timeout: u32) -> mono_bool;
}
extern "C" {
    pub fn mono_domain_free(domain: *mut MonoDomain, force: mono_bool);
}
extern "C" {
    pub fn mono_domain_has_type_resolve(domain: *mut MonoDomain) -> mono_bool;
}
extern "C" {
    pub fn mono_domain_try_type_resolve(
        domain: *mut MonoDomain,
        name: *mut ::std::os::raw::c_char,
        tb: *mut MonoObject,
    ) -> *mut MonoReflectionAssembly;
}
extern "C" {
    pub fn mono_domain_owns_vtable_slot(
        domain: *mut MonoDomain,
        vtable_slot: *mut ::std::os::raw::c_void,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_context_init(domain: *mut MonoDomain);
}
extern "C" {
    pub fn mono_context_set(new_context: *mut MonoAppContext);
}
extern "C" {
    pub fn mono_context_get() -> *mut MonoAppContext;
}
extern "C" {
    pub fn mono_context_get_id(context: *mut MonoAppContext) -> i32;
}
extern "C" {
    pub fn mono_context_get_domain_id(context: *mut MonoAppContext) -> i32;
}
extern "C" {
    pub fn mono_jit_info_table_find(
        domain: *mut MonoDomain,
        addr: *mut ::std::os::raw::c_void,
    ) -> *mut MonoJitInfo;
}
extern "C" {
    pub fn mono_jit_info_get_code_start(ji: *mut MonoJitInfo) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mono_jit_info_get_code_size(ji: *mut MonoJitInfo) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_jit_info_get_method(ji: *mut MonoJitInfo) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_get_corlib() -> *mut MonoImage;
}
extern "C" {
    pub fn mono_get_object_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_byte_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_void_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_boolean_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_sbyte_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_int16_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_uint16_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_int32_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_uint32_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_intptr_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_uintptr_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_int64_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_uint64_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_single_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_double_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_char_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_string_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_enum_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_array_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_thread_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_get_exception_class() -> *mut MonoClass;
}
extern "C" {
    pub fn mono_security_enable_core_clr();
}
pub type MonoCoreClrPlatformCB = ::std::option::Option<
    unsafe extern "C" fn(image_name: *const ::std::os::raw::c_char) -> mono_bool,
>;
extern "C" {
    pub fn mono_security_set_core_clr_platform_callback(callback: MonoCoreClrPlatformCB);
}
extern "C" {
    pub fn mono_jit_init(file: *const ::std::os::raw::c_char) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_jit_init_version(
        root_domain_name: *const ::std::os::raw::c_char,
        runtime_version: *const ::std::os::raw::c_char,
    ) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_jit_init_version_for_test_only(
        root_domain_name: *const ::std::os::raw::c_char,
        runtime_version: *const ::std::os::raw::c_char,
    ) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_jit_exec(
        domain: *mut MonoDomain,
        assembly: *mut MonoAssembly,
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mono_jit_cleanup(domain: *mut MonoDomain);
}
extern "C" {
    pub fn mono_jit_set_trace_options(options: *const ::std::os::raw::c_char) -> mono_bool;
}
extern "C" {
    pub fn mono_set_signal_chaining(chain_signals: mono_bool);
}
extern "C" {
    pub fn mono_set_crash_chaining(chain_signals: mono_bool);
}
extern "C" {
    #[doc = " This function is deprecated, use mono_jit_set_aot_mode instead."]
    pub fn mono_jit_set_aot_only(aot_only: mono_bool);
}
pub const MonoAotMode_MONO_AOT_MODE_NONE: MonoAotMode = 0;
pub const MonoAotMode_MONO_AOT_MODE_NORMAL: MonoAotMode = 1;
pub const MonoAotMode_MONO_AOT_MODE_HYBRID: MonoAotMode = 2;
pub const MonoAotMode_MONO_AOT_MODE_FULL: MonoAotMode = 3;
pub const MonoAotMode_MONO_AOT_MODE_LLVMONLY: MonoAotMode = 4;
pub const MonoAotMode_MONO_AOT_MODE_INTERP: MonoAotMode = 5;
pub const MonoAotMode_MONO_AOT_MODE_INTERP_LLVMONLY: MonoAotMode = 6;
pub const MonoAotMode_MONO_AOT_MODE_LLVMONLY_INTERP: MonoAotMode = 7;
pub const MonoAotMode_MONO_AOT_MODE_LAST: MonoAotMode = 1000;
#[doc = " Allows control over our AOT (Ahead-of-time) compilation mode."]
pub type MonoAotMode = ::std::os::raw::c_uint;
extern "C" {
    pub fn mono_jit_set_aot_mode(mode: MonoAotMode);
}
extern "C" {
    pub fn mono_jit_aot_compiling() -> mono_bool;
}
pub const MonoBreakPolicy_MONO_BREAK_POLICY_ALWAYS: MonoBreakPolicy = 0;
pub const MonoBreakPolicy_MONO_BREAK_POLICY_NEVER: MonoBreakPolicy = 1;
pub const MonoBreakPolicy_MONO_BREAK_POLICY_ON_DBG: MonoBreakPolicy = 2;
pub type MonoBreakPolicy = ::std::os::raw::c_uint;
pub type MonoBreakPolicyFunc =
    ::std::option::Option<unsafe extern "C" fn(method: *mut MonoMethod) -> MonoBreakPolicy>;
extern "C" {
    pub fn mono_set_break_policy(policy_callback: MonoBreakPolicyFunc);
}
extern "C" {
    pub fn mono_jit_parse_options(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mono_get_runtime_build_info() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_set_use_llvm(use_llvm: mono_bool);
}
extern "C" {
    pub fn mono_aot_register_module(aot_info: *mut *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn mono_jit_thread_attach(domain: *mut MonoDomain) -> *mut MonoDomain;
}
extern "C" {
    pub fn mono_assemblies_init();
}
extern "C" {
    pub fn mono_assemblies_cleanup();
}
extern "C" {
    pub fn mono_assembly_open(
        filename: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_open_full(
        filename: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
        refonly: mono_bool,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_load(
        aname: *mut MonoAssemblyName,
        basedir: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_load_full(
        aname: *mut MonoAssemblyName,
        basedir: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
        refonly: mono_bool,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_load_from(
        image: *mut MonoImage,
        fname: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_load_from_full(
        image: *mut MonoImage,
        fname: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
        refonly: mono_bool,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_load_with_partial_name(
        name: *const ::std::os::raw::c_char,
        status: *mut MonoImageOpenStatus,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_loaded(aname: *mut MonoAssemblyName) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_loaded_full(
        aname: *mut MonoAssemblyName,
        refonly: mono_bool,
    ) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_get_assemblyref(
        image: *mut MonoImage,
        index: ::std::os::raw::c_int,
        aname: *mut MonoAssemblyName,
    );
}
extern "C" {
    pub fn mono_assembly_load_reference(image: *mut MonoImage, index: ::std::os::raw::c_int);
}
extern "C" {
    pub fn mono_assembly_load_references(image: *mut MonoImage, status: *mut MonoImageOpenStatus);
}
extern "C" {
    pub fn mono_assembly_load_module(assembly: *mut MonoAssembly, idx: u32) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_assembly_close(assembly: *mut MonoAssembly);
}
extern "C" {
    pub fn mono_assembly_setrootdir(root_dir: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn mono_assembly_getrootdir() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_native_getrootdir() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_assembly_foreach(func: MonoFunc, user_data: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn mono_assembly_set_main(assembly: *mut MonoAssembly);
}
extern "C" {
    pub fn mono_assembly_get_main() -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_assembly_get_image(assembly: *mut MonoAssembly) -> *mut MonoImage;
}
extern "C" {
    pub fn mono_assembly_get_name(assembly: *mut MonoAssembly) -> *mut MonoAssemblyName;
}
extern "C" {
    pub fn mono_assembly_fill_assembly_name(
        image: *mut MonoImage,
        aname: *mut MonoAssemblyName,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_assembly_names_equal(
        l: *mut MonoAssemblyName,
        r: *mut MonoAssemblyName,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_stringify_assembly_name(
        aname: *mut MonoAssemblyName,
    ) -> *mut ::std::os::raw::c_char;
}
pub type MonoAssemblyLoadFunc = ::std::option::Option<
    unsafe extern "C" fn(assembly: *mut MonoAssembly, user_data: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn mono_install_assembly_load_hook(
        func: MonoAssemblyLoadFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
pub type MonoAssemblySearchFunc = ::std::option::Option<
    unsafe extern "C" fn(
        aname: *mut MonoAssemblyName,
        user_data: *mut ::std::os::raw::c_void,
    ) -> *mut MonoAssembly,
>;
extern "C" {
    pub fn mono_install_assembly_search_hook(
        func: MonoAssemblySearchFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_install_assembly_refonly_search_hook(
        func: MonoAssemblySearchFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_assembly_invoke_search_hook(aname: *mut MonoAssemblyName) -> *mut MonoAssembly;
}
extern "C" {
    pub fn mono_install_assembly_postload_search_hook(
        func: MonoAssemblySearchFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_install_assembly_postload_refonly_search_hook(
        func: MonoAssemblySearchFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
pub type MonoAssemblyPreLoadFunc = ::std::option::Option<
    unsafe extern "C" fn(
        aname: *mut MonoAssemblyName,
        assemblies_path: *mut *mut ::std::os::raw::c_char,
        user_data: *mut ::std::os::raw::c_void,
    ) -> *mut MonoAssembly,
>;
extern "C" {
    pub fn mono_install_assembly_preload_hook(
        func: MonoAssemblyPreLoadFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_install_assembly_refonly_preload_hook(
        func: MonoAssemblyPreLoadFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_assembly_invoke_load_hook(ass: *mut MonoAssembly);
}
extern "C" {
    pub fn mono_assembly_name_new(name: *const ::std::os::raw::c_char) -> *mut MonoAssemblyName;
}
extern "C" {
    pub fn mono_assembly_name_get_name(
        aname: *mut MonoAssemblyName,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_assembly_name_get_culture(
        aname: *mut MonoAssemblyName,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_assembly_name_get_version(
        aname: *mut MonoAssemblyName,
        minor: *mut u16,
        build: *mut u16,
        revision: *mut u16,
    ) -> u16;
}
extern "C" {
    pub fn mono_assembly_name_get_pubkeytoken(aname: *mut MonoAssemblyName) -> *mut mono_byte;
}
extern "C" {
    pub fn mono_assembly_name_free(aname: *mut MonoAssemblyName);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoBundledAssembly {
    pub name: *const ::std::os::raw::c_char,
    pub data: *const ::std::os::raw::c_uchar,
    pub size: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_MonoBundledAssembly() {
    assert_eq!(
        ::std::mem::size_of::<MonoBundledAssembly>(),
        24usize,
        concat!("Size of: ", stringify!(MonoBundledAssembly))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoBundledAssembly>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoBundledAssembly))
    );
    fn test_field_name() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoBundledAssembly>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoBundledAssembly),
                "::",
                stringify!(name)
            )
        );
    }
    test_field_name();
    fn test_field_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoBundledAssembly>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoBundledAssembly),
                "::",
                stringify!(data)
            )
        );
    }
    test_field_data();
    fn test_field_size() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoBundledAssembly>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).size) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoBundledAssembly),
                "::",
                stringify!(size)
            )
        );
    }
    test_field_size();
}
extern "C" {
    pub fn mono_register_bundled_assemblies(assemblies: *mut *const MonoBundledAssembly);
}
extern "C" {
    pub fn mono_register_config_for_assembly(
        assembly_name: *const ::std::os::raw::c_char,
        config_xml: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mono_register_symfile_for_assembly(
        assembly_name: *const ::std::os::raw::c_char,
        raw_contents: *const mono_byte,
        size: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mono_register_machine_config(config_xml: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn mono_set_rootdir();
}
extern "C" {
    pub fn mono_set_dirs(
        assembly_dir: *const ::std::os::raw::c_char,
        config_dir: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn mono_set_assemblies_path(path: *const ::std::os::raw::c_char);
}
pub type MonoDisIndenter = ::std::option::Option<
    unsafe extern "C" fn(
        dh: *mut MonoDisHelper,
        method: *mut MonoMethod,
        ip_offset: u32,
    ) -> *mut ::std::os::raw::c_char,
>;
pub type MonoDisTokener = ::std::option::Option<
    unsafe extern "C" fn(
        dh: *mut MonoDisHelper,
        method: *mut MonoMethod,
        token: u32,
    ) -> *mut ::std::os::raw::c_char,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoDisHelper {
    pub newline: *const ::std::os::raw::c_char,
    pub label_format: *const ::std::os::raw::c_char,
    pub label_target: *const ::std::os::raw::c_char,
    pub indenter: MonoDisIndenter,
    pub tokener: MonoDisTokener,
    pub user_data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_MonoDisHelper() {
    assert_eq!(
        ::std::mem::size_of::<MonoDisHelper>(),
        48usize,
        concat!("Size of: ", stringify!(MonoDisHelper))
    );
    assert_eq!(
        ::std::mem::align_of::<MonoDisHelper>(),
        8usize,
        concat!("Alignment of ", stringify!(MonoDisHelper))
    );
    fn test_field_newline() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDisHelper>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).newline) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDisHelper),
                "::",
                stringify!(newline)
            )
        );
    }
    test_field_newline();
    fn test_field_label_format() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDisHelper>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).label_format) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDisHelper),
                "::",
                stringify!(label_format)
            )
        );
    }
    test_field_label_format();
    fn test_field_label_target() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDisHelper>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).label_target) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDisHelper),
                "::",
                stringify!(label_target)
            )
        );
    }
    test_field_label_target();
    fn test_field_indenter() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDisHelper>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).indenter) as usize - ptr as usize
            },
            24usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDisHelper),
                "::",
                stringify!(indenter)
            )
        );
    }
    test_field_indenter();
    fn test_field_tokener() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDisHelper>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).tokener) as usize - ptr as usize
            },
            32usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDisHelper),
                "::",
                stringify!(tokener)
            )
        );
    }
    test_field_tokener();
    fn test_field_user_data() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<MonoDisHelper>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).user_data) as usize - ptr as usize
            },
            40usize,
            concat!(
                "Offset of field: ",
                stringify!(MonoDisHelper),
                "::",
                stringify!(user_data)
            )
        );
    }
    test_field_user_data();
}
extern "C" {
    pub fn mono_disasm_code_one(
        dh: *mut MonoDisHelper,
        method: *mut MonoMethod,
        ip: *const mono_byte,
        endp: *mut *const mono_byte,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_disasm_code(
        dh: *mut MonoDisHelper,
        method: *mut MonoMethod,
        ip: *const mono_byte,
        end: *const mono_byte,
    ) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MonoMethodDesc {
    _unused: [u8; 0],
}
extern "C" {
    pub fn mono_type_full_name(type_: *mut MonoType) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_signature_get_desc(
        sig: *mut MonoMethodSignature,
        include_namespace: mono_bool,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_context_get_desc(context: *mut MonoGenericContext) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_method_desc_new(
        name: *const ::std::os::raw::c_char,
        include_namespace: mono_bool,
    ) -> *mut MonoMethodDesc;
}
extern "C" {
    pub fn mono_method_desc_from_method(method: *mut MonoMethod) -> *mut MonoMethodDesc;
}
extern "C" {
    pub fn mono_method_desc_free(desc: *mut MonoMethodDesc);
}
extern "C" {
    pub fn mono_method_desc_match(desc: *mut MonoMethodDesc, method: *mut MonoMethod) -> mono_bool;
}
extern "C" {
    pub fn mono_method_desc_is_full(desc: *mut MonoMethodDesc) -> mono_bool;
}
extern "C" {
    pub fn mono_method_desc_full_match(
        desc: *mut MonoMethodDesc,
        method: *mut MonoMethod,
    ) -> mono_bool;
}
extern "C" {
    pub fn mono_method_desc_search_in_class(
        desc: *mut MonoMethodDesc,
        klass: *mut MonoClass,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_method_desc_search_in_image(
        desc: *mut MonoMethodDesc,
        image: *mut MonoImage,
    ) -> *mut MonoMethod;
}
extern "C" {
    pub fn mono_method_full_name(
        method: *mut MonoMethod,
        signature: mono_bool,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_method_get_reflection_name(method: *mut MonoMethod) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_field_full_name(field: *mut MonoClassField) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_exception_from_name(
        image: *mut MonoImage,
        name_space: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_exception_from_token(image: *mut MonoImage, token: u32) -> *mut MonoException;
}
extern "C" {
    pub fn mono_exception_from_name_two_strings(
        image: *mut MonoImage,
        name_space: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        a1: *mut MonoString,
        a2: *mut MonoString,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_exception_from_name_msg(
        image: *mut MonoImage,
        name_space: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_exception_from_token_two_strings(
        image: *mut MonoImage,
        token: u32,
        a1: *mut MonoString,
        a2: *mut MonoString,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_exception_from_name_domain(
        domain: *mut MonoDomain,
        image: *mut MonoImage,
        name_space: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_divide_by_zero() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_security() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_arithmetic() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_overflow() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_null_reference() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_execution_engine(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_thread_abort() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_thread_state(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_thread_interrupted() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_serialization(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_invalid_cast() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_invalid_operation(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_index_out_of_range() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_array_type_mismatch() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_type_load(
        class_name: *mut MonoString,
        assembly_name: *mut ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_missing_method(
        class_name: *const ::std::os::raw::c_char,
        member_name: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_missing_field(
        class_name: *const ::std::os::raw::c_char,
        member_name: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_not_implemented(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_not_supported(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_argument_null(
        arg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_argument(
        arg: *const ::std::os::raw::c_char,
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_argument_out_of_range(
        arg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_io(msg: *const ::std::os::raw::c_char) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_file_not_found(fname: *mut MonoString) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_file_not_found2(
        msg: *const ::std::os::raw::c_char,
        fname: *mut MonoString,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_type_initialization(
        type_name: *const ::std::os::raw::c_char,
        inner: *mut MonoException,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_synchronization_lock(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_cannot_unload_appdomain(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_appdomain_unloaded() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_bad_image_format(
        msg: *const ::std::os::raw::c_char,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_bad_image_format2(
        msg: *const ::std::os::raw::c_char,
        fname: *mut MonoString,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_stack_overflow() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_out_of_memory() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_field_access() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_method_access() -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_reflection_type_load(
        types: *mut MonoArray,
        exceptions: *mut MonoArray,
    ) -> *mut MonoException;
}
extern "C" {
    pub fn mono_get_exception_runtime_wrapped(
        wrapped_exception: *mut MonoObject,
    ) -> *mut MonoException;
}
pub type MonoUnhandledExceptionFunc = ::std::option::Option<
    unsafe extern "C" fn(exc: *mut MonoObject, user_data: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn mono_install_unhandled_exception_hook(
        func: MonoUnhandledExceptionFunc,
        user_data: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_invoke_unhandled_exception_hook(exc: *mut MonoObject);
}
pub type MonoThreadManageCallback =
    ::std::option::Option<unsafe extern "C" fn(thread: *mut MonoThread) -> mono_bool>;
extern "C" {
    pub fn mono_thread_init(start_cb: MonoThreadStartCB, attach_cb: MonoThreadAttachCB);
}
extern "C" {
    pub fn mono_thread_cleanup();
}
extern "C" {
    pub fn mono_thread_manage();
}
extern "C" {
    pub fn mono_thread_current() -> *mut MonoThread;
}
extern "C" {
    pub fn mono_thread_set_main(thread: *mut MonoThread);
}
extern "C" {
    pub fn mono_thread_get_main() -> *mut MonoThread;
}
extern "C" {
    pub fn mono_thread_stop(thread: *mut MonoThread);
}
extern "C" {
    pub fn mono_thread_new_init(
        tid: isize,
        stack_start: *mut ::std::os::raw::c_void,
        func: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_thread_create(
        domain: *mut MonoDomain,
        func: *mut ::std::os::raw::c_void,
        arg: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn mono_thread_attach(domain: *mut MonoDomain) -> *mut MonoThread;
}
extern "C" {
    pub fn mono_thread_detach(thread: *mut MonoThread);
}
extern "C" {
    pub fn mono_thread_exit();
}
extern "C" {
    pub fn mono_threads_attach_tools_thread();
}
extern "C" {
    pub fn mono_thread_get_name_utf8(thread: *mut MonoThread) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mono_thread_get_managed_id(thread: *mut MonoThread) -> i32;
}
extern "C" {
    pub fn mono_thread_set_manage_callback(thread: *mut MonoThread, func: MonoThreadManageCallback);
}
extern "C" {
    pub fn mono_threads_set_default_stacksize(stacksize: u32);
}
extern "C" {
    pub fn mono_threads_get_default_stacksize() -> u32;
}
extern "C" {
    pub fn mono_threads_request_thread_dump();
}
extern "C" {
    pub fn mono_thread_is_foreign(thread: *mut MonoThread) -> mono_bool;
}
extern "C" {
    pub fn mono_thread_detach_if_exiting() -> mono_bool;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    fn test_field_gp_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__va_list_tag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).gp_offset) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(__va_list_tag),
                "::",
                stringify!(gp_offset)
            )
        );
    }
    test_field_gp_offset();
    fn test_field_fp_offset() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__va_list_tag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).fp_offset) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(__va_list_tag),
                "::",
                stringify!(fp_offset)
            )
        );
    }
    test_field_fp_offset();
    fn test_field_overflow_arg_area() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__va_list_tag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).overflow_arg_area) as usize - ptr as usize
            },
            8usize,
            concat!(
                "Offset of field: ",
                stringify!(__va_list_tag),
                "::",
                stringify!(overflow_arg_area)
            )
        );
    }
    test_field_overflow_arg_area();
    fn test_field_reg_save_area() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<__va_list_tag>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).reg_save_area) as usize - ptr as usize
            },
            16usize,
            concat!(
                "Offset of field: ",
                stringify!(__va_list_tag),
                "::",
                stringify!(reg_save_area)
            )
        );
    }
    test_field_reg_save_area();
}
