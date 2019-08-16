#[cfg(feature = "sgx")]
use sgx_trts::libc::*;
#[cfg(not(feature = "sgx"))]
use libc::*;

pub const DTLS1_COOKIE_LENGTH: c_uint = 256;
