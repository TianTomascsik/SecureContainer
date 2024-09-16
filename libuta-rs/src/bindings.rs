pub type size_t = std::os::raw::c_ulong;
pub type uta_rc = u32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _uta_context_v1_t {
    pub _unused: [u8; 0],
}
pub type uta_context_v1_t = _uta_context_v1_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct uta_api_v1_t {
    pub context_v1_size: Option<unsafe extern "C" fn() -> size_t>,
    pub len_key_max: Option<unsafe extern "C" fn() -> size_t>,
    pub open:
        Option<unsafe extern "C" fn(uta_context: *const uta_context_v1_t) -> uta_rc>,
    pub close:
        Option<unsafe extern "C" fn(uta_context: *const uta_context_v1_t) -> uta_rc>,
    pub derive_key: Option<
        unsafe extern "C" fn(
            uta_context: *const uta_context_v1_t,
            key: *mut u8,
            len_key: size_t,
            dv: *const u8,
            len_dv: size_t,
            key_slot: u8,
        ) -> uta_rc,
    >,
    pub get_random: Option<
        unsafe extern "C" fn(
            uta_context: *const uta_context_v1_t,
            random: *mut u8,
            len_random: size_t,
        ) -> uta_rc,
    >,
    pub get_device_uuid: Option<
        unsafe extern "C" fn(uta_context: *const uta_context_v1_t, uuid: *mut u8) -> uta_rc,
    >,
}
extern "C" {
    pub fn uta_init_v1(uta: *mut uta_api_v1_t) -> uta_rc;
}

