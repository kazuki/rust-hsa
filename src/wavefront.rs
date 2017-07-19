use std::os::raw::c_void;

use native::*;
use super::{get_info, ErrorStatus};

impl Wavefront {
    pub fn size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(WavefrontInfo::Size, x))
    }

    fn get_info(&self, attr: WavefrontInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_wavefront_get_info(*self, attr, v) }
    }
}
