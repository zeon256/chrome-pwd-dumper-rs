use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::wincrypt::{ DATA_BLOB, CRYPTOAPI_BLOB };
use winapi::um::winnt::LPWSTR;

#[derive(Debug)]
pub struct ChromeAccount {
    pub website: String,
    pub username_value: String,
    pub password_value: Vec<u8>,
}

impl ChromeAccount {
    pub fn new(website: String, username_value: String, password_value: Vec<u8>) -> Self {
        ChromeAccount { website, username_value, password_value }
    }

    fn get_clear_text_pw(&mut self) -> String {
        unsafe {
            let vec_ptr: *mut u8 = *&self.password_value.as_mut_ptr();
            let vec_len: &usize = &self.password_value.len();
            let mut data_in: DATA_BLOB = CRYPTOAPI_BLOB { cbData: *vec_len as u32, pbData: vec_ptr };
            let mut data_out: DATA_BLOB = CRYPTOAPI_BLOB { cbData: 0, pbData: &mut 0 };
            let mut p_descr_out: LPWSTR = std::ptr::null_mut();

            let succ_unprotect = CryptUnprotectData(&mut data_in,
                                        &mut p_descr_out,
                                        std::ptr::null_mut(),
                                        std::ptr::null_mut(),
                                        std::ptr::null_mut(),
                                        0,
                                        &mut data_out);
            if succ_unprotect == 0 {
                panic!("Failed to decrypt");
            }

            String::from_raw_parts(data_out.pbData, 16,16)
        }
    }

    pub fn humanize(&mut self) -> String {
        let website = self.website.clone();
        let username = self.username_value.clone();

        if self.password_value.len() > 0 && !self.username_value.is_empty() {
            format!("Website: {}, Username: {}, Password: {}",
                    website, username, &mut self.get_clear_text_pw())
        } else {
            format!("Website: {}, Username: {}, Password: [ INVALID ]",
                    website, username)
        }
    }
}