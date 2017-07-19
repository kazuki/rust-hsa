use std::os::raw::c_void;
use super::{check, get_str, get_info, ErrorStatus, FromPrimitive};
use native::*;

fn _get_info(attr: SystemInfo, v: *mut c_void) -> HSAStatus {
    unsafe { hsa_system_get_info(attr, v) }
}

pub fn version_major() -> Result<u16, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::VersionMajor, x))
}

pub fn version_minor() -> Result<u16, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::VersionMinor, x))
}

pub fn timestamp() -> Result<u64, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::Timestamp, x))
}

pub fn timestamp_frequency() -> Result<u64, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::TimestampFrequency, x))
}

pub fn signal_max_wait() -> Result<u64, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::SignalMaxWait, x))
}

pub fn endianness() -> Result<Endianness, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::Endianness, x))
}

pub fn machine_model() -> Result<MachineModel, ErrorStatus> {
    get_info(|x| _get_info(SystemInfo::MachineModel, x))
}

pub fn extensions() -> Result<Vec<Extension>, ErrorStatus> {
    let ret: Result<[u8; 128], ErrorStatus> = get_info(|x| _get_info(SystemInfo::Extensions, x));
    ret.map(|x| {
        let mut v = Vec::new();
        for (i, b) in x.iter().enumerate().filter(|&(_, &b)| b != 0) {
            for j in (0..8).filter(|&j| (*b >> j) & 1 != 0) {
                if let Some(e) = Extension::from_u16((i * 8 + j) as u16) {
                    v.push(e)
                }
            }
        }
        v
    })
}

pub fn extension_names() -> Result<Vec<String>, ErrorStatus> {
    extensions().map(|e| {
        e.iter()
            .filter_map(|id| get_extension_name(*id).ok())
            .collect()
    })
}

pub fn get_extension_name(extension: Extension) -> Result<String, ErrorStatus> {
    get_str(hsa_extension_get_name, extension)
}

#[deprecated]
pub fn extension_supported(
    extension: Extension,
    version_major: u16,
    version_minor: u16,
) -> Result<bool, ErrorStatus> {
    let mut result = false;
    unsafe {
        check(
            hsa_system_extension_supported(extension, version_major, version_minor, &mut result),
            result,
        )
    }
}

pub fn major_extension_supported(
    extension: Extension,
    version_major: u16,
) -> Result<(u16, bool), ErrorStatus> {
    let mut result = false;
    let mut version_minor = 0u16;
    check(
        unsafe {
            hsa_system_major_extension_supported(
                extension,
                version_major,
                &mut version_minor,
                &mut result,
            )
        },
        (version_minor, result),
    )
}

pub fn get_finalizer1_extension_table() -> Result<ExtFinalizer1, ErrorStatus> {
    use std::mem::{size_of, zeroed};
    unsafe {
        let mut table: ExtFinalizer1 = zeroed();
        let p: *mut c_void = &mut table as *mut _ as *mut c_void;
        check(
            hsa_system_get_major_extension_table(
                Extension::Finalizer,
                1,
                size_of::<ExtFinalizer1>(),
                p,
            ),
            (),
        ).map(|_| table)
    }
}
