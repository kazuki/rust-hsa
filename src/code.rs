use std;
use std::mem;
use std::ffi::CString;
use std::ops::Drop;

use libc::c_void;
use super::{ErrorType, to_error_type, to_result};
use native::*;

pub struct Executable {
    handle: ExecutableHandle,
}

impl Executable {
    pub fn new(profile: Profile,
               executable_state: ExecutableState,
               options: &str) -> Result<Executable, ErrorType> {
        let handle: ExecutableHandle = unsafe { mem::zeroed() };
        match CString::new(options) {
            Ok(pstr) => {
                to_result(unsafe {
                    hsa_executable_create(profile, executable_state,
                                          pstr.as_ptr(), &handle)
                }, Executable {
                    handle: handle
                })
            },
            Err(_) => Err(ErrorType::Exception),
        }
    }

    pub fn new_and_freeze(agent: &Agent, codes: &Vec<CodeObject>, new_options: &str,
                          load_options: &str, freeze_options: &str) -> Result<Executable, ErrorType> {
        let mut executable = try!(Executable::new(try!(agent.profile()),
                                                  ExecutableState::Unfrozen, &new_options));
        for code in codes.iter() {
            try!(executable.load_code_object(&agent, &code, &load_options));
        }
        try!(executable.freeze(&freeze_options));
        Ok(executable)
    }

    pub fn load_code_object(&mut self, agent: &Agent, code_object: &CodeObject,
                            options: &str) -> Result<(), ErrorType> {
        match CString::new(options) {
            Ok(pstr) => {
                to_result(unsafe {
                    hsa_executable_load_code_object(self.handle.clone(),
                                                    agent.clone(),
                                                    code_object.clone(),
                                                    pstr.as_ptr())
                }, ())
            },
            Err(_) => Err(ErrorType::Exception),
        }
    }

    pub fn freeze(&mut self, options: &str) -> Result<(), ErrorType> {
        match CString::new(options) {
            Ok(pstr) => {
                to_result(unsafe {
                    hsa_executable_freeze(self.handle.clone(),
                                          pstr.as_ptr())
                }, ())
            },
            Err(_) => Err(ErrorType::Exception),
        }
    }

    pub fn get_symbol(&self, module_name: &str, symbol_name: &str, agent: &Agent,
                      call_convention: i32) -> Result<ExecutableSymbol, ErrorType> {
        match CString::new(module_name) {
            Ok(p_mod) => {
                let mut mod_ptr = p_mod.as_ptr();
                if module_name.len() == 0 {
                    mod_ptr = std::ptr::null();
                }
                match CString::new(symbol_name) {
                    Ok(p_sym) => {
                        let symbol: ExecutableSymbol = unsafe { mem::zeroed() };
                        to_result(unsafe {
                            hsa_executable_get_symbol(self.handle.clone(),
                                                      mod_ptr, p_sym.as_ptr(),
                                                      agent.clone(), call_convention,
                                                      &symbol)
                        }, symbol)
                    },
                    Err(_) => Err(ErrorType::Exception),
                }
            },
            Err(_) => Err(ErrorType::Exception),
        }
    }
}

impl Drop for Executable {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_executable_destroy(self.handle.clone());
            }
            self.handle.handle = 0;
        }
    }
}

impl ExecutableSymbol {
    pub fn symbol_type(&self) -> Result<SymbolKind, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::Type)
    }

    pub fn name(&self) -> Result<String, ErrorType> {
        match self.get_info_raw::<u32>(ExecutableSymbolInfo::NameLength) {
            Ok(len) => self.get_info_str(ExecutableSymbolInfo::Name, len as usize),
            Err(e) => Err(e),
        }
    }

    pub fn module_name(&self) -> Result<String, ErrorType> {
        match self.get_info_raw::<u32>(ExecutableSymbolInfo::ModuleNameLength) {
            Ok(len) => self.get_info_str(ExecutableSymbolInfo::ModuleName, len as usize),
            Err(e) => Err(e),
        }
    }

    pub fn agent(&self) -> Result<Agent, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::Agent)
    }

    pub fn variable_address(&self) -> Result<u64, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::VariableAddress)
    }

    pub fn linkage(&self) -> Result<SymbolKindLinkage, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::Linkage)
    }

    pub fn is_definition(&self) -> Result<bool, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::IsDefinition)
    }

    pub fn variable_allocation(&self) -> Result<VariableAllocation, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::VariableAllocation)
    }

    pub fn variable_segment(&self) -> Result<VariableSegment, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::VariableSegment)
    }

    pub fn variable_alignemnt(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::VariableAllocation)
    }

    pub fn variable_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::VariableSize)
    }

    pub fn variable_is_const(&self) -> Result<bool, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::VariableIsConst)
    }

    pub fn kernel_object(&self) -> Result<u64, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::KernelObject)
    }

    pub fn kernel_kernarg_segment_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::KernelKernArgSegmentSize)
    }

    pub fn kernel_kernarg_segment_alignment(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::KernelKernArgSegmentAlignment)
    }

    pub fn kernel_group_segment_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::KernelGroupSegmentSize)
    }

    pub fn kernel_private_segment_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::KernelPrivateSegmentSize)
    }

    pub fn kernel_dynamic_callstack(&self) -> Result<bool, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::KernelDynamicCallstack)
    }

    pub fn indirect_function_object(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::IndirectFunctionObject)
    }

    pub fn indirect_function_call_convention(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(ExecutableSymbolInfo::IndirectFunctionCallConvention)
    }
    
    fn get_info_raw<T>(&self, attr: ExecutableSymbolInfo) -> Result<T, ErrorType> {
        let mut x: T = unsafe { mem::zeroed() };
        let p: *mut c_void = &mut x as *mut _ as *mut c_void;
        unsafe {
            match hsa_executable_symbol_get_info(self.clone(), attr, p) {
                0 => Ok(x),
                e => Err(to_error_type(e))
            }
        }
    }

    fn get_info_str(&self, attr: ExecutableSymbolInfo, size: usize) -> Result<String, ErrorType> {
        let mut buf: Vec<u8> = Vec::with_capacity(size);
        let p: *mut c_void = buf.as_mut_ptr() as *mut _ as *mut c_void;
        unsafe {
            match hsa_executable_symbol_get_info(self.clone(), attr, p) {
                0 => {
                    for i in 0..buf.capacity() {
                        if *buf.get_unchecked(i) == 0 {
                            buf.set_len(i);
                            break;
                        }
                    }
                    Ok(String::from_utf8(buf).unwrap())
                },
                e => Err(to_error_type(e)),
            }
        }
    }
}
