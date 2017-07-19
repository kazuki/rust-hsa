extern crate hsa;

fn main() {
    hsa::init().unwrap();

    let ts: u64 = hsa::timestamp().unwrap();
    let ts_freq: u64 = hsa::timestamp_frequency().unwrap();
    println!(
        "[HSA SystemInfo]\n\
              * version: {}.{}\n\
              * timestamp: {} [s] (freq: {} [MHz])\n\
              * signal-max-wait: {}\n\
              * endianness: {:?}\n\
              * machine_model: {:?}\n\
              * extensions: {:?}\n",
        hsa::version_major().unwrap(),
        hsa::version_minor().unwrap(),
        ts as f64 / ts_freq as f64,
        ts_freq as f64 / 1000000.0,
        hsa::signal_max_wait().unwrap(),
        hsa::endianness().unwrap(),
        hsa::machine_model().unwrap(),
        hsa::extension_names().unwrap()
    );

    let agents = hsa::Agent::list().unwrap();
    assert!(!agents.is_empty());
    for agent in agents {
        println!(r#"[Agent ({:?})]
  * name: {}
  * vendor: {}
  * feature: {:?}
  * machine_model/profile: {:?} / {:?}
  * float_rounding: {:?} / {:?}
  * fast_f16_operation: {}
  * wavefront size: {}
  * workgroup max dim: {:?}
  * workgroup max size: {}
  * grid max dim: {:?}
  * grid max size: {}
  * fbarrier max size: {}
  * queues max: {}
  * queue min/max/type: {} / {} / {:?}
  * version: {}.{}"#,
                 agent.device().unwrap(),
                 agent.name().unwrap(),
                 agent.vendor().unwrap(),
                 agent.feature().unwrap(),
                 agent.machine_model().unwrap(),
                 agent.profile().unwrap(),
                 agent.default_float_rounding_mode().unwrap(),
                 agent.base_profile_default_float_rounding_mode().unwrap(),
                 agent.fast_f16_operation().unwrap(),
                 agent.wavefront_size().unwrap(),
                 agent.workgroup_max_dim().unwrap(),
                 agent.workgroup_max_size().unwrap(),
                 agent.grid_max_dim().unwrap(),
                 agent.grid_max_size().unwrap(),
                 agent.fbarrier_max_size().unwrap(),
                 agent.queues_max().unwrap(),
                 agent.queue_min_size().unwrap(),
                 agent.queue_max_size().unwrap(),
                 agent.queue_type().unwrap(),
                 agent.version_major().unwrap(), agent.version_minor().unwrap(),
                 );
        for isa in agent.isas().unwrap_or_default() {
            println!(
                r#"  [ISA]
    * name: {}
    * machine_models/profiles: {:?} / {:?}
    * float rounding modes: {:?} / {:?}
    * fast f16 ops: {}
    * workgroup max dim: {:?}
    * workgroup max size: {}
    * grid max dim: {:?}
    * grid max size: {}
    * fbarrier max size: {}"#,
                isa.name().unwrap(),
                isa.machine_models().unwrap(),
                isa.profiles().unwrap(),
                isa.default_float_rounding_modes().unwrap(),
                isa.base_profile_default_float_rounding_modes().unwrap(),
                isa.fast_f16_operation().unwrap(),
                isa.workgroup_max_dim().unwrap(),
                isa.workgroup_max_size().unwrap(),
                isa.grid_max_dim().unwrap(),
                isa.grid_max_size().unwrap(),
                isa.fbarrier_max_size().unwrap()
            );
            for wf in isa.wavefronts().unwrap() {
                println!("    [Wavefront]");
                println!("      * size: {}", wf.size().unwrap());
            }
        }
        for region in agent.regions().unwrap() {
            println!("  [Region ({:?})]\n    * size: {}",
                     region.segment().unwrap(),
                     region.size().unwrap(),
            );
            if region.segment().unwrap() == hsa::RegionSegment::Global {
                println!("    * global flags: {:?}", region.global_flags().unwrap());
            }
            if region.runtime_alloc_allowed().unwrap() {
                println!(
                    "    * alloc max_size/granule/align: {} {} {}",
                    region.alloc_max_size().unwrap(),
                    region.runtime_alloc_granule().unwrap(),
                    region.runtime_alloc_alignment().unwrap()
                );
            }
        }
        println!("");
    }
    hsa::shutdown().unwrap();
}
