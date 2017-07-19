use std::os::raw::{c_char, c_void};

use super::{check, get_info, ErrorStatus};
use native::*;

impl Cache {
    pub fn name(&self) -> Result<String, ErrorStatus> {
        unsafe {
            use std::ptr::null;
            use std::ffi::CStr;
            let mut p: *const c_char = null();
            let pp: *mut c_void = &mut p as *mut _ as *mut c_void;
            check(hsa_cache_get_info(*self, CacheInfo::Name, pp), ())
                .map(|_| CStr::from_ptr(p).to_string_lossy().to_string())
        }
    }

    pub fn level(&self) -> Result<u8, ErrorStatus> {
        get_info(|x| self.get_info(CacheInfo::Level, x))
    }

    pub fn size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(CacheInfo::Size, x))
    }

    fn get_info(&self, attr: CacheInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_cache_get_info(*self, attr, v) }
    }
}
