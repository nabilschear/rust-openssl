#[cfg(target_env = "sgx")]
use sgx_trts::libc::*;

#[cfg(not(target_env = "sgx"))]
use libc::*;

pub const DTLS1_COOKIE_LENGTH: c_uint = 256;
