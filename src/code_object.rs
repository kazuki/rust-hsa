use std::os::raw::c_void;

use native::*;
use native::CodeObject as CodeObjectHandle;
use super::{get_info, ErrorStatus};

pub struct CodeObject {
    pub handle: CodeObjectHandle,
}

impl CodeObject {
    pub fn version(&self) -> Result<String, ErrorStatus> {
        get_info(|x| self.get_info(CodeObjectInfo::Version, x)).map(|x: [u8; 64]| {
            let x = x.splitn(2, |c| *c == 0).next().unwrap_or(&[]);
            String::from_utf8_lossy(x).to_string()
        })
    }

    pub fn type_info(&self) -> Result<CodeObjectType, ErrorStatus> {
        get_info(|x| self.get_info(CodeObjectInfo::Type, x))
    }

    pub fn isa(&self) -> Result<ISA, ErrorStatus> {
        get_info(|x| self.get_info(CodeObjectInfo::ISA, x))
    }

    pub fn machine_model(&self) -> Result<MachineModel, ErrorStatus> {
        get_info(|x| self.get_info(CodeObjectInfo::MachineModel, x))
    }

    pub fn profile(&self) -> Result<Profile, ErrorStatus> {
        get_info(|x| self.get_info(CodeObjectInfo::Profile, x))
    }

    pub fn default_float_rounding_mode(&self) -> Result<DefaultFloatRoundingMode, ErrorStatus> {
        get_info(|x| {
            self.get_info(CodeObjectInfo::DefaultFloatRoundingMode, x)
        })
    }

    fn get_info(&self, attr: CodeObjectInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_code_object_get_info(self.handle, attr, v) }
    }
}

impl Drop for CodeObject {
    fn drop(&mut self) {
        unsafe {
            hsa_code_object_destroy(self.handle);
        }
    }
}
