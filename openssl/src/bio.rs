use std::prelude::v1::*;
use ffi;
#[cfg(feature = "sgx")]
use sgx_trts::libc::*;
#[cfg(not(feature = "sgx"))]
use libc::*;
use std::marker::PhantomData;
use std::ptr;
use std::slice;

use cvt_p;
use error::ErrorStack;

pub struct MemBioSlice<'a>(*mut ffi::BIO, PhantomData<&'a [u8]>);

impl<'a> Drop for MemBioSlice<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::BIO_free_all(self.0);
        }
    }
}

impl<'a> MemBioSlice<'a> {
    pub fn new(buf: &'a [u8]) -> Result<MemBioSlice<'a>, ErrorStack> {
        ffi::init();

        assert!(buf.len() <= c_int::max_value() as usize);
        let bio = unsafe {
            cvt_p(BIO_new_mem_buf(
                buf.as_ptr() as *const _,
                buf.len() as c_int,
            ))?
        };

        Ok(MemBioSlice(bio, PhantomData))
    }

    pub fn as_ptr(&self) -> *mut ffi::BIO {
        self.0
    }
}

pub struct MemBio(*mut ffi::BIO);

impl Drop for MemBio {
    fn drop(&mut self) {
        unsafe {
            ffi::BIO_free_all(self.0);
        }
    }
}

impl MemBio {
    pub fn new() -> Result<MemBio, ErrorStack> {
        ffi::init();

        let bio = unsafe { cvt_p(ffi::BIO_new(ffi::BIO_s_mem()))? };
        Ok(MemBio(bio))
    }

    pub fn as_ptr(&self) -> *mut ffi::BIO {
        self.0
    }

    pub fn get_buf(&self) -> &[u8] {
        unsafe {
            let mut ptr = ptr::null_mut();
            let len = ffi::BIO_get_mem_data(self.0, &mut ptr);
            slice::from_raw_parts(ptr as *const _ as *const _, len as usize)
        }
    }

    pub unsafe fn from_ptr(bio: *mut ffi::BIO) -> MemBio {
        MemBio(bio)
    }
}

cfg_if! {
    if #[cfg(ossl102)] {
        use ffi::BIO_new_mem_buf;
    } else {
        #[allow(bad_style)]
        unsafe fn BIO_new_mem_buf(buf: *const c_void, len: c_int) -> *mut ffi::BIO {
            ffi::BIO_new_mem_buf(buf as *mut _, len)
        }
    }
}
