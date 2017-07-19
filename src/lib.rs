#![feature(core_intrinsics)]
#![allow(deprecated)]

#[macro_use]
extern crate enum_primitive;
use enum_primitive::FromPrimitive;

use std::mem::zeroed;
use std::os::raw::{c_char, c_void};

mod native;
mod system;
mod agent;
mod cache;
mod signal;
mod queue;
mod region;
mod isa;
mod wavefront;
mod executable;
mod code_object;
mod ext_finalize;

use native::HSAStatus;
pub use native::{Agent, Cache, CodeObjectType, DefaultFloatRoundingMode, DeviceType, Extension,
                 FenceScope, KernelDispatchPacket, KernelDispatchPacketSetup, MachineModel,
                 PacketType, PacketHeader, Profile, QueueType, RegionSegment, SignalCondition,
                 SignalValue, WaitState};
pub use signal::*;
pub use queue::*;
pub use region::*;
pub use executable::*;
pub use code_object::*;
pub use ext_finalize::*;
pub use system::*;
pub use std::collections::BTreeSet as Flags;

enum_from_primitive! {
#[repr(C)]
#[derive(Debug)]
pub enum SuccessStatus {
    Success = 0x0,
    InfoBreak,
}}

enum_from_primitive! {
#[repr(C)]
#[derive(Debug)]
pub enum ErrorStatus {
    // 2.2.1.1
    InvalidArgument = 0x1001,
    InvalidQueueCreation,
    InvalidAllocation,
    InvalidAgent,
    InvalidRegion,
    InvalidSignal,
    InvalidQueue,
    OutOfResources,
    InvalidPacketFormat,
    ResourceFree,
    NotInitialized,
    RefCountOverflow,
    IncompatibleArguments,
    InvalidIndex,
    InvalidISA,
    InvalidCodeObject,
    InvalidExecutable,
    FrozenExecutable,
    InvalidSymbolName,
    VariableAlreadyDefined,
    VariableUndefined,
    Exception,
    InvalidISAName,
    InvalidCodeSymbol,
    InvalidExecutableSymbol,
    InvalidFile,
    InvalidCodeObjectReader,
    InvalidCache,
    InvalidWavefront,
    InvalidSignalGroup,
    InvalidRuntimeState,

    // 3.2.1.1
    InvalidProgram = 0x2000,
    InvalidModule,
    IncompatibleModule,
    ModuleAlreadyIncluded,
    SymbolMismatch,
    FinalizationFailed,
    DirectiveMismatch,
    InvalidCodeObjectWriter,

    // 3.3.1.1
    ImageFormatUnsupported = 0x3000,
    ImageSizeUnsupported,
    ImagePitchUnsupported,
    SamplerDescriptorUnsupported,

    // 3.4.1.1
    InvalidSessionState = 0x4000,
    InvalidSamplingContext,
    CannotStopSession,

    // 3.5.6.1
    EventsNotInitialized = 0x5000,
    AlreadyInitialized,
    OutOfEvents,
    EventNotRegistered,
    CannotUseProducers,
}}

fn check<T>(ret: HSAStatus, ok: T) -> Result<T, ErrorStatus> {
    match ret {
        0 => Ok(ok),
        _ => {
            match ErrorStatus::from_i32(ret) {
                Some(e) => Err(e),
                _ => Err(ErrorStatus::Exception),
            }
        }
    }
}

/*
fn check_with_success_status(ret: HSAStatus) -> Result<SuccessStatus, ErrorStatus> {
    match SuccessStatus::from_i32(ret) {
        Some(s) => Ok(s),
        _ => match ErrorStatus::from_i32(ret) {
            Some(e) => Err(e),
            _ => Err(ErrorStatus::Exception),
        }
    }
}
*/

pub fn init() -> Result<(), ErrorStatus> {
    check(unsafe { native::hsa_init() }, ())
}

pub fn shutdown() -> Result<(), ErrorStatus> {
    check(unsafe { native::hsa_shut_down() }, ())
}

pub fn status_string(status: ErrorStatus) -> Result<String, ErrorStatus> {
    get_str(native::hsa_status_string, status as HSAStatus)
}

fn get_str<T>(
    f: unsafe extern "C" fn(T, &*const c_char) -> HSAStatus,
    v: T,
) -> Result<String, ErrorStatus> {
    unsafe {
        let p: *const c_char = std::ptr::null();
        check(
            f(v, &p),
            std::ffi::CStr::from_ptr(p).to_string_lossy().to_string(),
        )
    }
}

fn get_fixed_str<F>(f: F, sz: usize) -> Result<String, ErrorStatus>
where
    F: Fn(*mut c_void) -> HSAStatus,
{
    let mut t = String::with_capacity(sz);
    for _ in 0..sz {
        t.push('\0');
    }
    check(f(t.as_ptr() as *mut c_void), ()).map(|_| {
        let pos = t.find('\0').unwrap_or_else(|| t.len());
        t.truncate(pos);
        t
    })
}

fn get_info<F, R>(f: F) -> Result<R, ErrorStatus>
where
    F: Fn(*mut c_void) -> HSAStatus,
{
    let mut x: R = unsafe { zeroed() };
    let p: *mut c_void = &mut x as *mut _ as *mut c_void;
    check(f(p), x)
}

fn iter_callback_helper<T>(x: T, data: *mut c_void) -> HSAStatus {
    let v: *mut Vec<T> = data as *mut Vec<T>;
    unsafe {
        (*v).push(x);
    }
    0
}

fn bitflags<T>(x: u32) -> Flags<T>
where
    T: FromPrimitive + Ord,
{
    let mut flags = Flags::new();
    for i in 0..32 {
        if (x >> i) & 1 == 1 {
            match T::from_u32(1 << i) {
                Some(t) => flags.insert(t),
                _ => true,
            };
        }
    }
    flags
}
