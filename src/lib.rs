use std::ffi::{c_char, c_void};

extern "C" {
    fn sdk_new_with_connection(connection: *const c_char) -> *mut c_void;
    fn sdk_delete(sdk: *mut c_void);

    fn sdk_is_valid(sdk: *mut c_void) -> bool;
    fn sdk_is_data_interface_valid(sdk: *mut c_void) -> bool;
}

pub struct DTrack {
    sdk: *mut c_void,
}

impl Drop for DTrack {
    fn drop(&mut self) {
        unsafe {
            // Call the destructor of the C++ object
            // This is necessary to free the memory allocated by the C++ object
            // Otherwise, the memory will leak
            sdk_delete(self.sdk);
        }
    }
}

impl DTrack {
    pub fn with_connection(connection: &str) -> Result<DTrack, std::ffi::NulError> {
        let connection = std::ffi::CString::new(connection)?;
        let sdk = unsafe { sdk_new_with_connection(connection.as_ptr()) };
        Ok(DTrack { sdk })
    }

    pub fn is_valid(&self) -> bool {
        unsafe { sdk_is_valid(self.sdk) }
    }

    pub fn is_data_interface_valid(&self) -> bool {
        unsafe { sdk_is_data_interface_valid(self.sdk) }
    }
}
