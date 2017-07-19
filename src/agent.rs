use std::os::raw::c_void;

use super::{check, get_fixed_str, get_info, iter_callback_helper, ErrorStatus, FromPrimitive};
use native::*;
use system::get_extension_name;

impl Agent {
    pub fn list() -> Result<Vec<Agent>, ErrorStatus> {
        let mut v: Vec<Agent> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(unsafe { hsa_iterate_agents(agent_list_callback, p) }, v)
    }

    pub fn from_device_type(device_type: DeviceType) -> Result<Vec<Agent>, ErrorStatus> {
        let mut v = try!(Agent::list());
        v.retain(|&agent| match agent.device() {
            Ok(t) => t == device_type,
            Err(_) => false,
        });
        Ok(v)
    }

    pub fn name(&self) -> Result<String, ErrorStatus> {
        get_fixed_str(|x| self.get_info(AgentInfo::Name, x), 64)
    }

    pub fn vendor(&self) -> Result<String, ErrorStatus> {
        get_fixed_str(|x| self.get_info(AgentInfo::VendorName, x), 64)
    }

    pub fn feature(&self) -> Result<AgentFeature, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::Feature, x))
    }

    #[deprecated]
    pub fn machine_model(&self) -> Result<MachineModel, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::MachineModel, x))
    }

    #[deprecated]
    pub fn profile(&self) -> Result<Profile, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::Profile, x))
    }

    #[deprecated]
    pub fn default_float_rounding_mode(&self) -> Result<DefaultFloatRoundingMode, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::DefaultFloatRoundingMode, x))
    }

    #[deprecated]
    pub fn base_profile_default_float_rounding_mode(
        &self,
    ) -> Result<DefaultFloatRoundingMode, ErrorStatus> {
        get_info(|x| {
            self.get_info(AgentInfo::BaseProfileDefaultFloatRoundingModes, x)
        })
    }

    #[deprecated]
    pub fn fast_f16_operation(&self) -> Result<bool, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::FastF16Operation, x))
    }

    #[deprecated]
    pub fn wavefront_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::WavefrontSize, x))
    }

    #[deprecated]
    pub fn workgroup_max_dim(&self) -> Result<[u16; 3], ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::WorkgroupMaxDim, x))
    }

    #[deprecated]
    pub fn workgroup_max_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::WorkgroupMaxSize, x))
    }

    #[deprecated]
    pub fn grid_max_dim(&self) -> Result<Dim3, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::GridMaxDim, x))
    }

    #[deprecated]
    pub fn grid_max_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::GridMaxSize, x))
    }

    #[deprecated]
    pub fn fbarrier_max_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::FbarrierMaxSize, x))
    }

    #[deprecated]
    pub fn queues_max(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::QueuesMax, x))
    }

    pub fn queue_min_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::QueueMinSize, x))
    }

    pub fn queue_max_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::QueueMaxSize, x))
    }

    pub fn queue_type(&self) -> Result<QueueType, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::QueueType, x))
    }

    #[deprecated]
    pub fn node(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::Node, x))
    }

    pub fn device(&self) -> Result<DeviceType, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::Device, x))
    }

    #[deprecated]
    pub fn cache_size(&self) -> Result<[u32; 4], ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::CacheSize, x))
    }

    #[deprecated]
    pub fn isa(&self) -> Result<ISA, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::ISA, x))
    }

    pub fn extensions(&self) -> Result<Vec<Extension>, ErrorStatus> {
        let ret: Result<[u8; 128], ErrorStatus> =
            get_info(|x| self.get_info(AgentInfo::Extensions, x));
        ret.map(|x| {
            let mut v = Vec::new();
            for (i, b) in x.iter().enumerate().filter(|&(_, &b)| b != 0) {
                for j in (0..8).filter(|&j| (*b >> j) & 1 != 0) {
                    if let Some(x) = Extension::from_u16((i * 8 + j) as u16) {
                        v.push(x)
                    }
                }
            }
            v
        })
    }

    pub fn extension_names(&self) -> Result<Vec<String>, ErrorStatus> {
        self.extensions().map(|e| {
            e.iter()
                .filter_map(|id| get_extension_name(*id).ok())
                .collect()
        })
    }

    pub fn version_major(&self) -> Result<u16, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::VersionMajor, x))
    }

    pub fn version_minor(&self) -> Result<u16, ErrorStatus> {
        get_info(|x| self.get_info(AgentInfo::VersionMinor, x))
    }

    fn get_info(&self, attr: AgentInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_agent_get_info(*self, attr, v) }
    }

    pub fn caches(&self) -> Result<Vec<Cache>, ErrorStatus> {
        let mut v: Vec<Cache> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe { hsa_agent_iterate_caches(*self, cache_list_callback, p) },
            v,
        )
    }

    #[deprecated]
    pub fn extension_supported(
        &self,
        extension: Extension,
        version_major: u16,
        version_minor: u16,
    ) -> Result<bool, ErrorStatus> {
        let mut result = false;
        check(
            unsafe {
                hsa_agent_extension_supported(
                    extension,
                    *self,
                    version_major,
                    version_minor,
                    &mut result,
                )
            },
            result,
        )
    }

    pub fn major_extension_supported(
        &self,
        extension: Extension,
        version_major: u16,
    ) -> Result<(u16, bool), ErrorStatus> {
        let mut result = false;
        let mut version_minor = 0u16;
        check(
            unsafe {
                hsa_agent_major_extension_supported(
                    extension,
                    *self,
                    version_major,
                    &mut version_minor,
                    &mut result,
                )
            },
            (version_minor, result),
        )
    }

    pub fn regions(&self) -> Result<Vec<Region>, ErrorStatus> {
        let mut v: Vec<Region> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe { hsa_agent_iterate_regions(*self, region_list_callback, p) },
            v,
        )
    }

    pub fn fine_grained_global_regions(&self) -> Result<Vec<Region>, ErrorStatus> {
        let mut regions = try!(self.regions());
        regions.retain(|&r| {
            match r.segment() {
                Ok(RegionSegment::Global) => (),
                _ => return false,
            }
            match r.global_flags() {
                Ok(x) => x.contains(&RegionGlobalFlag::FineGrained),
                _ => false,
            }
        });
        Ok(regions)
    }

    pub fn kernarg_global_regions(&self) -> Result<Vec<Region>, ErrorStatus> {
        let mut regions = try!(self.regions());
        regions.retain(|&r| {
            match r.segment() {
                Ok(RegionSegment::Global) => (),
                _ => return false,
            }
            match r.global_flags() {
                Ok(x) => x.contains(&RegionGlobalFlag::KernArg),
                _ => false,
            }
        });
        Ok(regions)
    }

    pub fn isas(&self) -> Result<Vec<ISA>, ErrorStatus> {
        let mut v: Vec<ISA> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        check(
            unsafe { hsa_agent_iterate_isas(*self, isa_list_callback, p) },
            v,
        )
    }
}

extern "C" fn agent_list_callback(agent: Agent, data: *mut c_void) -> HSAStatus {
    iter_callback_helper(agent, data)
}

extern "C" fn cache_list_callback(cache: Cache, data: *mut c_void) -> HSAStatus {
    iter_callback_helper(cache, data)
}

extern "C" fn region_list_callback(region: Region, data: *mut c_void) -> HSAStatus {
    iter_callback_helper(region, data)
}

extern "C" fn isa_list_callback(isa: ISA, data: *mut c_void) -> HSAStatus {
    iter_callback_helper(isa, data)
}
