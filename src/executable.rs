use std::ffi::CString;
use std::os::raw::c_void;
use std::mem::zeroed;

use native::*;
use native::Executable as ExecutableHandle;
use code_object::CodeObject;
use super::{check, get_info, iter_callback_helper, ErrorStatus};

pub struct Executable {
    handle: ExecutableHandle,
}

impl Executable {
    pub fn new<T: Into<Vec<u8>>>(
        profile: Profile,
        default_float_rounding_mode: DefaultFloatRoundingMode,
        options: T,
    ) -> Result<Executable, ErrorStatus> {
        let mode = default_float_rounding_mode;
        unsafe {
            let opt = CString::from_vec_unchecked(options.into());
            let handle: ExecutableHandle = zeroed();
            check(hsa_executable_create_alt(profile, mode, opt.as_ptr(), &handle), ())
                .map(|_| Executable { handle: handle })
        }
    }

    #[deprecated]
    pub fn load_code_object<T: Into<Vec<u8>>>(
        &self,
        agent: Agent,
        code_object: CodeObject,
        options: T,
    ) -> Result<(), ErrorStatus> {
        unsafe {
            let opt = CString::from_vec_unchecked(options.into());
            check(
                hsa_executable_load_code_object(
                    self.handle,
                    agent,
                    code_object.handle,
                    opt.as_ptr(),
                ),
                (),
            )
        }
    }

    pub fn freeze<T: Into<Vec<u8>>>(&self, options: T) -> Result<(), ErrorStatus> {
        unsafe {
            let opt = CString::from_vec_unchecked(options.into());
            check(hsa_executable_freeze(self.handle, opt.as_ptr()), ())
        }
    }

    #[deprecated]
    pub fn get_symbol<T: Into<Vec<u8>>>(
        &self,
        module_name: T,
        symbol_name: T,
        agent: Agent,
        call_convention: i32,
    ) -> Result<ExecutableSymbol, ErrorStatus> {
        unsafe {
            let symbol: ExecutableSymbol = zeroed();
            check(
                hsa_executable_get_symbol(
                    self.handle,
                    CString::from_vec_unchecked(module_name.into()).as_ptr(),
                    CString::from_vec_unchecked(symbol_name.into()).as_ptr(),
                    agent,
                    call_convention,
                    &symbol,
                ),
                symbol,
            )
        }
    }

    pub fn agent_symbols(&self, agent: Agent) -> Result<Vec<ExecutableSymbol>, ErrorStatus> {
        let mut v: Vec<ExecutableSymbol> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe {
                hsa_executable_iterate_agent_symbols(self.handle, agent, symbol_list_callback2, p)
            },
            v,
        )
    }

    pub fn program_symbols(&self) -> Result<Vec<ExecutableSymbol>, ErrorStatus> {
        let mut v: Vec<ExecutableSymbol> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe { hsa_executable_iterate_program_symbols(self.handle, symbol_list_callback, p) },
            v,
        )
    }

    #[deprecated]
    pub fn symbols(&self) -> Result<Vec<ExecutableSymbol>, ErrorStatus> {
        let mut v: Vec<ExecutableSymbol> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe { hsa_executable_iterate_symbols(self.handle, symbol_list_callback, p) },
            v,
        )
    }

    pub fn profile(&self) -> Result<Profile, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableInfo::Profile, x))
    }

    pub fn state(&self) -> Result<ExecutableState, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableInfo::State, x))
    }

    pub fn default_float_rounding_mode(&self) -> Result<DefaultFloatRoundingMode, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableInfo::DefaultFloatRoundingMode, x)
        })
    }

    fn get_info(&self, attr: ExecutableInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_executable_get_info(self.handle, attr, v) }
    }
}

impl Drop for Executable {
    fn drop(&mut self) {
        if self.handle.handle != 0 {
            unsafe {
                hsa_executable_destroy(self.handle);
            }
        }
        self.handle.handle = 0;
    }
}

extern "C" fn symbol_list_callback(
    _: ExecutableHandle,
    symbol: ExecutableSymbol,
    data: *mut c_void,
) -> HSAStatus {
    iter_callback_helper(symbol, data)
}

extern "C" fn symbol_list_callback2(
    _: ExecutableHandle,
    _: Agent,
    symbol: ExecutableSymbol,
    data: *mut c_void,
) -> HSAStatus {
    iter_callback_helper(symbol, data)
}

impl ExecutableSymbol {
    pub fn kind(&self) -> Result<SymbolKind, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::Type, x))
    }

    #[deprecated]
    pub fn name(&self) -> Result<String, ErrorStatus> {
        let len: u32 = get_info(|x| self.get_info(ExecutableSymbolInfo::NameLength, x))?;
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize + 1);
        unsafe {
            buf.set_len(len as usize);
            check(
                hsa_executable_symbol_get_info(
                    *self,
                    ExecutableSymbolInfo::Name,
                    buf.as_mut_ptr() as *mut c_void,
                ),
                (),
            ).map(|_| String::from_utf8_lossy(&buf).to_string())
        }
    }

    #[deprecated]
    pub fn module_name(&self) -> Result<String, ErrorStatus> {
        let len: u32 = get_info(|x| self.get_info(ExecutableSymbolInfo::ModuleNameLength, x))?;
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize + 1);
        unsafe {
            buf.set_len(len as usize);
            check(
                hsa_executable_symbol_get_info(
                    *self,
                    ExecutableSymbolInfo::ModuleName,
                    buf.as_mut_ptr() as *mut c_void,
                ),
                (),
            ).map(|_| String::from_utf8_lossy(&buf).to_string())
        }
    }

    pub fn linker_name(&self) -> Result<String, ErrorStatus> {
        let len: u32 = get_info(|x| self.get_info(ExecutableSymbolInfo::LinkerNameLength, x))?;
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize + 1);
        unsafe {
            buf.set_len(len as usize);
            check(
                hsa_executable_symbol_get_info(
                    *self,
                    ExecutableSymbolInfo::LinkerName,
                    buf.as_mut_ptr() as *mut c_void,
                ),
                (),
            ).map(|_| String::from_utf8_lossy(&buf).to_string())
        }
    }

    pub fn agent(&self) -> Result<Agent, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::Agent, x))
    }

    pub fn variable_address(&self) -> Result<u64, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::VariableAddress, x))
    }

    pub fn linkage(&self) -> Result<SymbolKindLinkage, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::Linkage, x))
    }

    pub fn is_definition(&self) -> Result<bool, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::IsDefinition, x))
    }

    #[deprecated]
    pub fn variable_allocation(&self) -> Result<VariableAllocation, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::VariableAllocation, x)
        })
    }

    #[deprecated]
    pub fn variable_segment(&self) -> Result<VariableSegment, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::VariableSegment, x))
    }

    #[deprecated]
    pub fn variable_alignment(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::VariableAlignment, x)
        })
    }

    #[deprecated]
    pub fn variable_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::VariableSize, x))
    }

    #[deprecated]
    pub fn variable_is_const(&self) -> Result<bool, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::VariableIsConst, x))
    }

    pub fn kernel_object(&self) -> Result<u64, ErrorStatus> {
        get_info(|x| self.get_info(ExecutableSymbolInfo::KernelObject, x))
    }

    pub fn kernel_kernarg_segment_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::KernelKernArgSegmentSize, x)
        })
    }

    pub fn kernel_kernarg_segment_alignment(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::KernelKernArgSegmentAlignment, x)
        })
    }

    pub fn kernel_group_segment_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::KernelGroupSegmentSize, x)
        })
    }

    pub fn kernel_private_segment_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::KernelPrivateSegmentSize, x)
        })
    }

    pub fn kernel_dynamic_callstack(&self) -> Result<bool, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::KernelDynamicCallstack, x)
        })
    }

    #[deprecated]
    pub fn kernel_call_convention(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::KernelCallConvertion, x)
        })
    }

    #[cfg(target_pointer_width = "32")]
    pub fn indirect_function_object(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::IndirectFunctionObject, x)
        })
    }

    #[cfg(target_pointer_width = "64")]
    pub fn indirect_function_object(&self) -> Result<u64, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::IndirectFunctionObject, x)
        })
    }

    #[deprecated]
    pub fn indirect_function_call_convention(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ExecutableSymbolInfo::IndirectFunctionCallConvertion, x)
        })
    }

    fn get_info(&self, attr: ExecutableSymbolInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_executable_symbol_get_info(*self, attr, v) }
    }
}
