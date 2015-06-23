#![feature(core_intrinsics)]

#[macro_use]
extern crate enum_primitive;
extern crate num;
use num::FromPrimitive;

extern crate libc;
use libc::c_int;

mod native;
mod system;
mod agent;
mod signal;
mod queue;
pub mod memory;
mod code;
mod ext;

pub fn init() -> Result<(), ErrorType> {
    to_result(unsafe {
        native::hsa_init()
    }, ())
}
pub fn shutdown() -> Result<(), ErrorType> {
    to_result(unsafe {
        native::hsa_shut_down()
    }, ())
}

pub use native::*;
pub use system::{
    major_version,
    minor_version,
    timestamp,
    timestamp_frequency,
    signal_max_wait,
    endianness,
    machine_model,
};
pub use signal::Signal;
pub use queue::Queue;
pub use code::Executable;
pub use ext::Program;

enum_from_primitive! {
#[derive(PartialEq, Debug)]
#[repr(C)]
pub enum ErrorType {
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
    InvalidProgram = 0x2000,
    InvalidModule,
    IncompatibleModule,
    ModuleAlreadyIncluded,
    SymbolMismatch,
    FinalizationFailed,
    DirectiveMismatch,
}
}

fn to_error_type(i: c_int) -> ErrorType {
    match ErrorType::from_i32(i) {
        Some(x) => x,
        _ => ErrorType::Exception,
    }
}
fn to_result<T>(i: c_int, v: T) -> Result<T, ErrorType> {
    match i {
        0x0 => Ok(v), // SUCCESS
        _ => Err(to_error_type(i)),
    }
}
