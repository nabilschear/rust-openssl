//! High level interface to certain symmetric ciphers.
//!
//! # Examples
//!
//! Encrypt data in AES128 CBC mode
//!
//! ```
//! use openssl::symm::{encrypt, Cipher};
//!
//! let cipher = Cipher::aes_128_cbc();
//! let data = b"Some Crypto Text";
//! let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
//! let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
//! let ciphertext = encrypt(
//!     cipher,
//!     key,
//!     Some(iv),
//!     data).unwrap();
//!
//! assert_eq!(
//!     b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
//!       \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
//!     &ciphertext[..]);
//! ```
//!
//! Encrypting an asymmetric key with a symmetric cipher
//!
//! ```
//! use openssl::rsa::{Padding, Rsa};
//! use openssl::symm::Cipher;
//!
//! // Generate keypair and encrypt private key:
//! let keypair = Rsa::generate(2048).unwrap();
//! let cipher = Cipher::aes_256_cbc();
//! let pubkey_pem = keypair.public_key_to_pem_pkcs1().unwrap();
//! let privkey_pem = keypair.private_key_to_pem_passphrase(cipher, b"Rust").unwrap();
//! // pubkey_pem and privkey_pem could be written to file here.
//!
//! // Load private and public key from string:
//! let pubkey = Rsa::public_key_from_pem_pkcs1(&pubkey_pem).unwrap();
//! let privkey = Rsa::private_key_from_pem_passphrase(&privkey_pem, b"Rust").unwrap();
//!
//! // Use the asymmetric keys to encrypt and decrypt a short message:
//! let msg = b"Foo bar";
//! let mut encrypted = vec![0; pubkey.size() as usize];
//! let mut decrypted = vec![0; privkey.size() as usize];
//! let len = pubkey.public_encrypt(msg, &mut encrypted, Padding::PKCS1).unwrap();
//! assert!(len > msg.len());
//! let len = privkey.private_decrypt(&encrypted, &mut decrypted, Padding::PKCS1).unwrap();
//! let output_string = String::from_utf8(decrypted[..len].to_vec()).unwrap();
//! assert_eq!("Foo bar", output_string);
//! println!("Decrypted: '{}'", output_string);
//! ```
use std::prelude::v1::*;
use ffi;
#[cfg(feature = "sgx")]
use sgx_trts::libc::*;
#[cfg(not(feature = "sgx"))]
use libc::*;
use std::cmp;
use std::ptr;

use error::ErrorStack;
use nid::Nid;
use {cvt, cvt_p};

#[derive(Copy, Clone)]
pub enum Mode {
    Encrypt,
    Decrypt,
}

/// Represents a particular cipher algorithm.
///
/// See OpenSSL doc at [`EVP_EncryptInit`] for more information on each algorithms.
///
/// [`EVP_EncryptInit`]: https://www.openssl.org/docs/man1.1.0/crypto/EVP_EncryptInit.html
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Cipher(*const ffi::EVP_CIPHER);

impl Cipher {
    /// Looks up the cipher for a certain nid.
    ///
    /// This corresponds to [`EVP_get_cipherbynid`]
    ///
    /// [`EVP_get_cipherbynid`]: https://www.openssl.org/docs/man1.0.2/crypto/EVP_get_cipherbyname.html
    pub fn from_nid(nid: Nid) -> Option<Cipher> {
        let ptr = unsafe { ffi::EVP_get_cipherbyname(ffi::OBJ_nid2sn(nid.as_raw())) };
        if ptr.is_null() {
            None
        } else {
            Some(Cipher(ptr))
        }
    }

    pub fn aes_128_ecb() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_ecb()) }
    }

    pub fn aes_128_cbc() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_cbc()) }
    }

    pub fn aes_128_xts() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_xts()) }
    }

    pub fn aes_128_ctr() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_ctr()) }
    }

    pub fn aes_128_cfb1() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_cfb1()) }
    }

    pub fn aes_128_cfb128() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_cfb128()) }
    }

    pub fn aes_128_cfb8() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_cfb8()) }
    }

    pub fn aes_128_gcm() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_gcm()) }
    }

    pub fn aes_128_ccm() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_ccm()) }
    }

    pub fn aes_128_ofb() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_128_ofb()) }
    }

    pub fn aes_192_ecb() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_ecb()) }
    }

    pub fn aes_192_cbc() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_cbc()) }
    }

    pub fn aes_192_ctr() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_ctr()) }
    }

    pub fn aes_192_cfb1() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_cfb1()) }
    }

    pub fn aes_192_cfb128() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_cfb128()) }
    }

    pub fn aes_192_cfb8() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_cfb8()) }
    }

    pub fn aes_192_gcm() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_gcm()) }
    }

    pub fn aes_192_ccm() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_ccm()) }
    }

    pub fn aes_192_ofb() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_192_ofb()) }
    }

    pub fn aes_256_ecb() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_ecb()) }
    }

    pub fn aes_256_cbc() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_cbc()) }
    }

    pub fn aes_256_xts() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_xts()) }
    }

    pub fn aes_256_ctr() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_ctr()) }
    }

    pub fn aes_256_cfb1() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_cfb1()) }
    }

    pub fn aes_256_cfb128() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_cfb128()) }
    }

    pub fn aes_256_cfb8() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_cfb8()) }
    }

    pub fn aes_256_gcm() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_gcm()) }
    }

    pub fn aes_256_ccm() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_ccm()) }
    }

    pub fn aes_256_ofb() -> Cipher {
        unsafe { Cipher(ffi::EVP_aes_256_ofb()) }
    }

    pub fn bf_cbc() -> Cipher {
        unsafe { Cipher(ffi::EVP_bf_cbc()) }
    }

    pub fn bf_ecb() -> Cipher {
        unsafe { Cipher(ffi::EVP_bf_ecb()) }
    }

    pub fn bf_cfb64() -> Cipher {
        unsafe { Cipher(ffi::EVP_bf_cfb64()) }
    }

    pub fn bf_ofb() -> Cipher {
        unsafe { Cipher(ffi::EVP_bf_ofb()) }
    }

    pub fn des_cbc() -> Cipher {
        unsafe { Cipher(ffi::EVP_des_cbc()) }
    }

    pub fn des_ecb() -> Cipher {
        unsafe { Cipher(ffi::EVP_des_ecb()) }
    }

    pub fn des_ede3() -> Cipher {
        unsafe { Cipher(ffi::EVP_des_ede3()) }
    }

    pub fn des_ede3_cbc() -> Cipher {
        unsafe { Cipher(ffi::EVP_des_ede3_cbc()) }
    }

    pub fn des_ede3_cfb64() -> Cipher {
        unsafe { Cipher(ffi::EVP_des_ede3_cfb64()) }
    }

    pub fn rc4() -> Cipher {
        unsafe { Cipher(ffi::EVP_rc4()) }
    }

    /// Requires OpenSSL 1.1.0 or newer.
    #[cfg(any(ossl110))]
    pub fn chacha20() -> Cipher {
        unsafe { Cipher(ffi::EVP_chacha20()) }
    }

    /// Requires OpenSSL 1.1.0 or newer.
    #[cfg(any(ossl110))]
    pub fn chacha20_poly1305() -> Cipher {
        unsafe { Cipher(ffi::EVP_chacha20_poly1305()) }
    }

    pub unsafe fn from_ptr(ptr: *const ffi::EVP_CIPHER) -> Cipher {
        Cipher(ptr)
    }

    pub fn as_ptr(&self) -> *const ffi::EVP_CIPHER {
        self.0
    }

    /// Returns the length of keys used with this cipher.
    pub fn key_len(&self) -> usize {
        unsafe { EVP_CIPHER_key_length(self.0) as usize }
    }

    /// Returns the length of the IV used with this cipher, or `None` if the
    /// cipher does not use an IV.
    pub fn iv_len(&self) -> Option<usize> {
        unsafe {
            let len = EVP_CIPHER_iv_length(self.0) as usize;
            if len == 0 {
                None
            } else {
                Some(len)
            }
        }
    }

    /// Returns the block size of the cipher.
    ///
    /// # Note
    ///
    /// Stream ciphers such as RC4 have a block size of 1.
    pub fn block_size(&self) -> usize {
        unsafe { EVP_CIPHER_block_size(self.0) as usize }
    }

    /// Determines whether the cipher is using CCM mode
    fn is_ccm(&self) -> bool {
        // NOTE: OpenSSL returns pointers to static structs, which makes this work as expected
        *self == Cipher::aes_128_ccm() || *self == Cipher::aes_256_ccm()
    }
}

unsafe impl Sync for Cipher {}
unsafe impl Send for Cipher {}

/// Represents a symmetric cipher context.
///
/// Padding is enabled by default.
///
/// # Examples
///
/// Encrypt some plaintext in chunks, then decrypt the ciphertext back into plaintext, in AES 128
/// CBC mode.
///
/// ```
/// use openssl::symm::{Cipher, Mode, Crypter};
///
/// let plaintexts: [&[u8]; 2] = [b"Some Stream of", b" Crypto Text"];
/// let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
/// let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
/// let data_len = plaintexts.iter().fold(0, |sum, x| sum + x.len());
///
/// // Create a cipher context for encryption.
/// let mut encrypter = Crypter::new(
///     Cipher::aes_128_cbc(),
///     Mode::Encrypt,
///     key,
///     Some(iv)).unwrap();
///
/// let block_size = Cipher::aes_128_cbc().block_size();
/// let mut ciphertext = vec![0; data_len + block_size];
///
/// // Encrypt 2 chunks of plaintexts successively.
/// let mut count = encrypter.update(plaintexts[0], &mut ciphertext).unwrap();
/// count += encrypter.update(plaintexts[1], &mut ciphertext[count..]).unwrap();
/// count += encrypter.finalize(&mut ciphertext[count..]).unwrap();
/// ciphertext.truncate(count);
///
/// assert_eq!(
///     b"\x0F\x21\x83\x7E\xB2\x88\x04\xAF\xD9\xCC\xE2\x03\x49\xB4\x88\xF6\xC4\x61\x0E\x32\x1C\xF9\
///       \x0D\x66\xB1\xE6\x2C\x77\x76\x18\x8D\x99",
///     &ciphertext[..]
/// );
///
///
/// // Let's pretend we don't know the plaintext, and now decrypt the ciphertext.
/// let data_len = ciphertext.len();
/// let ciphertexts = [&ciphertext[..9], &ciphertext[9..]];
///
/// // Create a cipher context for decryption.
/// let mut decrypter = Crypter::new(
///     Cipher::aes_128_cbc(),
///     Mode::Decrypt,
///     key,
///     Some(iv)).unwrap();
/// let mut plaintext = vec![0; data_len + block_size];
///
/// // Decrypt 2 chunks of ciphertexts successively.
/// let mut count = decrypter.update(ciphertexts[0], &mut plaintext).unwrap();
/// count += decrypter.update(ciphertexts[1], &mut plaintext[count..]).unwrap();
/// count += decrypter.finalize(&mut plaintext[count..]).unwrap();
/// plaintext.truncate(count);
///
/// assert_eq!(b"Some Stream of Crypto Text", &plaintext[..]);
/// ```
pub struct Crypter {
    ctx: *mut ffi::EVP_CIPHER_CTX,
    block_size: usize,
}

unsafe impl Sync for Crypter {}
unsafe impl Send for Crypter {}

impl Crypter {
    /// Creates a new `Crypter`.  The initialisation vector, `iv`, is not necesarry for certain
    /// types of `Cipher`.
    ///
    /// # Panics
    ///
    /// Panics if an IV is required by the cipher but not provided.  Also make sure that the key
    /// and IV size are appropriate for your cipher.
    pub fn new(
        t: Cipher,
        mode: Mode,
        key: &[u8],
        iv: Option<&[u8]>,
    ) -> Result<Crypter, ErrorStack> {
        ffi::init();

        unsafe {
            let ctx = cvt_p(ffi::EVP_CIPHER_CTX_new())?;
            let crypter = Crypter {
                ctx: ctx,
                block_size: t.block_size(),
            };

            let mode = match mode {
                Mode::Encrypt => 1,
                Mode::Decrypt => 0,
            };

            cvt(ffi::EVP_CipherInit_ex(
                crypter.ctx,
                t.as_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                mode,
            ))?;

            assert!(key.len() <= c_int::max_value() as usize);
            cvt(ffi::EVP_CIPHER_CTX_set_key_length(
                crypter.ctx,
                key.len() as c_int,
            ))?;

            let key = key.as_ptr() as *mut _;
            let iv = match (iv, t.iv_len()) {
                (Some(iv), Some(len)) => {
                    if iv.len() != len {
                        assert!(iv.len() <= c_int::max_value() as usize);
                        cvt(ffi::EVP_CIPHER_CTX_ctrl(
                            crypter.ctx,
                            ffi::EVP_CTRL_GCM_SET_IVLEN,
                            iv.len() as c_int,
                            ptr::null_mut(),
                        ))?;
                    }
                    iv.as_ptr() as *mut _
                }
                (Some(_), None) | (None, None) => ptr::null_mut(),
                (None, Some(_)) => panic!("an IV is required for this cipher"),
            };
            cvt(ffi::EVP_CipherInit_ex(
                crypter.ctx,
                ptr::null(),
                ptr::null_mut(),
                key,
                iv,
                mode,
            ))?;

            Ok(crypter)
        }
    }

    /// Enables or disables padding.
    ///
    /// If padding is disabled, total amount of data encrypted/decrypted must
    /// be a multiple of the cipher's block size.
    pub fn pad(&mut self, padding: bool) {
        unsafe {
            ffi::EVP_CIPHER_CTX_set_padding(self.ctx, padding as c_int);
        }
    }

    /// Sets the tag used to authenticate ciphertext in AEAD ciphers such as AES GCM.
    ///
    /// When decrypting cipher text using an AEAD cipher, this must be called before `finalize`.
    pub fn set_tag(&mut self, tag: &[u8]) -> Result<(), ErrorStack> {
        unsafe {
            assert!(tag.len() <= c_int::max_value() as usize);
            // NB: this constant is actually more general than just GCM.
            cvt(ffi::EVP_CIPHER_CTX_ctrl(
                self.ctx,
                ffi::EVP_CTRL_GCM_SET_TAG,
                tag.len() as c_int,
                tag.as_ptr() as *mut _,
            ))
            .map(|_| ())
        }
    }

    /// Sets the length of the authentication tag to generate in AES CCM.
    ///
    /// When encrypting with AES CCM, the tag length needs to be explicitly set in order
    /// to use a value different than the default 12 bytes.
    pub fn set_tag_len(&mut self, tag_len: usize) -> Result<(), ErrorStack> {
        unsafe {
            assert!(tag_len <= c_int::max_value() as usize);
            // NB: this constant is actually more general than just GCM.
            cvt(ffi::EVP_CIPHER_CTX_ctrl(
                self.ctx,
                ffi::EVP_CTRL_GCM_SET_TAG,
                tag_len as c_int,
                ptr::null_mut(),
            ))
            .map(|_| ())
        }
    }

    /// Feeds total plaintext length to the cipher.
    ///
    /// The total plaintext or ciphertext length MUST be passed to the cipher when it operates in
    /// CCM mode.
    pub fn set_data_len(&mut self, data_len: usize) -> Result<(), ErrorStack> {
        unsafe {
            assert!(data_len <= c_int::max_value() as usize);
            let mut len = 0;
            cvt(ffi::EVP_CipherUpdate(
                self.ctx,
                ptr::null_mut(),
                &mut len,
                ptr::null_mut(),
                data_len as c_int,
            ))
            .map(|_| ())
        }
    }

    /// Feeds Additional Authenticated Data (AAD) through the cipher.
    ///
    /// This can only be used with AEAD ciphers such as AES GCM. Data fed in is not encrypted, but
    /// is factored into the authentication tag. It must be called before the first call to
    /// `update`.
    pub fn aad_update(&mut self, input: &[u8]) -> Result<(), ErrorStack> {
        unsafe {
            assert!(input.len() <= c_int::max_value() as usize);
            let mut len = 0;
            cvt(ffi::EVP_CipherUpdate(
                self.ctx,
                ptr::null_mut(),
                &mut len,
                input.as_ptr(),
                input.len() as c_int,
            ))
            .map(|_| ())
        }
    }

    /// Feeds data from `input` through the cipher, writing encrypted/decrypted
    /// bytes into `output`.
    ///
    /// The number of bytes written to `output` is returned. Note that this may
    /// not be equal to the length of `input`.
    ///
    /// # Panics
    ///
    /// Panics for stream ciphers if `output.len() < input.len()`.
    ///
    /// Panics for block ciphers if `output.len() < input.len() + block_size`,
    /// where `block_size` is the block size of the cipher (see `Cipher::block_size`).
    ///
    /// Panics if `output.len() > c_int::max_value()`.
    pub fn update(&mut self, input: &[u8], output: &mut [u8]) -> Result<usize, ErrorStack> {
        unsafe {
            let block_size = if self.block_size > 1 { self.block_size } else { 0 };
            assert!(output.len() >= input.len() + block_size);
            assert!(output.len() <= c_int::max_value() as usize);
            let mut outl = output.len() as c_int;
            let inl = input.len() as c_int;

            cvt(ffi::EVP_CipherUpdate(
                self.ctx,
                output.as_mut_ptr(),
                &mut outl,
                input.as_ptr(),
                inl,
            ))?;

            Ok(outl as usize)
        }
    }

    /// Finishes the encryption/decryption process, writing any remaining data
    /// to `output`.
    ///
    /// The number of bytes written to `output` is returned.
    ///
    /// `update` should not be called after this method.
    ///
    /// # Panics
    ///
    /// Panics for block ciphers if `output.len() < block_size`,
    /// where `block_size` is the block size of the cipher (see `Cipher::block_size`).
    pub fn finalize(&mut self, output: &mut [u8]) -> Result<usize, ErrorStack> {
        unsafe {
            if self.block_size > 1 { assert!(output.len() >= self.block_size); }
            let mut outl = cmp::min(output.len(), c_int::max_value() as usize) as c_int;

            cvt(ffi::EVP_CipherFinal(
                self.ctx,
                output.as_mut_ptr(),
                &mut outl,
            ))?;

            Ok(outl as usize)
        }
    }

    /// Retrieves the authentication tag used to authenticate ciphertext in AEAD ciphers such
    /// as AES GCM.
    ///
    /// When encrypting data with an AEAD cipher, this must be called after `finalize`.
    ///
    /// The size of the buffer indicates the required size of the tag. While some ciphers support a
    /// range of tag sizes, it is recommended to pick the maximum size. For AES GCM, this is 16
    /// bytes, for example.
    pub fn get_tag(&self, tag: &mut [u8]) -> Result<(), ErrorStack> {
        unsafe {
            assert!(tag.len() <= c_int::max_value() as usize);
            cvt(ffi::EVP_CIPHER_CTX_ctrl(
                self.ctx,
                ffi::EVP_CTRL_GCM_GET_TAG,
                tag.len() as c_int,
                tag.as_mut_ptr() as *mut _,
            ))
            .map(|_| ())
        }
    }
}

impl Drop for Crypter {
    fn drop(&mut self) {
        unsafe {
            ffi::EVP_CIPHER_CTX_free(self.ctx);
        }
    }
}

/// Encrypts data in one go, and returns the encrypted data.
///
/// Data is encrypted using the specified cipher type `t` in encrypt mode with the specified `key`
/// and initailization vector `iv`. Padding is enabled.
///
/// This is a convenient interface to `Crypter` to encrypt all data in one go.  To encrypt a stream
/// of data increamentally , use `Crypter` instead.
///
/// # Examples
///
/// Encrypt data in AES128 CBC mode
///
/// ```
/// use openssl::symm::{encrypt, Cipher};
///
/// let cipher = Cipher::aes_128_cbc();
/// let data = b"Some Crypto Text";
/// let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
/// let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
/// let ciphertext = encrypt(
///     cipher,
///     key,
///     Some(iv),
///     data).unwrap();
///
/// assert_eq!(
///     b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\x87\x4D\
///       \xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1",
///     &ciphertext[..]);
/// ```
pub fn encrypt(
    t: Cipher,
    key: &[u8],
    iv: Option<&[u8]>,
    data: &[u8],
) -> Result<Vec<u8>, ErrorStack> {
    cipher(t, Mode::Encrypt, key, iv, data)
}

/// Decrypts data in one go, and returns the decrypted data.
///
/// Data is decrypted using the specified cipher type `t` in decrypt mode with the specified `key`
/// and initailization vector `iv`. Padding is enabled.
///
/// This is a convenient interface to `Crypter` to decrypt all data in one go.  To decrypt a  stream
/// of data increamentally , use `Crypter` instead.
///
/// # Examples
///
/// Decrypt data in AES128 CBC mode
///
/// ```
/// use openssl::symm::{decrypt, Cipher};
///
/// let cipher = Cipher::aes_128_cbc();
/// let data = b"\xB4\xB9\xE7\x30\xD6\xD6\xF7\xDE\x77\x3F\x1C\xFF\xB3\x3E\x44\x5A\x91\xD7\x27\x62\
///              \x87\x4D\xFB\x3C\x5E\xC4\x59\x72\x4A\xF4\x7C\xA1";
/// let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F";
/// let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x00\x01\x02\x03\x04\x05\x06\x07";
/// let ciphertext = decrypt(
///     cipher,
///     key,
///     Some(iv),
///     data).unwrap();
///
/// assert_eq!(
///     b"Some Crypto Text",
///     &ciphertext[..]);
/// ```
pub fn decrypt(
    t: Cipher,
    key: &[u8],
    iv: Option<&[u8]>,
    data: &[u8],
) -> Result<Vec<u8>, ErrorStack> {
    cipher(t, Mode::Decrypt, key, iv, data)
}

fn cipher(
    t: Cipher,
    mode: Mode,
    key: &[u8],
    iv: Option<&[u8]>,
    data: &[u8],
) -> Result<Vec<u8>, ErrorStack> {
    let mut c = Crypter::new(t, mode, key, iv)?;
    let mut out = vec![0; data.len() + t.block_size()];
    let count = c.update(data, &mut out)?;
    let rest = c.finalize(&mut out[count..])?;
    out.truncate(count + rest);
    Ok(out)
}

/// Like `encrypt`, but for AEAD ciphers such as AES GCM.
///
/// Additional Authenticated Data can be provided in the `aad` field, and the authentication tag
/// will be copied into the `tag` field.
///
/// The size of the `tag` buffer indicates the required size of the tag. While some ciphers support
/// a range of tag sizes, it is recommended to pick the maximum size. For AES GCM, this is 16 bytes,
/// for example.
pub fn encrypt_aead(
    t: Cipher,
    key: &[u8],
    iv: Option<&[u8]>,
    aad: &[u8],
    data: &[u8],
    tag: &mut [u8],
) -> Result<Vec<u8>, ErrorStack> {
    let mut c = Crypter::new(t, Mode::Encrypt, key, iv)?;
    let mut out = vec![0; data.len() + t.block_size()];

    if t.is_ccm() {
        c.set_tag_len(tag.len())?;
        c.set_data_len(data.len())?;
    }

    c.aad_update(aad)?;
    let count = c.update(data, &mut out)?;
    let rest = c.finalize(&mut out[count..])?;
    c.get_tag(tag)?;
    out.truncate(count + rest);
    Ok(out)
}

/// Like `decrypt`, but for AEAD ciphers such as AES GCM.
///
/// Additional Authenticated Data can be provided in the `aad` field, and the authentication tag
/// should be provided in the `tag` field.
pub fn decrypt_aead(
    t: Cipher,
    key: &[u8],
    iv: Option<&[u8]>,
    aad: &[u8],
    data: &[u8],
    tag: &[u8],
) -> Result<Vec<u8>, ErrorStack> {
    let mut c = Crypter::new(t, Mode::Decrypt, key, iv)?;
    let mut out = vec![0; data.len() + t.block_size()];

    if t.is_ccm() {
        c.set_tag(tag)?;
        c.set_data_len(data.len())?;
    }

    c.aad_update(aad)?;
    let count = c.update(data, &mut out)?;
    let mut rest = 0;

    if !t.is_ccm() {
        c.set_tag(tag)?;
        rest = c.finalize(&mut out[count..])?;
    }

    out.truncate(count + rest);
    Ok(out)
}

cfg_if! {
    if #[cfg(any(ossl110, libressl273))] {
        use ffi::{EVP_CIPHER_block_size, EVP_CIPHER_iv_length, EVP_CIPHER_key_length};
    } else {
        #[allow(bad_style)]
        pub unsafe fn EVP_CIPHER_iv_length(ptr: *const ffi::EVP_CIPHER) -> c_int {
            (*ptr).iv_len
        }

        #[allow(bad_style)]
        pub unsafe fn EVP_CIPHER_block_size(ptr: *const ffi::EVP_CIPHER) -> c_int {
            (*ptr).block_size
        }

        #[allow(bad_style)]
        pub unsafe fn EVP_CIPHER_key_length(ptr: *const ffi::EVP_CIPHER) -> c_int {
            (*ptr).key_len
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex::{self, FromHex};

    #[test]
    fn test_stream_cipher_output() {
        let key = [0u8; 16];
        let iv = [0u8; 16];
        let mut c = super::Crypter::new(
            super::Cipher::aes_128_ctr(),
            super::Mode::Encrypt,
            &key,
            Some(&iv),
        ).unwrap();

        assert_eq!(c.update(&[0u8; 15], &mut [0u8; 15]).unwrap(), 15);
        assert_eq!(c.update(&[0u8; 1], &mut [0u8; 1]).unwrap(), 1);
        assert_eq!(c.finalize(&mut [0u8; 0]).unwrap(), 0);
    }

    // Test vectors from FIPS-197:
    // http://csrc.nist.gov/publications/fips/fips197/fips-197.pdf
    #[test]
    fn test_aes_256_ecb() {
        let k0 = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8, 0x04u8, 0x05u8, 0x06u8, 0x07u8, 0x08u8, 0x09u8, 0x0au8,
            0x0bu8, 0x0cu8, 0x0du8, 0x0eu8, 0x0fu8, 0x10u8, 0x11u8, 0x12u8, 0x13u8, 0x14u8, 0x15u8,
            0x16u8, 0x17u8, 0x18u8, 0x19u8, 0x1au8, 0x1bu8, 0x1cu8, 0x1du8, 0x1eu8, 0x1fu8,
        ];
        let p0 = [
            0x00u8, 0x11u8, 0x22u8, 0x33u8, 0x44u8, 0x55u8, 0x66u8, 0x77u8, 0x88u8, 0x99u8, 0xaau8,
            0xbbu8, 0xccu8, 0xddu8, 0xeeu8, 0xffu8,
        ];
        let c0 = [
            0x8eu8, 0xa2u8, 0xb7u8, 0xcau8, 0x51u8, 0x67u8, 0x45u8, 0xbfu8, 0xeau8, 0xfcu8, 0x49u8,
            0x90u8, 0x4bu8, 0x49u8, 0x60u8, 0x89u8,
        ];
        let mut c = super::Crypter::new(
            super::Cipher::aes_256_ecb(),
            super::Mode::Encrypt,
            &k0,
            None,
        )
        .unwrap();
        c.pad(false);
        let mut r0 = vec![0; c0.len() + super::Cipher::aes_256_ecb().block_size()];
        let count = c.update(&p0, &mut r0).unwrap();
        let rest = c.finalize(&mut r0[count..]).unwrap();
        r0.truncate(count + rest);
        assert_eq!(hex::encode(&r0), hex::encode(c0));

        let mut c = super::Crypter::new(
            super::Cipher::aes_256_ecb(),
            super::Mode::Decrypt,
            &k0,
            None,
        )
        .unwrap();
        c.pad(false);
        let mut p1 = vec![0; r0.len() + super::Cipher::aes_256_ecb().block_size()];
        let count = c.update(&r0, &mut p1).unwrap();
        let rest = c.finalize(&mut p1[count..]).unwrap();
        p1.truncate(count + rest);
        assert_eq!(hex::encode(p1), hex::encode(p0));
    }

    #[test]
    fn test_aes_256_cbc_decrypt() {
        let iv = [
            4_u8, 223_u8, 153_u8, 219_u8, 28_u8, 142_u8, 234_u8, 68_u8, 227_u8, 69_u8, 98_u8,
            107_u8, 208_u8, 14_u8, 236_u8, 60_u8,
        ];
        let data = [
            143_u8, 210_u8, 75_u8, 63_u8, 214_u8, 179_u8, 155_u8, 241_u8, 242_u8, 31_u8, 154_u8,
            56_u8, 198_u8, 145_u8, 192_u8, 64_u8, 2_u8, 245_u8, 167_u8, 220_u8, 55_u8, 119_u8,
            233_u8, 136_u8, 139_u8, 27_u8, 71_u8, 242_u8, 119_u8, 175_u8, 65_u8, 207_u8,
        ];
        let ciphered_data = [
            0x4a_u8, 0x2e_u8, 0xe5_u8, 0x6_u8, 0xbf_u8, 0xcf_u8, 0xf2_u8, 0xd7_u8, 0xea_u8,
            0x2d_u8, 0xb1_u8, 0x85_u8, 0x6c_u8, 0x93_u8, 0x65_u8, 0x6f_u8,
        ];
        let mut cr = super::Crypter::new(
            super::Cipher::aes_256_cbc(),
            super::Mode::Decrypt,
            &data,
            Some(&iv),
        )
        .unwrap();
        cr.pad(false);
        let mut unciphered_data = vec![0; data.len() + super::Cipher::aes_256_cbc().block_size()];
        let count = cr.update(&ciphered_data, &mut unciphered_data).unwrap();
        let rest = cr.finalize(&mut unciphered_data[count..]).unwrap();
        unciphered_data.truncate(count + rest);

        let expected_unciphered_data = b"I love turtles.\x01";

        assert_eq!(&unciphered_data, expected_unciphered_data);
    }

    fn cipher_test(ciphertype: super::Cipher, pt: &str, ct: &str, key: &str, iv: &str) {
        let pt = Vec::from_hex(pt).unwrap();
        let ct = Vec::from_hex(ct).unwrap();
        let key = Vec::from_hex(key).unwrap();
        let iv = Vec::from_hex(iv).unwrap();

        let computed = super::decrypt(ciphertype, &key, Some(&iv), &ct).unwrap();
        let expected = pt;

        if computed != expected {
            println!("Computed: {}", hex::encode(&computed));
            println!("Expected: {}", hex::encode(&expected));
            if computed.len() != expected.len() {
                println!(
                    "Lengths differ: {} in computed vs {} expected",
                    computed.len(),
                    expected.len()
                );
            }
            panic!("test failure");
        }
    }

    fn cipher_test_nopad(ciphertype: super::Cipher, pt: &str, ct: &str, key: &str, iv: &str) {
        let pt = Vec::from_hex(pt).unwrap();
        let ct = Vec::from_hex(ct).unwrap();
        let key = Vec::from_hex(key).unwrap();
        let iv = Vec::from_hex(iv).unwrap();

        let computed = {
            let mut c = Crypter::new(ciphertype, Mode::Decrypt, &key, Some(&iv)).unwrap();
            c.pad(false);
            let mut out = vec![0; ct.len() + ciphertype.block_size()];
            let count = c.update(&ct, &mut out).unwrap();
            let rest = c.finalize(&mut out[count..]).unwrap();
            out.truncate(count + rest);
            out
        };
        let expected = pt;

        if computed != expected {
            println!("Computed: {}", hex::encode(&computed));
            println!("Expected: {}", hex::encode(&expected));
            if computed.len() != expected.len() {
                println!(
                    "Lengths differ: {} in computed vs {} expected",
                    computed.len(),
                    expected.len()
                );
            }
            panic!("test failure");
        }
    }

    #[test]
    fn test_rc4() {
        let pt = "0000000000000000000000000000000000000000000000000000000000000000000000000000";
        let ct = "A68686B04D686AA107BD8D4CAB191A3EEC0A6294BC78B60F65C25CB47BD7BB3A48EFC4D26BE4";
        let key = "97CD440324DA5FD1F7955C1C13B6B466";
        let iv = "";

        cipher_test(super::Cipher::rc4(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes256_xts() {
        // Test case 174 from
        // http://csrc.nist.gov/groups/STM/cavp/documents/aes/XTSTestVectors.zip
        let pt = "77f4ef63d734ebd028508da66c22cdebdd52ecd6ee2ab0a50bc8ad0cfd692ca5fcd4e6dedc45df7f\
                  6503f462611dc542";
        let ct = "ce7d905a7776ac72f240d22aafed5e4eb7566cdc7211220e970da634ce015f131a5ecb8d400bc9e8\
                  4f0b81d8725dbbc7";
        let key = "b6bfef891f83b5ff073f2231267be51eb084b791fa19a154399c0684c8b2dfcb37de77d28bbda3b\
                   4180026ad640b74243b3133e7b9fae629403f6733423dae28";
        let iv = "db200efb7eaaa737dbdf40babb68953f";

        cipher_test(super::Cipher::aes_256_xts(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes128_ctr() {
        let pt = "6BC1BEE22E409F96E93D7E117393172AAE2D8A571E03AC9C9EB76FAC45AF8E5130C81C46A35CE411\
                  E5FBC1191A0A52EFF69F2445DF4F9B17AD2B417BE66C3710";
        let ct = "874D6191B620E3261BEF6864990DB6CE9806F66B7970FDFF8617187BB9FFFDFF5AE4DF3EDBD5D35E\
                  5B4F09020DB03EAB1E031DDA2FBE03D1792170A0F3009CEE";
        let key = "2B7E151628AED2A6ABF7158809CF4F3C";
        let iv = "F0F1F2F3F4F5F6F7F8F9FAFBFCFDFEFF";

        cipher_test(super::Cipher::aes_128_ctr(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes128_cfb1() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1";
        let ct = "68b3";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_128_cfb1(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes128_cfb128() {
        let pt = "6bc1bee22e409f96e93d7e117393172a";
        let ct = "3b3fd92eb72dad20333449f8e83cfb4a";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_128_cfb128(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes128_cfb8() {
        let pt = "6bc1bee22e409f96e93d7e117393172aae2d";
        let ct = "3b79424c9c0dd436bace9e0ed4586a4f32b9";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_128_cfb8(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes128_ofb() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710";
        let ct = "3b3fd92eb72dad20333449f8e83cfb4a7789508d16918f03f53c52dac54ed8259740051e9c5fecf64344f7a82260edcc304c6528f659c77866a510d9c1d6ae5e";
        let key = "2b7e151628aed2a6abf7158809cf4f3c";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_128_ofb(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes192_ctr() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710";
        let ct = "1abc932417521ca24f2b0459fe7e6e0b090339ec0aa6faefd5ccc2c6f4ce8e941e36b26bd1ebc670d1bd1d665620abf74f78a7f6d29809585a97daec58c6b050";
        let key = "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b";
        let iv = "f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff";

        cipher_test(super::Cipher::aes_192_ctr(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes192_cfb1() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1";
        let ct = "9359";
        let key = "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_192_cfb1(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes192_cfb128() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710";
        let ct = "cdc80d6fddf18cab34c25909c99a417467ce7f7f81173621961a2b70171d3d7a2e1e8a1dd59b88b1c8e60fed1efac4c9c05f9f9ca9834fa042ae8fba584b09ff";
        let key = "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_192_cfb128(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes192_cfb8() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1bee22e409f96e93d7e117393172aae2d";
        let ct = "cda2521ef0a905ca44cd057cbf0d47a0678a";
        let key = "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_192_cfb8(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes192_ofb() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710";
        let ct = "cdc80d6fddf18cab34c25909c99a4174fcc28b8d4c63837c09e81700c11004018d9a9aeac0f6596f559c6d4daf59a5f26d9f200857ca6c3e9cac524bd9acc92a";
        let key = "8e73b0f7da0e6452c810f32b809079e562f8ead2522c6b7b";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_192_ofb(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes256_cfb1() {
        let pt = "6bc1";
        let ct = "9029";
        let key = "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_256_cfb1(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes256_cfb128() {
        let pt = "6bc1bee22e409f96e93d7e117393172a";
        let ct = "dc7e84bfda79164b7ecd8486985d3860";
        let key = "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_256_cfb128(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes256_cfb8() {
        let pt = "6bc1bee22e409f96e93d7e117393172aae2d";
        let ct = "dc1f1a8520a64db55fcc8ac554844e889700";
        let key = "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_256_cfb8(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes256_ofb() {
        // Lifted from http://csrc.nist.gov/publications/nistpubs/800-38a/sp800-38a.pdf

        let pt = "6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710";
        let ct = "dc7e84bfda79164b7ecd8486985d38604febdc6740d20b3ac88f6ad82a4fb08d71ab47a086e86eedf39d1c5bba97c4080126141d67f37be8538f5a8be740e484";
        let key = "603deb1015ca71be2b73aef0857d77811f352c073b6108d72d9810a30914dff4";
        let iv = "000102030405060708090a0b0c0d0e0f";

        cipher_test(super::Cipher::aes_256_ofb(), pt, ct, key, iv);
    }

    #[test]
    fn test_bf_cbc() {
        // https://www.schneier.com/code/vectors.txt

        let pt = "37363534333231204E6F77206973207468652074696D6520666F722000000000";
        let ct = "6B77B4D63006DEE605B156E27403979358DEB9E7154616D959F1652BD5FF92CC";
        let key = "0123456789ABCDEFF0E1D2C3B4A59687";
        let iv = "FEDCBA9876543210";

        cipher_test_nopad(super::Cipher::bf_cbc(), pt, ct, key, iv);
    }

    #[test]
    fn test_bf_ecb() {
        let pt = "5CD54CA83DEF57DA";
        let ct = "B1B8CC0B250F09A0";
        let key = "0131D9619DC1376E";
        let iv = "0000000000000000";

        cipher_test_nopad(super::Cipher::bf_ecb(), pt, ct, key, iv);
    }

    #[test]
    fn test_bf_cfb64() {
        let pt = "37363534333231204E6F77206973207468652074696D6520666F722000";
        let ct = "E73214A2822139CAF26ECF6D2EB9E76E3DA3DE04D1517200519D57A6C3";
        let key = "0123456789ABCDEFF0E1D2C3B4A59687";
        let iv = "FEDCBA9876543210";

        cipher_test_nopad(super::Cipher::bf_cfb64(), pt, ct, key, iv);
    }

    #[test]
    fn test_bf_ofb() {
        let pt = "37363534333231204E6F77206973207468652074696D6520666F722000";
        let ct = "E73214A2822139CA62B343CC5B65587310DD908D0C241B2263C2CF80DA";
        let key = "0123456789ABCDEFF0E1D2C3B4A59687";
        let iv = "FEDCBA9876543210";

        cipher_test_nopad(super::Cipher::bf_ofb(), pt, ct, key, iv);
    }

    #[test]
    fn test_des_cbc() {
        let pt = "54686973206973206120746573742e";
        let ct = "6f2867cfefda048a4046ef7e556c7132";
        let key = "7cb66337f3d3c0fe";
        let iv = "0001020304050607";

        cipher_test(super::Cipher::des_cbc(), pt, ct, key, iv);
    }

    #[test]
    fn test_des_ecb() {
        let pt = "54686973206973206120746573742e";
        let ct = "0050ab8aecec758843fe157b4dde938c";
        let key = "7cb66337f3d3c0fe";
        let iv = "0001020304050607";

        cipher_test(super::Cipher::des_ecb(), pt, ct, key, iv);
    }

    #[test]
    fn test_des_ede3() {
        let pt = "9994f4c69d40ae4f34ff403b5cf39d4c8207ea5d3e19a5fd";
        let ct = "9e5c4297d60582f81071ac8ab7d0698d4c79de8b94c519858207ea5d3e19a5fd";
        let key = "010203040506070801020304050607080102030405060708";
        let iv = "5cc118306dc702e4";

        cipher_test(super::Cipher::des_ede3(), pt, ct, key, iv);
    }

    #[test]
    fn test_des_ede3_cbc() {
        let pt = "54686973206973206120746573742e";
        let ct = "6f2867cfefda048a4046ef7e556c7132";
        let key = "7cb66337f3d3c0fe7cb66337f3d3c0fe7cb66337f3d3c0fe";
        let iv = "0001020304050607";

        cipher_test(super::Cipher::des_ede3_cbc(), pt, ct, key, iv);
    }

    #[test]
    fn test_des_ede3_cfb64() {
        let pt = "2b1773784b5889dc788477367daa98ad";
        let ct = "6f2867cfefda048a4046ef7e556c7132";
        let key = "7cb66337f3d3c0fe7cb66337f3d3c0fe7cb66337f3d3c0fe";
        let iv = "0001020304050607";

        cipher_test(super::Cipher::des_ede3_cfb64(), pt, ct, key, iv);
    }

    #[test]
    fn test_aes128_gcm() {
        let key = "0e00c76561d2bd9b40c3c15427e2b08f";
        let iv = "492cadaccd3ca3fbc9cf9f06eb3325c4e159850b0dbe98199b89b7af528806610b6f63998e1eae80c348e7\
             4cbb921d8326631631fc6a5d304f39166daf7ea15fa1977f101819adb510b50fe9932e12c5a85aa3fd1e73\
             d8d760af218be829903a77c63359d75edd91b4f6ed5465a72662f5055999e059e7654a8edc921aa0d496";
        let pt = "fef03c2d7fb15bf0d2df18007d99f967c878ad59359034f7bb2c19af120685d78e32f6b8b83b032019956c\
             a9c0195721476b85";
        let aad = "d8f1163d8c840292a2b2dacf4ac7c36aff8733f18fabb4fa5594544125e03d1e6e5d6d0fd61656c8d8f327\
             c92839ae5539bb469c9257f109ebff85aad7bd220fdaa95c022dbd0c7bb2d878ad504122c943045d3c5eba\
             8f1f56c0";
        let ct = "4f6cf471be7cbd2575cd5a1747aea8fe9dea83e51936beac3e68f66206922060c697ffa7af80ad6bb68f2c\
             f4fc97416ee52abe";
        let tag = "e20b6655";

        // this tag is smaller than you'd normally want, but I pulled this test from the part of
        // the NIST test vectors that cover 4 byte tags.
        let mut actual_tag = [0; 4];
        let out = encrypt_aead(
            Cipher::aes_128_gcm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(iv).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(pt).unwrap(),
            &mut actual_tag,
        )
        .unwrap();
        assert_eq!(ct, hex::encode(out));
        assert_eq!(tag, hex::encode(actual_tag));

        let out = decrypt_aead(
            Cipher::aes_128_gcm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(iv).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(ct).unwrap(),
            &Vec::from_hex(tag).unwrap(),
        )
        .unwrap();
        assert_eq!(pt, hex::encode(out));
    }

    #[test]
    fn test_aes128_ccm() {
        let key = "3ee186594f110fb788a8bf8aa8be5d4a";
        let nonce = "44f705d52acf27b7f17196aa9b";
        let aad = "2c16724296ff85e079627be3053ea95adf35722c21886baba343bd6c79b5cb57";

        let pt = "d71864877f2578db092daba2d6a1f9f4698a9c356c7830a1";
        let ct = "b4dd74e7a0cc51aea45dfb401a41d5822c96901a83247ea0";
        let tag = "d6965f5aa6e31302a9cc2b36";

        let mut actual_tag = [0; 12];
        let out = encrypt_aead(
            Cipher::aes_128_ccm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(nonce).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(pt).unwrap(),
            &mut actual_tag,
        )
        .unwrap();

        assert_eq!(ct, hex::encode(out));
        assert_eq!(tag, hex::encode(actual_tag));

        let out = decrypt_aead(
            Cipher::aes_128_ccm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(nonce).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(ct).unwrap(),
            &Vec::from_hex(tag).unwrap(),
        )
        .unwrap();
        assert_eq!(pt, hex::encode(out));
    }

    #[test]
    fn test_aes128_ccm_verify_fail() {
        let key = "3ee186594f110fb788a8bf8aa8be5d4a";
        let nonce = "44f705d52acf27b7f17196aa9b";
        let aad = "2c16724296ff85e079627be3053ea95adf35722c21886baba343bd6c79b5cb57";

        let ct = "b4dd74e7a0cc51aea45dfb401a41d5822c96901a83247ea0";
        let tag = "00005f5aa6e31302a9cc2b36";

        let out = decrypt_aead(
            Cipher::aes_128_ccm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(nonce).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(ct).unwrap(),
            &Vec::from_hex(tag).unwrap(),
        );
        assert!(out.is_err());
    }

    #[test]
    fn test_aes256_ccm() {
        let key = "7f4af6765cad1d511db07e33aaafd57646ec279db629048aa6770af24849aa0d";
        let nonce = "dde2a362ce81b2b6913abc3095";
        let aad = "404f5df97ece7431987bc098cce994fc3c063b519ffa47b0365226a0015ef695";

        let pt = "7ebef26bf4ecf6f0ebb2eb860edbf900f27b75b4a6340fdb";
        let ct = "353022db9c568bd7183a13c40b1ba30fcc768c54264aa2cd";
        let tag = "2927a053c9244d3217a7ad05";

        let mut actual_tag = [0; 12];
        let out = encrypt_aead(
            Cipher::aes_256_ccm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(nonce).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(pt).unwrap(),
            &mut actual_tag,
        )
        .unwrap();

        assert_eq!(ct, hex::encode(out));
        assert_eq!(tag, hex::encode(actual_tag));

        let out = decrypt_aead(
            Cipher::aes_256_ccm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(nonce).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(ct).unwrap(),
            &Vec::from_hex(tag).unwrap(),
        )
        .unwrap();
        assert_eq!(pt, hex::encode(out));
    }

    #[test]
    fn test_aes256_ccm_verify_fail() {
        let key = "7f4af6765cad1d511db07e33aaafd57646ec279db629048aa6770af24849aa0d";
        let nonce = "dde2a362ce81b2b6913abc3095";
        let aad = "404f5df97ece7431987bc098cce994fc3c063b519ffa47b0365226a0015ef695";

        let ct = "353022db9c568bd7183a13c40b1ba30fcc768c54264aa2cd";
        let tag = "0000a053c9244d3217a7ad05";

        let out = decrypt_aead(
            Cipher::aes_256_ccm(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(nonce).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(ct).unwrap(),
            &Vec::from_hex(tag).unwrap(),
        );
        assert!(out.is_err());
    }

    #[test]
    #[cfg(any(ossl110))]
    fn test_chacha20() {
        let key = "0000000000000000000000000000000000000000000000000000000000000000";
        let iv = "00000000000000000000000000000000";
        let pt =
            "000000000000000000000000000000000000000000000000000000000000000000000000000000000\
             00000000000000000000000000000000000000000000000";
        let ct =
            "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7da41597c5157488d7\
             724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586";

        cipher_test(Cipher::chacha20(), pt, ct, key, iv);
    }

    #[test]
    #[cfg(any(ossl110))]
    fn test_chacha20_poly1305() {
        let key = "808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f";
        let iv = "070000004041424344454647";
        let aad = "50515253c0c1c2c3c4c5c6c7";
        let pt =
            "4c616469657320616e642047656e746c656d656e206f662074686520636c617373206f66202739393\
             a204966204920636f756c64206f6666657220796f75206f6e6c79206f6e652074697020666f722074\
             6865206675747572652c2073756e73637265656e20776f756c642062652069742e";
        let ct =
            "d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d63dbea45e8ca967128\
             2fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b3692ddbd7f2d778b8c9803aee328091b58fa\
             b324e4fad675945585808b4831d7bc3ff4def08e4b7a9de576d26586cec64b6116";
        let tag = "1ae10b594f09e26a7e902ecbd0600691";

        let mut actual_tag = [0; 16];
        let out = encrypt_aead(
            Cipher::chacha20_poly1305(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(iv).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(pt).unwrap(),
            &mut actual_tag,
        )
        .unwrap();
        assert_eq!(ct, hex::encode(out));
        assert_eq!(tag, hex::encode(actual_tag));

        let out = decrypt_aead(
            Cipher::chacha20_poly1305(),
            &Vec::from_hex(key).unwrap(),
            Some(&Vec::from_hex(iv).unwrap()),
            &Vec::from_hex(aad).unwrap(),
            &Vec::from_hex(ct).unwrap(),
            &Vec::from_hex(tag).unwrap(),
        )
        .unwrap();
        assert_eq!(pt, hex::encode(out));
    }
}
