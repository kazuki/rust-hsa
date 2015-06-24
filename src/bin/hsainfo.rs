extern crate hsa;

fn main() {
    hsa::init().unwrap();

    let ts: u64 = hsa::timestamp().unwrap();
    let ts_freq: u64 = hsa::timestamp_frequency().unwrap();
    println!("HSA SystemInfo\n\
              * version: {}.{}\n\
              * timestamp: {} [s] (freq: {} [MHz])\n\
              * signal-max-wait: {}\n\
              * endianness: {:?}\n\
              * machine_model: {:?}\n",
             hsa::major_version().unwrap(),
             hsa::minor_version().unwrap(),
             ts as f64 / ts_freq as f64,
             ts_freq as f64 / 1000000.0,
             hsa::signal_max_wait().unwrap(),
             hsa::endianness().unwrap(),
             hsa::machine_model().unwrap());

    let agents = hsa::Agent::list().unwrap();
    assert!(agents.len() > 0);
    for agent in agents.iter() {
        println!("AgentInfo\n\
                  * name: {}\n\
                  * vendor: {}\n\
                  * feature: {:?}\n\
                  * machine_model: {:?}\n\
                  * profile: {:?}\n\
                  * float_rouding: {:?}\n\
                  * base_profile_float_rouding: {:?}\n\
                  * fast_f16_operation: {}\n\
                  * wavefront size: {}\n\
                  * workgroup max dim: {:?}\n\
                  * workgroup max size: {}\n\
                  * grid max dim: {:?}\n\
                  * grid max size: {}\n\
                  * fbarrier max size: {}\n\
                  * queues max: {}\n\
                  * queue max size: {}\n\
                  * queue min size: {}\n\
                  * queue type: {:?}\n\
                  * node: {}\n\
                  * device: {:?}\n\
                  * isa: {:?}\n\
                  * version: {}.{}\n",
                 agent.name().unwrap(),
                 agent.vendor().unwrap(),
                 agent.feature().unwrap(),
                 agent.machine_model().unwrap(),
                 agent.profile().unwrap(),
                 agent.default_float_rouding_mode().unwrap(),
                 agent.base_profile_default_float_rouding_mode().unwrap(),
                 agent.fast_f16_operation().unwrap(),
                 agent.wavefront_size().unwrap(),
                 agent.workgroup_max_dim().unwrap(),
                 agent.workgroup_max_size().unwrap(),
                 agent.grid_max_dim().unwrap(),
                 agent.grid_max_size().unwrap(),
                 agent.fbarrier_max_size().unwrap(),
                 agent.queues_max().unwrap(),
                 agent.queue_max_size().unwrap(),
                 agent.queue_min_size().unwrap(),
                 agent.queue_type().unwrap(),
                 agent.node().unwrap(),
                 agent.device().unwrap(),
                 agent.isa().unwrap(),
                 agent.major_version().unwrap(), agent.minor_version().unwrap(),
                 );
        let regions = hsa::Region::list(&agent).unwrap();
        for region in regions.iter() {
            println!("| RegionInfo\n\
                      |  * segment: {:?}\n\
                      |  * global flags: {:?}\n\
                      |  * size: {}\n\
                      |  * alloc max size: {}\n\
                      |  * runtime alloc allowed: {}\n\
                      |  * runtime alloc granule: {} \n\
                      |  * runtime alloc alignment: {}\n",
                     region.segment().unwrap(),
                     region.global_flags().unwrap(),
                     region.size().unwrap(),
                     region.alloc_max_size().unwrap(),
                     region.runtime_alloc_allowed().unwrap(),
                     region.runtime_alloc_granule().unwrap(),
                     region.runtime_alloc_alignment().unwrap()
                     );
        }
    }

    hsa::shutdown().unwrap();
}
