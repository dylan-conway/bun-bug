/* automatically generated by rust-bindgen 0.71.1 */

pub const __WORDSIZE: u32 = 64;
pub const __has_safe_buffers: u32 = 1;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const USE_CLANG_TYPES: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BunLoader {
    BUN_LOADER_JSX = 0,
    BUN_LOADER_JS = 1,
    BUN_LOADER_TS = 2,
    BUN_LOADER_TSX = 3,
    BUN_LOADER_CSS = 4,
    BUN_LOADER_FILE = 5,
    BUN_LOADER_JSON = 6,
    BUN_LOADER_TOML = 7,
    BUN_LOADER_WASM = 8,
    BUN_LOADER_NAPI = 9,
    BUN_LOADER_BASE64 = 10,
    BUN_LOADER_DATAURL = 11,
    BUN_LOADER_TEXT = 12,
}
extern "C" {
    pub static BUN_LOADER_MAX: BunLoader;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BunLogOptions {
    pub __struct_size: usize,
    pub message_ptr: *const u8,
    pub message_len: usize,
    pub path_ptr: *const u8,
    pub path_len: usize,
    pub source_line_text_ptr: *const u8,
    pub source_line_text_len: usize,
    pub level: i8,
    pub line: ::std::os::raw::c_int,
    pub lineEnd: ::std::os::raw::c_int,
    pub column: ::std::os::raw::c_int,
    pub columnEnd: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OnBeforeParseArguments {
    pub __struct_size: usize,
    pub bun: *mut ::std::os::raw::c_void,
    pub path_ptr: *const u8,
    pub path_len: usize,
    pub namespace_ptr: *const u8,
    pub namespace_len: usize,
    pub default_loader: u8,
    pub external: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OnBeforeParseResult {
    pub __struct_size: usize,
    pub source_ptr: *mut u8,
    pub source_len: usize,
    pub loader: u8,
    pub fetchSourceCode: ::std::option::Option<
        unsafe extern "C" fn(
            args: *const OnBeforeParseArguments,
            result: *mut OnBeforeParseResult,
        ) -> ::std::os::raw::c_int,
    >,
    pub plugin_source_code_context: *mut ::std::os::raw::c_void,
    pub free_plugin_source_code_context:
        ::std::option::Option<unsafe extern "C" fn(ctx: *mut ::std::os::raw::c_void)>,
    pub log: ::std::option::Option<
        unsafe extern "C" fn(args: *const OnBeforeParseArguments, options: *mut BunLogOptions),
    >,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BunLogLevel {
    BUN_LOG_LEVEL_VERBOSE = 0,
    BUN_LOG_LEVEL_DEBUG = 1,
    BUN_LOG_LEVEL_INFO = 2,
    BUN_LOG_LEVEL_WARN = 3,
    BUN_LOG_LEVEL_ERROR = 4,
}
extern "C" {
    pub static BUN_LOG_MAX: BunLogLevel;
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;