use std::mem;
use std::collections::HashSet;
use libc::{c_int, c_void, size_t};
use super::{ErrorType, to_error_type, to_result};
use native::*;

impl Region {
    pub fn list(agent: &Agent) -> Result<Vec<Region>, ErrorType> {
        let mut v: Vec<Region> = Vec::new();
        let p: *mut c_void = &mut v as *mut _ as *mut c_void;
        to_result(unsafe {
            hsa_agent_iterate_regions(agent.clone(),
                                      region_list_callback,
                                      p)
        }, v)
    }

    pub fn get_first_region(agent: &Agent,
                            segment: RegionSegment,
                            flags: &[RegionGlobalFlag]) -> Result<Region, ErrorType> {
        match Region::list(agent) {
            Ok(mut regions) => {
                let set: HashSet<_> = flags.iter().cloned().collect();
                for i in 0..regions.len() {
                    match regions[i].segment() {
                        Ok(seg) => {
                            if seg != segment {
                                continue
                            }
                        },
                        Err(_) => continue
                    }
                    match regions[i].global_flags() {
                        Ok(region_flags) => {
                            if region_flags.intersection(&set).count() == set.len() {
                                return Ok(regions.remove(i))
                            }
                        },
                        Err(_) => {}
                    }
                }
            },
            Err(e) => return Err(e),
        }
        Err(ErrorType::Exception)
    }

    pub fn segment(&self) -> Result<RegionSegment, ErrorType> {
        self.get_info_raw(RegionInfo::Segment)
    }

    pub fn global_flags(&self) -> Result<HashSet<RegionGlobalFlag>, ErrorType> {
        match self.get_info_raw::<i32>(RegionInfo::GlobalFlags) {
            Ok(flags) => {
                let mut set = HashSet::new();
                if (flags & 1) == 1 {
                    set.insert(RegionGlobalFlag::KernArg);
                }
                if (flags & 2) == 2 {
                    set.insert(RegionGlobalFlag::FineGrained);
                }
                if (flags & 4) == 4 {
                    set.insert(RegionGlobalFlag::CoarseGrained);
                }
                Ok(set)
            },
            Err(e) => Err(e)
        }
    }

    pub fn size(&self) -> Result<size_t, ErrorType> {
        self.get_info_raw(RegionInfo::Size)
    }

    pub fn alloc_max_size(&self) -> Result<size_t, ErrorType> {
        self.get_info_raw(RegionInfo::AllocMaxSize)
    }

    pub fn runtime_alloc_allowed(&self) -> Result<bool, ErrorType> {
        self.get_info_raw(RegionInfo::RuntimeAllocAllowed)
    }

    pub fn runtime_alloc_granule(&self) -> Result<size_t, ErrorType> {
        self.get_info_raw(RegionInfo::RuntimeAllocGranule)
    }

    pub fn runtime_alloc_alignment(&self) -> Result<size_t, ErrorType> {
        self.get_info_raw(RegionInfo::RuntimeAllocAlignment)
    }

    pub fn allocate(&self, size: size_t) -> Result<*mut c_void, ErrorType> {
        let p: *mut c_void = unsafe { mem::zeroed() };
        to_result(unsafe {
            hsa_memory_allocate(self.clone(), size, &p)
        }, p)
    }

    fn get_info_raw<T>(&self, attr: RegionInfo) -> Result<T, ErrorType> {
        let mut x: T = unsafe { mem::zeroed() };
        let p: *mut c_void = &mut x as *mut _ as *mut c_void;
        unsafe {
            match hsa_region_get_info(self.clone(), attr, p) {
                0 => Ok(x),
                e => Err(to_error_type(e))
            }
        }
    }
}

pub fn free(ptr: *mut c_void) -> Result<(), ErrorType> {
    to_result(unsafe {
        hsa_memory_free(ptr)
    }, ())
}

pub fn copy(dst: *mut c_void, src: *const c_void, size: size_t) -> Result<(), ErrorType> {
    to_result(unsafe {
        hsa_memory_copy(dst, src, size)
    }, ())
}

pub fn register(ptr: *mut c_void, size: size_t) -> Result<(), ErrorType> {
    to_result(unsafe {
        hsa_memory_register(ptr, size)
    }, ())
}

pub fn deregister(ptr: *mut c_void, size: size_t) -> Result<(), ErrorType> {
    to_result(unsafe {
        hsa_memory_deregister(ptr, size)
    }, ())
}

extern "C" fn region_list_callback(region: Region, data: *mut c_void) -> c_int {
    let v: *mut Vec<Region> = data as *mut Vec<Region>;
    unsafe {
        (*v).push(region);
    }
    0
}
