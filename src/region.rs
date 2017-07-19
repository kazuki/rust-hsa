use std::u32;
use std::os::raw::c_void;
use std::ptr::{null, null_mut};

use native::*;
use super::{bitflags, check, get_info, ErrorStatus, Flags};

impl Region {
    pub fn segment(&self) -> Result<RegionSegment, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::Segment, x))
    }

    pub fn global_flags(&self) -> Result<Flags<RegionGlobalFlag>, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::GlobalFlags, x))
            .map(|flags: RegionGlobalFlag| bitflags(flags as u32))
    }

    pub fn size(&self) -> Result<usize, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::Size, x))
    }

    pub fn alloc_max_size(&self) -> Result<usize, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::AllocMaxSize, x))
    }

    pub fn alloc_max_private_workgroup_size(&self) -> Result<u32, ErrorStatus> {
        get_info(|x| {
            self.get_info(RegionInfo::AllocMaxPrivateWorkgroupSize, x)
        })
    }

    pub fn runtime_alloc_allowed(&self) -> Result<bool, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::RuntimeAllocAllowed, x))
    }

    pub fn runtime_alloc_granule(&self) -> Result<usize, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::RuntimeAllocGranule, x))
    }

    pub fn runtime_alloc_alignment(&self) -> Result<usize, ErrorStatus> {
        get_info(|x| self.get_info(RegionInfo::RuntimeAllocAlignment, x))
    }

    fn get_info(&self, attr: RegionInfo, v: *mut c_void) -> HSAStatus {
        unsafe { hsa_region_get_info(*self, attr, v) }
    }
}

pub enum Memory<T> {
    RegionMemory(*mut T),
    Registered(*mut T, usize),
    None,
}

impl Memory<u8> {
    pub fn allocate(region: Region, size: usize) -> Result<Memory<u8>, ErrorStatus> {
        let ptr: *mut c_void = null_mut();
        unsafe {
            check(hsa_memory_allocate(region, size, &ptr), ()).map(
                |_| {
                    Memory::RegionMemory(ptr as *mut u8)
                },
            )
        }
    }

    pub fn register(ptr: *mut u8, size: usize) -> Result<Memory<u8>, ErrorStatus> {
        unsafe {
            check(hsa_memory_register(ptr as *mut c_void, size), ())
                .map(|_| Memory::Registered(ptr, size))
        }
    }
}

impl<T> Memory<T> {
    pub fn new(region: Region) -> Result<Memory<T>, ErrorStatus> {
        use std::mem::size_of;
        let ptr: *mut c_void = null_mut();
        let size = size_of::<T>();
        unsafe {
            check(hsa_memory_allocate(region, size, &ptr), ()).map(
                |_| {
                    Memory::RegionMemory(ptr as *mut T)
                },
            )
        }
    }

    pub fn as_ptr(&self) -> *const T {
        match *self {
            Memory::RegionMemory(x) |
            Memory::Registered(x, _) => x,
            Memory::None => null(),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut T {
        match *self {
            Memory::RegionMemory(x) |
            Memory::Registered(x, _) => x,
            Memory::None => null_mut(),
        }
    }

    pub fn assign_agent(&self, agent: Agent, access: AccessPermission) -> Result<(), ErrorStatus> {
        let ptr = self.as_mut_ptr();
        if ptr.is_null() {
            return Err(ErrorStatus::InvalidArgument);
        }
        check(unsafe {
            hsa_memory_assign_agent(ptr as *mut c_void, agent, access)
        }, ())
    }

    pub fn copy_from(&mut self, src: &T) {
        unsafe {
            use std::ptr::copy_nonoverlapping;
            let ptr = src as *const _ as *const T;
            copy_nonoverlapping(ptr, self.as_mut_ptr(), 1);
        }
    }
}

impl<T> Drop for Memory<T> {
    fn drop(&mut self) {
        match *self {
            Memory::RegionMemory(x) => unsafe {
                hsa_memory_free(x as *mut c_void);
            },
            Memory::Registered(x, sz) => unsafe {
                hsa_memory_deregister(x as *mut c_void, sz);
            },
            _ => (),
        }
    }
}

pub unsafe fn copy<T>(src: *const T, dst: *mut T, bytes: usize) -> Result<(), ErrorStatus> {
    check(hsa_memory_copy(dst as *mut c_void, src as *const c_void, bytes), ())
}
