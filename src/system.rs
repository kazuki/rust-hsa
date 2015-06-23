use std::mem;
use libc::c_void;
use super::{to_result, ErrorType};
use native::{hsa_system_get_info,
             Endianness, MachineModel, SystemInfo};

pub fn major_version() -> Result<u16, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::VersionMajor);
}
pub fn minor_version() -> Result<u16, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::VersionMinor);
}
pub fn timestamp() -> Result<u64, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::Timestamp);
}
pub fn timestamp_frequency() -> Result<u64, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::TimestampFrequency);
}
pub fn signal_max_wait() -> Result<u64, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::SignalMaxWait);
}
pub fn endianness() -> Result<Endianness, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::Endianness);
}
pub fn machine_model() -> Result<MachineModel, ErrorType> {
    return hsa_system_get_info_raw(SystemInfo::MachineModel);
}

fn hsa_system_get_info_raw<T>(attribute: SystemInfo) -> Result<T, ErrorType> {
    let mut x: T = unsafe { mem::zeroed() };
    let p: *mut c_void = &mut x as *mut _ as *mut c_void;
    to_result(unsafe {
        hsa_system_get_info(attribute, p)
    }, x)
}
