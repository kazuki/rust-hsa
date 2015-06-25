extern crate hsa;
extern crate libc;
extern crate time;

use std::io::Read;
use libc::{c_void, size_t};

struct VectorCopyArgs {
    pub input: *mut c_void,
    pub output: *mut c_void,
}

fn main() {
    hsa::init().unwrap();

    let agent = hsa::Agent::from_device_type(hsa::DeviceType::GPU).unwrap();
    let mut queue = hsa::Queue::create(&agent,
                                       agent.queue_max_size().unwrap(),
                                       hsa::QueueType::Single).unwrap();

    let mut brig_buf: Vec<u8> = Vec::new();
    std::fs::File::open("brigs/vector_copy.brig").unwrap()
        .read_to_end(&mut brig_buf).unwrap();

    let code_object = agent.create_and_finalize_program(&vec![brig_buf], "", "").unwrap();
    let executable = hsa::Executable::new_and_freeze(&agent, &vec![code_object], "", "", "").unwrap();
    let symbol = executable.get_symbol("", "&__vector_copy_kernel", &agent, 0).unwrap();

    memcpy(&agent, &mut queue, &symbol, 1024 * 1024 * 4);
    memcpy(&agent, &mut queue, &symbol, 1024 * 1024 * 64);
    memcpy(&agent, &mut queue, &symbol, 1024 * 1024 * 256);
    memcpy(&agent, &mut queue, &symbol, 1024 * 1024 * 1024);
    memcpy(&agent, &mut queue, &symbol, 1024 * 1024 * 1024 * 4);

    hsa::shutdown().unwrap();
}

fn memcpy(agent: &hsa::Agent, queue: &mut hsa::Queue, symbol: &hsa::ExecutableSymbol, memsize: usize)
{
    let kernel_object = symbol.kernel_object().unwrap();
    let kernarg_segment_size = symbol.kernel_kernarg_segment_size().unwrap();
    let group_segment_size = symbol.kernel_group_segment_size().unwrap();
    let private_segment_size = symbol.kernel_private_segment_size().unwrap();

    let vec_size = memsize / std::mem::size_of::<u32>();
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

    hsa::memory::register(in_ptr, memsize as size_t).unwrap();
    hsa::memory::register(out_ptr, memsize as size_t).unwrap();

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
    let mut signal = hsa::Signal::new(1, &vec![]).unwrap();

    let packet = hsa::KernelDispatchPacket::new(
        &hsa::PacketHeader {
            header_type: hsa::PacketType::KernelDispatch,
            barrier: false,
            acquire_fence_scope: hsa::FenceScope::System,
            release_fence_scope: hsa::FenceScope::System,
        }, &hsa::KernelDispatchPacketSetup {
            dimensions: 1,
        }, &[256, 1, 1], &[vec_size as u32, 1, 1],
        private_segment_size, group_segment_size,
        kernel_object, kernarg_address, signal.clone_handle());
    let mut total_time: f64 = 0.0;
    let tries = 5;
    for _ in 0..tries {
        let start_time = time::precise_time_s();
        queue.enqueue(&packet);
        signal.wait_acquire(hsa::SignalCondition::LT, 1, std::u64::MAX, hsa::WaitState::Blocked);
        total_time += time::precise_time_s() - start_time;
        signal.store_release(1);
    }
    println!("memcpy {} MB => {} MB/s", memsize / 1024 / 1024,
             (memsize as f64) / 1024.0 / 1024.0 / (total_time / tries as f64));
}
