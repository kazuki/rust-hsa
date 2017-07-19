use std::os::raw::{c_void, c_char};
use std::mem::zeroed;

use native::*;
use super::{bitflags, check, get_info, iter_callback_helper, ErrorStatus, Flags};

impl ISA {
    pub fn new(name: &str) -> Result<ISA, ErrorStatus> {
        unsafe {
            let isa = zeroed();
            check(hsa_isa_from_name(name.as_ptr() as *const c_char, &isa), isa)
        }
    }

    pub fn name(&self) -> Result<String, ErrorStatus> {
        let len: u32 = get_info(|x| self.get_info(ISAInfo::NameLength, x))?;
        let mut buf: Vec<u8> = Vec::with_capacity(len as usize + 1);
        unsafe {
            buf.set_len(len as usize);
            let ptr = buf.as_mut_ptr() as *mut c_void;
            check(hsa_isa_get_info_alt(*self, ISAInfo::Name, ptr), ())
                .map(|_| String::from_utf8_lossy(&buf).to_string())
        }
    }

    #[deprecated]
    pub fn call_convention_count(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::CallConvertionCount, x))
    }

    #[deprecated]
    pub fn call_convention_info_wavefront_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ISAInfo::CallConvertionInfoWavefrontSize, x)
        })
    }

    #[deprecated]
    pub fn call_convention_info_wavefronts_per_compute_unit(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(ISAInfo::CallConvertionInfoWavefrontsPerComputeUnit, x)
        })
    }

    pub fn machine_models(&self) -> Result<Flags<MachineModel>, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::MachineModels, x)).map(|x: [bool; 2]| {
            self.boolflags(&x, &[MachineModel::Small, MachineModel::Large])
        })
    }

    pub fn profiles(&self) -> Result<Flags<Profile>, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::Profiles, x)).map(|x: [bool; 2]| {
            self.boolflags(&x, &[Profile::Base, Profile::Full])
        })
    }

    pub fn default_float_rounding_modes(
        &self,
    ) -> Result<Flags<DefaultFloatRoundingMode>, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::DefaultFloatRoundingModes, x)).map(|x: [bool; 3]| {
            self.boolflags(
                &x,
                &[
                    DefaultFloatRoundingMode::Default,
                    DefaultFloatRoundingMode::Zero,
                    DefaultFloatRoundingMode::Near,
                ],
            )
        })
    }

    pub fn base_profile_default_float_rounding_modes(
        &self,
    ) -> Result<Flags<DefaultFloatRoundingMode>, ErrorStatus> {
        get_info(|x| {
            self.get_info(ISAInfo::BaseProfileDefaultFloatRoundingModes, x)
        }).map(|x: [bool; 3]| {
            self.boolflags(
                &x,
                &[
                    DefaultFloatRoundingMode::Default,
                    DefaultFloatRoundingMode::Zero,
                    DefaultFloatRoundingMode::Near,
                ],
            )
        })
    }

    pub fn fast_f16_operation(&self) -> Result<bool, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::FastF16Operation, x))
    }

    pub fn workgroup_max_dim(&self) -> Result<[u16; 3], ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::WorkgroupMaxDim, x))
    }

    pub fn workgroup_max_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::WorkgroupMaxSize, x))
    }

    pub fn grid_max_dim(&self) -> Result<Dim3, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::GridMaxDim, x))
    }

    pub fn grid_max_size(&self) -> Result<u64, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::GridMaxSize, x))
    }

    pub fn fbarrier_max_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(ISAInfo::FbarrierMaxSize, x))
    }

    pub fn exception_policies(
        &self,
        profile: Profile,
    ) -> Result<Flags<ExceptionPolicy>, ErrorStatus> {
        let mut mask: u16 = 0;
        check(unsafe { hsa_isa_get_exception_policies(*self, profile, &mut mask) }, ()).map(|_| {
            bitflags(mask as u32)
        })
    }

    pub fn round_method(
        &self,
        fp_type: FpType,
        flush_mode: FlushMode,
    ) -> Result<RoundMethod, ErrorStatus> {
        unsafe {
            let m: RoundMethod = zeroed();
            check(hsa_isa_get_round_method(*self, fp_type, flush_mode, &m), m)
        }
    }

    pub fn wavefronts(&self) -> Result<Vec<Wavefront>, ErrorStatus> {
        let mut v: Vec<Wavefront> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe { hsa_isa_iterate_wavefronts(*self, wavefront_list_callback, p) },
            v,
        )
    }

    fn boolflags<T>(&self, flags: &[bool], enum_list: &[T]) -> Flags<T>
    where
        T: Eq + Ord + Into<usize> + Copy,
    {
        let mut ret = Flags::new();
        for e in enum_list {
            let idx: usize = (*e).into();
            if flags[idx] {
                ret.insert(*e);
            }
        }
        ret
    }

    fn get_info(&self, attr: ISAInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_isa_get_info_alt(*self, attr, v) }
    }
}

impl Into<usize> for MachineModel {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for Profile {
    fn into(self) -> usize {
        self as usize
    }
}

impl Into<usize> for DefaultFloatRoundingMode {
    fn into(self) -> usize {
        self as usize
    }
}

extern "C" fn wavefront_list_callback(wavefront: Wavefront, data: *mut c_void) -> HSAStatus {
    iter_callback_helper(wavefront, data)
}
