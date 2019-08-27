#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(dead_code, overflowing_literals, unused_imports)]
#![doc(html_root_url = "https://docs.rs/openssl-sys/0.9")]

#![cfg_attr(feature = "sgx", no_std)]
#[cfg(feature = "sgx")]
#[macro_use]
extern crate sgx_tstd as std;
use std::prelude::v1::*;

#[cfg(feature = "sgx")]
extern crate sgx_trts;
#[cfg(feature = "sgx")]
use sgx_trts::libc::*;

#[cfg(not(feature = "sgx"))]
extern crate libc;
#[cfg(not(feature = "sgx"))]
use libc::*;

pub use aes::*;
pub use asn1::*;
pub use bio::*;
pub use bn::*;
pub use cms::*;
pub use conf::*;
pub use crypto::*;
pub use dh::*;
pub use dsa::*;
pub use dtls1::*;
pub use ec::*;
pub use err::*;
pub use evp::*;
pub use hmac::*;
pub use obj_mac::*;
pub use object::*;
pub use ocsp::*;
pub use ossl_typ::*;
pub use pem::*;
pub use pkcs12::*;
pub use pkcs7::*;
pub use rand::*;
pub use rsa::*;
pub use safestack::*;
pub use sha::*;
pub use srtp::*;
pub use ssl::*;
pub use ssl3::*;
pub use stack::*;
pub use tls1::*;
pub use x509::*;
pub use x509_vfy::*;
pub use x509v3::*;

#[macro_use]
mod macros;

mod aes;
mod asn1;
mod bio;
mod bn;
mod cms;
mod conf;
mod crypto;
mod dh;
mod dsa;
mod dtls1;
mod ec;
mod err;
mod evp;
mod hmac;
mod obj_mac;
mod object;
mod ocsp;
mod ossl_typ;
mod pem;
mod pkcs12;
mod pkcs7;
mod rand;
mod rsa;
mod safestack;
mod sha;
mod srtp;
mod ssl;
mod ssl3;
mod stack;
mod tls1;
mod x509;
mod x509_vfy;
mod x509v3;

// FIXME remove
pub type PasswordCallback = unsafe extern "C" fn(
    buf: *mut c_char,
    size: c_int,
    rwflag: c_int,
    user_data: *mut c_void,
) -> c_int;

#[cfg(ossl110)]
pub fn init() {
    use std::ptr;
    use std::sync::{Once, ONCE_INIT};

    // explicitly initialize to work around https://github.com/openssl/openssl/issues/3505
    static INIT: Once = ONCE_INIT;

    INIT.call_once(|| unsafe {
        OPENSSL_init_ssl(OPENSSL_INIT_LOAD_SSL_STRINGS, ptr::null_mut());
    })
}

#[cfg(not(ossl110))]
pub fn init() {
    use std::sync::{Once, ONCE_INIT};
    static INIT: Once = ONCE_INIT;
    INIT.call_once(|| unsafe {
        SSL_library_init();
        SSL_load_error_strings();
        OPENSSL_add_all_algorithms_noconf();
    })
}