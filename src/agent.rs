use std::mem;
use libc::{c_int, c_void};

use super::{ErrorType, to_error_type, to_result};
use native::*;

impl Agent {
    pub fn list() -> Result<Vec<Agent>, ErrorType> {
        let mut v: Vec<Agent> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        to_result(unsafe {
            hsa_iterate_agents(agent_list_callback, p)
        }, v)
    }

    pub fn name(&self) -> Result<String, ErrorType> {
        self.get_info_str(AgentInfo::Name, 64)
    }

    pub fn vendor(&self) -> Result<String, ErrorType> {
        self.get_info_str(AgentInfo::VendorName, 64)
    }

    pub fn feature(&self) -> Result<AgentFeature, ErrorType> {
        self.get_info_raw(AgentInfo::Feature)
    }

    pub fn machine_model(&self) -> Result<MachineModel, ErrorType> {
        self.get_info_raw(AgentInfo::MachineModel)
    }

    pub fn profile(&self) -> Result<Profile, ErrorType> {
        self.get_info_raw(AgentInfo::Profile)
    }

    pub fn default_float_rouding_mode(&self) -> Result<DefaultFloatRoundingMode, ErrorType> {
        self.get_info_raw(AgentInfo::DefaultFloatRoundingMode)
    }

    pub fn base_profile_default_float_rouding_mode(&self) -> Result<DefaultFloatRoundingMode, ErrorType> {
        self.get_info_raw(AgentInfo::BaseProfileDefaultFloatRoundingModes)
    }

    pub fn fast_f16_operation(&self) -> Result<bool, ErrorType> {
        let r: Result<c_int, ErrorType> = self.get_info_raw(AgentInfo::FastF16Operation);
        match r {
            Ok(i) => {
                if i == 0 {
                    Ok(false)
                } else {
                    Ok(true)
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn wavefront_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::WavefrontSize)
    }

    pub fn workgroup_max_dim(&self) -> Result<[u16;3], ErrorType> {
        self.get_info_raw(AgentInfo::WorkgroupMaxDim)
    }

    pub fn workgroup_max_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::WorkgroupMaxSize)
    }

    pub fn grid_max_dim(&self) -> Result<Dim3, ErrorType> {
        self.get_info_raw(AgentInfo::GridMaxDim)
    }

    pub fn grid_max_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::GridMaxSize)
    }

    pub fn fbarrier_max_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::FbarrierMaxSize)
    }

    pub fn queues_max(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::QueuesMax)
    }

    pub fn queue_min_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::QueueMinSize)
    }

    pub fn queue_max_size(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::QueueMaxSize)
    }

    pub fn queue_type(&self) -> Result<QueueType, ErrorType> {
        self.get_info_raw(AgentInfo::QueueType)
    }

    pub fn node(&self) -> Result<u32, ErrorType> {
        self.get_info_raw(AgentInfo::Node)
    }

    pub fn device(&self) -> Result<DeviceType, ErrorType> {
        self.get_info_raw(AgentInfo::Device)
    }

    pub fn isa(&self) -> Result<ISA, ErrorType> {
        self.get_info_raw(AgentInfo::ISA)
    }

    pub fn extensions(&self) -> Result<[u8;128], ErrorType> {
        self.get_info_raw(AgentInfo::Extensions)
    }

    pub fn major_version(&self) -> Result<u16, ErrorType> {
        self.get_info_raw(AgentInfo::VersionMajor)
    }

    pub fn minor_version(&self) -> Result<u16, ErrorType> {
        self.get_info_raw(AgentInfo::VersionMinor)
    }

    pub fn assign_memory(&self, ptr: *mut c_void,
                         access: AccessPermission) -> Result<(), ErrorType> {
        to_result(unsafe {
            hsa_memory_assign_agent(ptr, self.clone(), access)
        }, ())
    }

    fn get_info_raw<T>(&self, attr: AgentInfo) -> Result<T, ErrorType> {
        let mut x: T = unsafe { mem::zeroed() };
        let p: *mut c_void = &mut x as *mut _ as *mut c_void;
        unsafe {
            match hsa_agent_get_info(self.clone(), attr, p) {
                0 => Ok(x),
                e => Err(to_error_type(e))
            }
        }
    }

    fn get_info_str(&self, attribute: AgentInfo, max_size: usize) -> Result<String, ErrorType> {
        let mut buf: Vec<u8> = Vec::with_capacity(max_size);
        let p: *mut c_void = buf.as_mut_ptr() as *mut _ as *mut c_void;
        unsafe {
            match hsa_agent_get_info(self.clone(), attribute, p) {
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

extern "C" fn agent_list_callback(agent: Agent, data: *mut c_void) -> c_int {
    let v: *mut Vec<Agent> = data as *mut Vec<Agent>;
    unsafe {
        (*v).push(agent);
    }
    0
}
