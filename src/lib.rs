use std::ffi::{c_char, c_int, c_ushort, c_void};

extern "C" {
    fn sdk_new_with_connection(connection: *const c_char) -> *mut c_void;
    fn sdk_delete(sdk: *mut c_void);

    fn sdk_is_valid(sdk: *mut c_void) -> bool;
    fn sdk_is_data_interface_valid(sdk: *mut c_void) -> bool;
    fn sdk_get_data_port(sdk: *mut c_void) -> c_ushort;
    fn sdk_is_command_interface_valid(sdk: *mut c_void) -> bool;
    fn sdk_is_command_interface_full_access(sdk: *mut c_void) -> bool;
    fn sdk_get_remote_system_type(sdk: *mut c_void) -> c_int;
    fn sdk_set_data_timeout_us(sdk: *mut c_void, timeout_us: c_int);
    fn sdk_set_command_timeout_us(sdk: *mut c_void, timeout_us: c_int);
    fn sdk_set_controller_timeout_us(sdk: *mut c_void, timeout_us: c_int);
    fn sdk_receive(sdk: *mut c_void) -> bool;
    fn sdk_start_measurement(sdk: *mut c_void) -> bool;
    fn sdk_stop_measurement(sdk: *mut c_void) -> bool;
}

#[derive(Debug)]
pub enum RemoteSystemType {
    Unknown,
    DTrack,
    DTrack2,
}

#[derive(Debug)]
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

    pub fn data_port(&self) -> u16 {
        unsafe { sdk_get_data_port(self.sdk) }
    }

    pub fn is_command_interface_valid(&self) -> bool {
        unsafe { sdk_is_command_interface_valid(self.sdk) }
    }

    pub fn is_command_interface_full_access(&self) -> bool {
        unsafe { sdk_is_command_interface_full_access(self.sdk) }
    }

    pub fn remote_system_type(&self) -> RemoteSystemType {
        match unsafe { sdk_get_remote_system_type(self.sdk) } {
            0 => RemoteSystemType::Unknown,
            1 => RemoteSystemType::DTrack,
            2 => RemoteSystemType::DTrack2,
            _ => unreachable!(),
        }
    }

    pub fn receive(&self) -> bool {
        unsafe { sdk_receive(self.sdk) }
    }

    pub fn start_measurement(&self) -> bool {
        unsafe { sdk_start_measurement(self.sdk) }
    }

    pub fn stop_measurement(&self) -> bool {
        unsafe { sdk_stop_measurement(self.sdk) }
    }
}
