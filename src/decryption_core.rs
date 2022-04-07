use crate::dumper::DumperError;
use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use aes_gcm::Aes256Gcm;
use std::{ptr, slice};
use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wincrypt::DATA_BLOB;

/// Decryption for chrome v80 based browsers
pub fn aes_gcm_256(key_buf: &mut [u8], pwd_buf: &[u8]) -> Result<String, DumperError> {
    let key = GenericArray::from_slice(key_buf);
    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&pwd_buf[3..15]);
    let plaintext = cipher
        .decrypt(nonce, &pwd_buf[15..])
        .map_err(|_| DumperError::AesFailedToDecrypt)?;

    String::from_utf8(plaintext).map_err(|_| DumperError::FromUtf8Error)
}

/// Wrapper around DPAPI `CryptUnprotectData`
pub fn crypt_unprotect_data(data_buf: &mut [u8]) -> Result<Vec<u8>, DumperError> {
    let buf_ptr = data_buf.as_mut_ptr();
    let buf_len = data_buf.len();
    let mut data_in = DATA_BLOB {
        cbData: buf_len as u32,
        pbData: buf_ptr,
    };

    let mut data_out = unsafe { std::mem::zeroed() };

    let unprotect_result = unsafe {
        CryptUnprotectData(
            &mut data_in,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            &mut data_out,
        )
    };

    if unprotect_result == 0 {
        let error = unsafe { GetLastError() };
        return Err(DumperError::DpapiFailedToDecrypt(error));
    }

    // SAFETY: We cannot use Vec::from_raw_parts because the data is not allocated by Vec
    // Hence, we just take a slice of it then allocate a new buffer 
    // See: https://github.com/BudiNverse/chrome-pwd-dumper-rs/issues/5
    let buf = unsafe { slice::from_raw_parts(data_out.pbData, data_out.cbData as usize) }.to_vec();

    Ok(buf)
}
