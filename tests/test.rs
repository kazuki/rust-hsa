extern crate hsa;
extern crate libc;

use libc::{c_void, size_t};
use std::io::Read;

// FIXME align on 16-byte
struct VectorCopyArgs {
    pub input: *mut c_void,
    pub output: *mut c_void,
}

#[test]
fn test_vector_copy() {
    hsa::init().unwrap();

    let agent = _get_first_agent(hsa::DeviceType::GPU).unwrap();

    let queue_size = agent.queue_max_size().unwrap();
    let mut queue = hsa::Queue::create(&agent, queue_size, hsa::QueueType::Single).unwrap();

    let mut program = hsa::Program::new(hsa::MachineModel::Large,
                                        hsa::Profile::Full,
                                        hsa::DefaultFloatRoundingMode::Default, "").unwrap();

    let mut brig_buf: Vec<u8> = Vec::new();
    std::fs::File::open("tests/vector_copy.brig").unwrap()
        .read_to_end(&mut brig_buf).unwrap();
    program.add_module(&brig_buf).unwrap();

    let code_object = program.finalize(agent.isa().unwrap(), 0, "",
                                       hsa::CodeObjectType::Program).unwrap();

    let mut executable = hsa::Executable::new(hsa::Profile::Full,
                                              hsa::ExecutableState::Unfrozen,
                                              "").unwrap();
    executable.load_code_object(&agent, &code_object, "").unwrap();
    executable.freeze("").unwrap();
    let symbol = executable.get_symbol("", "&__vector_copy_kernel", &agent, 0).unwrap();
    let kernel_object = symbol.kernel_object().unwrap();
    let kernarg_segment_size = symbol.kernel_kernarg_segment_size().unwrap();
    let group_segment_size = symbol.kernel_group_segment_size().unwrap();
    let private_segment_size = symbol.kernel_private_segment_size().unwrap();

    let signal = hsa::Signal::new(1, 0, &vec![]).unwrap();

    let vec_size = 1024 * 1024;
    let mut in_vec: Vec<u32> = Vec::with_capacity(vec_size);
    let mut out_vec: Vec<u32> = Vec::with_capacity(vec_size);
    unsafe {
        in_vec.set_len(vec_size);
        out_vec.set_len(vec_size);
    }
    for i in 0..vec_size {
        in_vec[i] = i as u32;
        out_vec[i] = 0;
    }
    let in_ptr = in_vec.as_mut_ptr() as *mut c_void;
    let out_ptr = out_vec.as_mut_ptr() as *mut c_void;

    hsa::memory::register(in_ptr, (vec_size * std::mem::size_of::<u32>()) as size_t).unwrap();
    hsa::memory::register(out_ptr, (vec_size * std::mem::size_of::<u32>()) as size_t).unwrap();

    let args = VectorCopyArgs {
        input: in_ptr,
        output: out_ptr,
    };

    let kernarg_region = hsa::Region::get_first_region(&agent,
                                                       hsa::RegionSegment::Global,
                                                       &[hsa::RegionGlobalFlag::KernArg]).unwrap();
    let kernarg_address = kernarg_region.allocate(kernarg_segment_size as u64).unwrap();
    unsafe {
        std::ptr::copy(&args as *const _ as *const c_void,
                       kernarg_address,
                       std::mem::size_of::<VectorCopyArgs>());
    }

    let packet = hsa::KernelDispatchPacket::new(
        &hsa::PacketHeader {
            header_type: hsa::PacketType::KernelDispatch,
            barrier: false,
            acquire_fence_scope: hsa::FenceScope::System,
            release_fence_scope: hsa::FenceScope::System,
        }, &hsa::KernelDispatchPacketSetup {
            dimensions: 1,
        }, &[256, 1, 1], &[1024 * 1024, 1, 1],
        private_segment_size, group_segment_size,
        kernel_object, kernarg_address, signal.clone_handle());
    queue.enqueue(&packet);

    signal.wait_acquire(hsa::SignalCondition::LT, 1, std::u64::MAX, hsa::WaitState::Blocked);

    let mut valid = true;
    for i in 0..in_vec.len() {
        if in_vec[i] != out_vec[i] || in_vec[i] != i as u32 {
            valid = false;
            break;
        }
    }

    hsa::memory::free(kernarg_address).unwrap();
    hsa::shutdown().unwrap();

    assert!(valid);
}

fn _get_first_agent(device_type: hsa::DeviceType) -> Result<hsa::Agent, hsa::ErrorType> {
    match hsa::Agent::list() {
        Ok(mut agents) => {
            for i in 0..agents.len() {
                match agents[i].device() {
                    Ok(dtype) => {
                        if dtype == device_type {
                            return Ok(agents.remove(i))
                        }
                    },
                    Err(e) => return Err(e)
                }
            }
        },
        Err(e) => return Err(e)
    }
    Err(hsa::ErrorType::InvalidAgent)
}
