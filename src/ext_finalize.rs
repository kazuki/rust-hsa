use std::ffi::CString;
use std::os::raw::{c_void, c_char};
use std::mem::zeroed;
use std::ptr::null;

use native::*;
use native::CodeObject as CodeObjectHandle;
use code_object::CodeObject;
use super::{check, get_info, ErrorStatus};

pub struct ExtProgram {
    handle: ExtProgramHandle,
}

impl ExtProgram {
    pub fn new(
        machine_model: MachineModel,
        profile: Profile,
        default_float_rouding_mode: DefaultFloatRoundingMode,
        options: Option<&str>,
    ) -> Result<ExtProgram, ErrorStatus> {
        unsafe {
            let opt = match options {
                Some(x) => x.as_ptr(),
                None => null(),
            } as *const c_char;
            let prog: ExtProgramHandle = zeroed();
            check(
                hsa_ext_program_create(
                    machine_model,
                    profile,
                    default_float_rouding_mode,
                    opt,
                    &prog,
                ),
                (),
            ).map(|_| ExtProgram { handle: prog })
        }
    }

    pub fn add_module(&self, module: &[u8]) -> Result<(), ErrorStatus> {
        let ptr = module.as_ptr() as *const c_void;
        unsafe { check(hsa_ext_program_add_module(self.handle, ptr), ()) }
    }

    #[deprecated]
    pub fn finalize<T: Into<Vec<u8>>>(
        &self,
        isa: ISA,
        options: T,
        code_object_type: CodeObjectType,
    ) -> Result<CodeObject, ErrorStatus> {
        unsafe {
            let opt = CString::from_vec_unchecked(options.into());
            let handle: CodeObjectHandle = zeroed();
            let directives: ExtControlDirectives = zeroed();
            check(
                hsa_ext_program_finalize(
                    self.handle,
                    isa,
                    0,
                    directives,
                    opt.as_ptr(),
                    code_object_type,
                    &handle,
                ),
                (),
            ).map(|_| CodeObject { handle: handle })
        }
    }

    pub fn agent_code_object_finalize(
        &self,
        isa: ISA,
        options: Option<&str>,
        writer: &ExtCodeObjectWriter,
    ) -> Result<(), ErrorStatus> {
        let opt = match options {
            Some(x) => x.as_ptr(),
            None => null(),
        } as *const c_char;
        unsafe {
            check(hsa_ext_agent_code_object_finalize(
                self.handle, isa, opt, &(writer.handle)), ())
        }
    }

    pub fn state(&self) -> Result<MachineModel, ErrorStatus> {
        get_info(|x| self.get_info(ExtProgramInfo::MachineModel, x))
    }

    pub fn profile(&self) -> Result<Profile, ErrorStatus> {
        get_info(|x| self.get_info(ExtProgramInfo::Profile, x))
    }

    pub fn default_float_rounding_mode(&self) -> Result<DefaultFloatRoundingMode, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExtProgramInfo::DefaultFloatRoundingMode, x)
        })
    }

    fn get_info(&self, attr: ExtProgramInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_ext_program_get_info(self.handle, attr, v) }
    }
}

impl Drop for ExtProgram {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_ext_program_destroy(self.handle);
            }
        }
        self.handle.handle = 0;
    }
}

pub struct ExtCodeObjectWriter {
    handle: ExtCodeObjectWriterHandle,
    buffer: Vec<u8>,
}

impl ExtCodeObjectWriter {
    pub fn new() -> Result<ExtCodeObjectWriter, ErrorStatus> {
        let mut buf: Vec<u8> = Vec::new();
        let p: *mut c_void = &mut buf as *mut _ as *mut c_void;
        unsafe {
            let handle: ExtCodeObjectWriterHandle = zeroed();
            check(
                hsa_ext_code_object_writer_create_from_memory(
                    object_writer_memory_alloc,
                    p,
                    &handle,
                ),
                (),
            ).map(|_| {
                ExtCodeObjectWriter {
                    handle: handle,
                    buffer: buf,
                }
            })
        }
    }
}

impl Drop for ExtCodeObjectWriter {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_ext_code_object_writer_destroy(self.handle);
            }
        }
        self.handle.handle = 0;
        self.buffer.clear();
    }
}

extern "C" fn object_writer_memory_alloc(
    size: usize,
    align: usize,
    ptr: *mut *mut c_void,
    data: *mut c_void,
) -> HSAStatus {
    let v: *mut Vec<u8> = data as *mut Vec<u8>;
    println!("ALLOC: size={} align={}", size, align);
    unsafe {
        (*v).resize(size, 0);
        (*ptr) = (*v).as_mut_ptr() as *mut c_void;
    }
    0
}
