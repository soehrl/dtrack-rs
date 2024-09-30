use std::ffi::{c_char, c_int, c_uint, c_ushort, c_void};

#[derive(Debug)]
#[repr(C)]
pub struct DTrackBody {
    id: c_int,
    quality: std::ffi::c_double,
    loc: [std::ffi::c_double; 3],
    rot: [std::ffi::c_double; 9],
    covref: [std::ffi::c_double; 3],
    cov: [std::ffi::c_double; 36],
}

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

    // Parser functions
    fn get_latency_usec(sdk: *mut c_void) -> c_uint;
    fn sdk_get_num_body(sdk: *mut c_void) -> c_int;
    fn sdk_get_body(sdk: *mut c_void, index: c_int) -> *const DTrackBody;
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

    pub fn latency(&self) -> std::time::Duration {
        std::time::Duration::from_micros(unsafe { get_latency_usec(self.sdk) as u64 })
    }

    pub fn num_bodies(&self) -> usize {
        unsafe { sdk_get_num_body(self.sdk) as usize } 
    }

    pub fn body(&self, index: usize) -> Option<&DTrackBody> {
        let body = unsafe { sdk_get_body(self.sdk, index as c_int) };
        if body.is_null() {
            None
        } else {
            Some(unsafe { &*body })
        }
    }
}
