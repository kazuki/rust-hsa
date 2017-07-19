extern crate hsa;

use std::u64;
use std::os::raw::c_void;

use hsa::SignalBase;

fn check<T>(r: Result<T, hsa::ErrorStatus>, msg: &str) -> T {
    match r {
        Ok(x) => {
            println!("{} succeeded.", msg);
            x
        }
        Err(e) => panic!("{} failed. reason={:?}", msg, e),
    }
}

fn main() {
    let test_size = 1024 * 1024 * 4;
    hsa::init().unwrap();
    check(
        hsa::major_extension_supported(hsa::Extension::Finalizer, 1),
        "Checking finalizer 1.0 extension support",
    );
    let agent = {
        let mut gpu_agents = check(
            hsa::Agent::from_device_type(hsa::DeviceType::GPU),
            "Getting a gpu agent",
        );
        gpu_agents.pop().expect("GPU agent not found")
    };
    println!(
        "The agent name is {}.",
        check(agent.name(), "Querying the agent name")
    );
    let isa = *check(agent.isas(), "Getting a ISA").get(0).expect(
        "ISA not found",
    );
    let (profile, brig) = {
        if isa.profiles().unwrap().contains(&hsa::Profile::Full) {
            (hsa::Profile::Full, include_bytes!("vector_copy_full.brig"))
        } else {
            (hsa::Profile::Base, include_bytes!("vector_copy_base.brig"))
        }
    };
    let machine_model = *check(isa.machine_models(), "Obtaining machine model")
        .iter()
        .next()
        .unwrap();
    let queue_size = check(
        agent.queue_max_size(),
        "Querying the isa maximum queue size",
    );
    println!("The maximum queue size is {}.", queue_size);
    let queue = check(
        hsa::Queue::new(agent, queue_size, hsa::QueueType::Single),
        "Creating the queue",
    );
    let code_object = {
        let ext_prog = check(
            hsa::ExtProgram::new(
                machine_model,
                profile,
                hsa::DefaultFloatRoundingMode::Default,
                None,
            ),
            "Create the program",
        );
        check(
            ext_prog.add_module(brig),
            "Adding the brig module to the program",
        );
        check(
            ext_prog.finalize(isa, "", hsa::CodeObjectType::Program),
            "Finalizing the program",
        )
    };
    let executable = check(
        hsa::Executable::new(profile, hsa::DefaultFloatRoundingMode::Default, ""),
        "Create the executable",
    );
    check(
        executable.load_code_object(agent, code_object, ""),
        "Loading the code object",
    );
    check(executable.freeze(""), "Freeze the executable");
    let symbol = check(
        executable.get_symbol("", "&__vector_copy_kernel", agent, 0),
        "Extract the symbol from the executable",
    );
    let kernel_object = check(
        symbol.kernel_object(),
        "Extracting the kernel object from the executable",
    );
    let group_segment_size = check(
        symbol.kernel_group_segment_size(),
        "Extracting the group segment size from the executable",
    );
    let private_segment_size = check(
        symbol.kernel_private_segment_size(),
        "Extracting the private segment size from the executable",
    );
    let signal = check(hsa::Signal::new(1, &[]), "Creating a HSA signal");
    let fine_grained_region = check(
        agent.fine_grained_global_regions(),
        "Finding a fine grained memory region",
    ).pop()
        .expect("Not found fine grained memory region");
    let kernarg_region = check(
        agent.kernarg_global_regions(),
        "Finding a kernarg memory region",
    ).pop()
        .expect("Not found kernarg memory region");
    let in_mem = check(
        hsa::Memory::allocate(fine_grained_region, test_size),
        "Allocating argument memory for input parameter",
    );
    let out_mem = check(
        hsa::Memory::allocate(fine_grained_region, test_size),
        "Allocating argument memory for output parameter",
    );
    unsafe {
        let pi = in_mem.as_mut_ptr();
        let po = out_mem.as_mut_ptr();
        for i in 0..test_size as isize {
            *pi.offset(i) = i as u8;
            *po.offset(i) = 0;
        }
    }

    #[repr(C)]
    struct Args {
        in_ptr: *const c_void,
        out_ptr: *mut c_void,
    };
    let args = {
        let mut m = check(
            hsa::Memory::<Args>::new(kernarg_region),
            "Allocating kernel argument memory buffer",
        );
        let tmp = Args {
            in_ptr: in_mem.as_ptr() as *const c_void,
            out_ptr: out_mem.as_mut_ptr() as *mut c_void,
        };
        m.copy_from(&tmp);
        m
    };
    let packet = hsa::KernelDispatchPacket::new(
        hsa::PacketType::KernelDispatch,
        &[
            (
                hsa::FenceScope::System,
                hsa::PacketHeader::ScacquireFenceScope,
            ),
            (
                hsa::FenceScope::System,
                hsa::PacketHeader::ScreleaseFenceScope,
            ),
        ],
        1,
        &[256],
        &[1024 * 1024],
        private_segment_size,
        group_segment_size,
        kernel_object,
        args.as_ptr(),
        signal.handle(),
    ).unwrap();
    let index: u64 = queue.load_write_index_relaxed();
    queue.copy_kernel_dispatch_packet(&packet, index);
    queue.add_write_index_relaxed(1);
    queue.doorbell_signal().store_screlease(index as i64);
    println!("Dispatching the kernel");
    signal.wait_scacquire(
        hsa::SignalCondition::Eq,
        0,
        u64::MAX,
        hsa::WaitState::Blocked,
    );
    let (valid, fail_index) = unsafe {
        let x = in_mem.as_ptr();
        let y = out_mem.as_ptr();
        let mut valid = true;
        let mut fail_index = 0;
        for i in 0..1024 * 1024 * 4 {
            if *x.offset(i) != *y.offset(i) {
                valid = false;
                fail_index = i;
                break;
            }
        }
        (valid, fail_index)
    };
    if valid {
        println!("Passed validation.");
    } else {
        println!("VALIDATION FAILED!");
        println!("Bad index: {}", fail_index);
    }
    hsa::shutdown().unwrap();
}
