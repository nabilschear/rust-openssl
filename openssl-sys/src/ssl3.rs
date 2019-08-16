#[cfg(feature = "sgx")]
use sgx_trts::libc::*;
#[cfg(not(feature = "sgx"))]
use libc::*;

pub const SSL3_VERSION: c_int = 0x300;

pub const SSL3_AD_ILLEGAL_PARAMETER: c_int = 47;
