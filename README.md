# rust-hsa
HSA (Heterogeneous System Architecture) bindings for Rust

## Requirements

* AMD Kaveri APUs
* [HSA-Drivers-Linux-AMD](https://github.com/HSAFoundation/HSA-Drivers-Linux-AMD)
* [HSA-Runtime-AMD](https://github.com/HSAFoundation/HSA-Runtime-AMD)

## Tested Environment

* CPU: AMD A10-7850K (Kaveri)
* M/B: [MSI A88XI AC](http://www.msi.com/product/mb/A88XI_AC.html) (BIOS: v1.6)
* Linux Distribution: Gentoo Linux (amd64)
* HSA Driver: amdkfd v1.4 (linux 4.0.0+)

## Test outputs

```
$ cargo test -- --nocapture
   Compiling hsa v0.1.0 (file:///home/kazuki/projects/rust-hsa)
     Running target/debug/hsa-53dc2c312dc2da42

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured

     Running target/debug/test-406bceff3464cc09

running 2 tests
HSA SystemInfo
* version: 1.0
* timestamp: 2141.220068967 [s] (freq: 1000 [MHz])
* signal-max-wait: 18446744073709551615
* endianness: Little
* machine_model: Large

AgentInfo
* name: Kaveri CPU
* vendor: AMD
* feature: AgentDispatch
* machine_model: Large
* profile: Full
* float_rouding: Near
* base_profile_float_rouding: Near
* fast_f16_operation: false
* wavefront size: 0
* workgroup max dim: [0, 0, 0]
* workgroup max size: 0
* grid max dim: Dim3 { x: 0, y: 0, z: 0 }
* grid max size: 0
* fbarrier max size: 0
* queues max: 0
* queue max size: 0
* queue min size: 0
* queue type: Multi
* node: 0
* device: CPU
* isa: ISA { handle: 0 }
* version: 1.0

| RegionInfo
|  * segment: Global
|  * global flags: {KernArg, FineGrained}
|  * size: 140737488355328
|  * alloc max size: 15642705920
|  * runtime alloc allowed: true
|  * runtime alloc granule: 4096
|  * runtime alloc alignment: 4096

AgentInfo
* name: Spectre
* vendor: AMD
* feature: KernelDispatch
* machine_model: Large
* profile: Full
* float_rouding: Near
* base_profile_float_rouding: Near
* fast_f16_operation: false
* wavefront size: 64
* workgroup max dim: [2048, 2048, 2048]
* workgroup max size: 2048
* grid max dim: Dim3 { x: 4294967295, y: 4294967295, z: 4294967295 }
* grid max size: 4294967295
* fbarrier max size: 32
* queues max: 128
* queue max size: 131072
* queue min size: 4096
* queue type: Multi
* node: 0
* device: GPU
* isa: ISA { handle: 139926947498336 }
* version: 1.0

| RegionInfo
|  * segment: Group
|  * global flags: {}
|  * size: 65536
|  * alloc max size: 0
|  * runtime alloc allowed: false
|  * runtime alloc granule: 0
|  * runtime alloc alignment: 0

| RegionInfo
|  * segment: Global
|  * global flags: {CoarseGrained}
|  * size: 1073741824
|  * alloc max size: 268435456
|  * runtime alloc allowed: true
|  * runtime alloc granule: 4096
|  * runtime alloc alignment: 4096

| RegionInfo
|  * segment: Global
|  * global flags: {FineGrained, KernArg}
|  * size: 140737488355328
|  * alloc max size: 15642705920
|  * runtime alloc allowed: true
|  * runtime alloc granule: 4096
|  * runtime alloc alignment: 4096

test test_info ... ok
test test_vector_copy ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests hsa

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured
```
