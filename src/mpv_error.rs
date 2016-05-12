use std::error::Error;
use std::{result, ffi, fmt};
use num::FromPrimitive;

use mpv_gen::mpv_error_string;
pub use mpv_gen::MpvError;

pub type Result<T> = result::Result<T, MpvError>;

impl Error for MpvError {
    fn description(&self) -> &str {
        let str_ptr = unsafe { mpv_error_string(*self as ::std::os::raw::c_int) };
        assert!(!str_ptr.is_null());
        unsafe { ffi::CStr::from_ptr(str_ptr).to_str().unwrap() }
    }
}

impl fmt::Display for MpvError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.description(), self)
    }
}

pub fn ret_to_result<T>(ret: i32, default: T) -> Result<T> {
    if ret < 0 {
        Err(MpvError::from_i32(ret).unwrap())
    } else {
        Ok(default)
    }
}
