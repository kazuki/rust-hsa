use std::mem;
use std::ffi::CString;
use std::ops::Drop;

use libc::c_void;
use super::{ErrorType, to_result};
use native::*;

pub struct Program {
    handle: ProgramHandle,
}

impl Program {
    pub fn new(machine_model: MachineModel,
               profile: Profile,
               default_float_rouding_mode: DefaultFloatRoundingMode,
               options: &str) -> Result<Program, ErrorType> {
        let handle: ProgramHandle = unsafe { mem::zeroed() };
        match CString::new(options) {
            Ok(pstr) => {
                to_result(unsafe {
                    hsa_ext_program_create(
                        machine_model, profile,
                        default_float_rouding_mode,
                        pstr.as_ptr(), &handle)
                }, Program {
                    handle: handle
                })
            },
            Err(_) => Err(ErrorType::Exception),
        }
    }

    pub fn add_module(&mut self, module: &Vec<u8>) -> Result<(), ErrorType> {
        to_result(unsafe {
            hsa_ext_program_add_module(self.handle.clone(),
                                       module.as_ptr() as *const c_void)
        }, ())
    }

    pub fn finalize(&mut self, isa: ISA, call_convention: i32,
                    options: &str,
                    code_object_type: CodeObjectType) -> Result<CodeObject, ErrorType> {
        match CString::new(options) {
            Ok(pstr) => {
                let obj: CodeObject = unsafe { mem::zeroed() };
                let control_directives: ControlDirectives = unsafe { mem::zeroed() };
                to_result(unsafe {
                    hsa_ext_program_finalize(self.handle.clone(),
                                             isa,
                                             call_convention,
                                             control_directives,
                                             pstr.as_ptr(), code_object_type, &obj)
                }, obj)
            },
            Err(_) => Err(ErrorType::Exception),
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_ext_program_destroy(self.handle.clone());
            }
            self.handle.handle = 0;
        }
    }
}

impl Agent {
    pub fn create_program(&self, options: &str) -> Result<Program, ErrorType> {
        Program::new(try!(self.machine_model()),
                     try!(self.profile()),
                     DefaultFloatRoundingMode::Default, options)
    }

    pub fn create_and_finalize_program(&self, modules: &Vec<Vec<u8>>,
                                       create_options: &str,
                                       finalize_options: &str) -> Result<CodeObject, ErrorType> {
        let mut prog = try!(self.create_program(&create_options));
        for module in modules.iter() {
            try!(prog.add_module(&module));
        }
        prog.finalize(try!(self.isa()), 0, &finalize_options, CodeObjectType::Program)
    }
}
